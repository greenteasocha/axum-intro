use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, RwLock}};
use std::env;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TaskRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTaskPayload) -> Task;
    fn find(&self, id: i32) -> Option<Task>; // findされないかも
    fn all(&self) -> Vec<Task>; // array
    fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task>; // any
    fn delete(&self, id: i32) -> anyhow::Result<()>; // anyhowは何？
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTaskPayload {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTaskPayload {
    text: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: i32,
    text: String,
    completed: bool,
}

impl Task {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id, // id: id, と書かなくて良いらしい
            text, // text: text, と書かなくて良いらしい
            completed: false,
        }
    }
}

type TaskHashMap = HashMap<i32, Task>;

#[derive(Debug, Clone)]
pub struct TaskRepositoryForMemory {
    store: Arc<RwLock<TaskHashMap>>,
}

impl TaskRepositoryForMemory {
    pub fn new() -> Self {
        TaskRepositoryForMemory { store: Arc::default() }
    }
}

impl TaskRepository for TaskRepositoryForMemory {
    fn create(&self, payload: CreateTaskPayload) -> Task {
        todo!();
    }
    fn find(&self, id: i32) -> Option<Task> {
        todo!();
    }
    fn all(&self) -> Vec<Task> {
        todo!();
    }
    fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task> {
        todo!();
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // route setting
    let repository = TaskRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));

    // bind
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

fn create_app<T: TaskRepository>(repositoroy: T) -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(Extension(Arc::new(repositoroy)));
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            // .header(header::CONTENT_TYPE, mime::APPLICATION_JSON) // TODO: こちらだと失敗
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{ "name": "hoge" }"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        // let body = r#"{ "id": 1337, "name": "hoge" }"#;
        let user: User = serde_json::from_str(&body).expect("cannot convert user instance");

        assert_eq!(
            user,
            User {
                id: 1337,
                name: "hoge".to_string()
            }
        );
    }
}
