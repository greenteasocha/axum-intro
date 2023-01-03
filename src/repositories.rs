use axum::{async_trait, Json};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
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

type TaskHashMap = HashMap<i32, Task>;

#[async_trait]
pub trait TaskRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTaskPayload) -> Option<Task>;
    async fn find(&self, id: i32) -> Option<Task>; // findされないかも
    async fn all(&self) -> Vec<Task>; // array
    async fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task>; // any
    async fn delete(&self, id: i32) -> anyhow::Result<()>; // anyhowは何？
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
    async fn create(&self, payload: CreateTaskPayload) -> Option<Task> {
        tracing::debug!("create");
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (text, completed) values ($1, false) returning *
            "#,
        )
        .bind(payload.text)
        .fetch_one(&self.pool)
        .await
        .ok()?;
        tracing::debug!("finding id: {}", serde_json::to_string(&task).unwrap());
        Some(task)
    }
    async fn find(&self, id: i32) -> Option<Task> {
        tracing::debug!("finding id: {}", id);
        // FIXME: なぜかbindか効かないのでクエリに直書きしている。とりあえずDBからデータ取れることだけ確認。
        let task = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks where id = 1 limit 1", // "SELECT * FROM tasks where id = ? limit 1"
        )
        // .bind(1) // ???
        .fetch_one(&self.pool)
        .await
        .ok()?;
        Some(task)
    }
    async fn all(&self) -> Vec<Task> {
        todo!();
    }
    async fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task> {
        todo!();
    }
    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct TaskRepositoryForMemory {
    store: Arc<RwLock<TaskHashMap>>,
}

impl TaskRepositoryForMemory {
    pub fn new() -> Self {
        TaskRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryForMemory {
    async fn create(&self, payload: CreateTaskPayload) -> Option<Task> {
        todo!();
    }
    async fn find(&self, id: i32) -> Option<Task> {
        todo!();
    }
    async fn all(&self) -> Vec<Task> {
        todo!();
    }
    async fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task> {
        todo!();
    }
    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}
