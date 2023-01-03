use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::repositories::{CreateTaskPayload, TaskRepository};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn find_task<T: TaskRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let tasks = repository.find(id).await;

    (StatusCode::OK, Json(tasks))
}

pub async fn find_all_tasks<T: TaskRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let tasks = repository.all().await;

    (StatusCode::OK, Json(tasks))
}

// #[axum_macros::debug_handler]
pub async fn create_task<T: TaskRepository>(
    Json(payload): Json<CreateTaskPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    tracing::debug!("create");
    let task = repository.create(payload).await;

    (StatusCode::CREATED, Json(task))
}
