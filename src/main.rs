mod handlers;
mod repositories;

use axum::{
    extract::Extension,
    // http::StatusCode,
    // response::IntoResponse,
    routing::{get, post},
    // Json,
    Router,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
// use thiserror::Error;

use handlers::{create_task, find_all_tasks, find_task, root};
use repositories::{TaskRepository, TaskRepositoryForDb, TaskRepositoryForMemory};

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
        "Fail to connect database. ur;: [{}]",
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
        .layer(Extension(Arc::new(repository)));
}

// #[derive(Deserialize)]
// struct CreateUser {
//     name: String,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct User {
//     id: u32,
//     name: String,
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{header, Method, Request},
//     };
//     use tower::ServiceExt;

//     #[tokio::test]
//     async fn should_return_hello_world() {
//         let req = Request::builder().uri("/").body(Body::empty()).unwrap();

//         let repository = TaskRepositoryForMemory::new();
//         let res = create_app(repository).oneshot(req).await.unwrap();

//         let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
//         let body: String = String::from_utf8(bytes.to_vec()).unwrap();

//         assert_eq!(body, "Hello, World!");
//     }

//     #[tokio::test]
//     async fn should_return_user_data() {
//         let req = Request::builder()
//             .uri("/users")
//             .method(Method::POST)
//             // .header(header::CONTENT_TYPE, mime::APPLICATION_JSON) // TODO: こちらだと失敗
//             .header("Content-Type", "application/json")
//             .body(Body::from(r#"{ "name": "hoge" }"#))
//             .unwrap();

//         let repository = TaskRepositoryForMemory::new();
//         let res = create_app(repository).oneshot(req).await.unwrap();

//         let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
//         let body = String::from_utf8(bytes.to_vec()).unwrap();
//         // let body = r#"{ "id": 1337, "name": "hoge" }"#;
//         let user: User = serde_json::from_str(&body).expect("cannot convert user instance");

//         assert_eq!(
//             user,
//             User {
//                 id: 1337,
//                 name: "hoge".to_string()
//             }
//         );
//     }
// }
