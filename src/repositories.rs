use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpexted Error: [{0}]")]
    Unexpexted(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::FromRow)]
pub struct Task {
    id: i32,
    text: String,
    completed: bool,
}

impl Task {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,   // id: id, と書かなくて良いらしい
            text, // text: text, と書かなくて良いらしい
            completed: false,
        }
    }
}

#[async_trait]
pub trait TaskRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTaskPayload) -> anyhow::Result<Task>;
    async fn find(&self, id: i32) -> anyhow::Result<Task>; // findされないかも
    async fn all(&self) -> anyhow::Result<Vec<Task>>; // array
    async fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task>;
    async fn delete(&self, id: i32) -> anyhow::Result<(), RepositoryError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTaskPayload {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTaskPayload {
    text: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskRepositoryForDb {
    pool: PgPool,
}

impl TaskRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TaskRepositoryForDb { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryForDb {
    async fn create(&self, payload: CreateTaskPayload) -> anyhow::Result<Task> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (text, completed) values ($1, false) returning *
            "#,
        )
        .bind(payload.text)
        .fetch_one(&self.pool)
        .await
        .expect("Failed to create task");

        Ok(task)
    }
    async fn find(&self, id: i32) -> anyhow::Result<Task> {
        // FIXME: なぜかbindか効かないのでクエリに直書きしている。とりあえずDBからデータ取れることだけ確認。
        let task = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks where id = $1 limit 1", // "SELECT * FROM tasks where id = ? limit 1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .expect("Specified Task not found");

        Ok(task)
    }
    async fn all(&self) -> anyhow::Result<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .expect("Failed to find tasks");

        Ok(tasks)
    }
    async fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            update tasks set text = $1, completed = $2 where id = $3 returning *;
            "#,
        )
        .bind(payload.text.unwrap())
        .bind(payload.completed.unwrap())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .expect("Failed to update task");

        Ok(task)
    }
    async fn delete(&self, id: i32) -> anyhow::Result<(), RepositoryError> {
        sqlx::query(
            r#"
            delete from tasks where id = $1 ;
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpexted(e.to_string()),
        })
        .ok();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn scenario() {
        dotenv().ok();

        // connect DB
        let database_url = &env::var("TEST_DATABASE_URL").expect("undefined: [TEST_DATABASE_URL]");
        tracing::debug!("Connecting database...");
        let pool = PgPool::connect(database_url).await.expect(&format!(
            "Fail to connect database. url: [{}]",
            database_url
        ));

        // refresh database
        sqlx::query("Delete from tasks")
            .execute(&pool)
            .await
            .expect("[Refresh database] failed");
        // transactionを貼ったりしていないので並列でテストが走っていると危険な感じがするので、テスト側をベタ書きidに依存しない形に変更
        // CI毎にtest DBのidが膨れ上がっていくのでこれはこれで気になる
        // sqlx::query("ALTER SEQUENCE tasks_id_seq RESTART WITH 1").execute(&pool).await.expect("[Refresh database] failed");

        // initialize repository
        let repository = TaskRepositoryForDb::new(pool);
        let text = "test_task";

        // create
        let create_payload: CreateTaskPayload = CreateTaskPayload {
            text: text.to_string(),
        };
        let created = repository
            .create(create_payload)
            .await
            .expect("[create] failed.");
        assert_eq!(text, created.text);
        assert!(!created.completed);

        // find one
        let task = repository
            .find(created.id)
            .await
            .expect("[find one] failed.");
        assert_eq!(text, task.text);

        // find all
        let tasks = repository.all().await.expect("[all] failed.");
        assert_eq!(1, tasks.len());
        assert_eq!(created, *tasks.first().unwrap());

        // update
        let updated_text = "Successfully updated.";
        let update_payload: UpdateTaskPayload = UpdateTaskPayload {
            text: Some(updated_text.to_string()),
            completed: Some(false),
        };
        let updated = repository
            .update(created.id, update_payload)
            .await
            .expect("[update] failed.");
        assert_eq!(updated_text, updated.text);
        assert!(!updated.completed);

        // delete
        repository
            .delete(created.id)
            .await
            .expect("[delete] failed.");
        let tasks = repository.all().await.expect("[all] failed.");
        assert_eq!(0, tasks.len());
    }
}
