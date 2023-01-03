mod handlers;
mod repositories;

use axum::{
    extract::Extension,
    // http::StatusCode,
    // response::IntoResponse,
    routing::{delete, get, patch, post},
    // Json,
    Router,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
// use thiserror::Error;

use handlers::{create_task, delete_task, find_all_tasks, find_task, root, update_task};
use repositories::{TaskRepository, TaskRepositoryForDb};

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // .env
    dotenv().ok();

    // connect DB
    let database_url = &env::var("DATABASE_URL").expect("undefined: [DATABASE_URL]");
    tracing::debug!("Connecting database...");
    let pool = PgPool::connect(database_url).await.expect(&format!(
        "Fail to connect database. url: [{}]",
        database_url
    ));
    let repository = TaskRepositoryForDb::new(pool);

    // route setting
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));

    // bind
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

fn create_app<T: TaskRepository>(repository: T) -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/tasks/:id", get(find_task::<T>))
        .route("/tasks", get(find_all_tasks::<T>))
        .route("/tasks", post(create_task::<T>))
        .route("/tasks/:id", patch(update_task::<T>))
        .route("/tasks/:id", delete(delete_task::<T>))
        .layer(Extension(Arc::new(repository)));
}