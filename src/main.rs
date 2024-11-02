mod models;
mod handlers;

use std::sync::Arc;

use axum::{Router, routing::get};
use axum::routing::post;
use axum::extract::Extension;

use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use crate::handlers::{create_new_post, get_all_posts};

const IMG_DIR: &str = "data/images";

#[tokio::main]
async fn main() {
    match db_connect().await {
        Ok(pool) => {
            let state = Arc::new(pool);
            start_server(state).await;
        },
        Err(_) => {
            println!("Failed to connect to database");
        }
    };
}

async fn start_server(state: Arc<Pool<Sqlite>>) {
    let app = Router::new()
        .route( "/api/posts", get(get_all_posts))
        .route("/api/posts", post(create_new_post))
        .nest_service("/home", ServeDir::new("public"))
        .nest_service("/content", ServeDir::new(IMG_DIR))
        .layer(Extension(state));

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn db_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect("sqlite:data/database.db").await
}

