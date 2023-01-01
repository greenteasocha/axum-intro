use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::{CreateTaskPayload, TaskRepository};

// #[axum_macros::debug_handler]
pub async fn create_task<T: TaskRepository>(
    Json(payload): Json<CreateTaskPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let task = repository.create(payload);

    (StatusCode::CREATED, Json(task))
}
