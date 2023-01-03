use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::repositories::{CreateTaskPayload, TaskRepository, UpdateTaskPayload};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn find_task<T: TaskRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let tasks = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn find_all_tasks<T: TaskRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let tasks = repository.all().await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn create_task<T: TaskRepository>(
    Json(payload): Json<CreateTaskPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("create");
    let task = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task<T: TaskRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTaskPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("create");
    let task = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn delete_task<T: TaskRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
