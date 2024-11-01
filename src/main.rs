mod templates;
mod models;

use axum::{
    routing::get,
    response::{Html, Json},
    Router,
};
use sqlx::{query_as, Pool, Sqlite};
use tokio::net::TcpListener;
use crate::models::BlogPost;

use std::vec::Vec;

#[tokio::main]
async fn main() {
    match db_connect().await {
        Ok(pool) => {
            start_server(pool).await;
        },
        Err(_) => {
            println!("Failed to connect to database");
        }
    };
}

async fn start_server(pool: Pool<Sqlite>) {
    let app: Router = Router::new()
        .route("/home", get(get_home_html))
        .route( "/api/posts", get(get_all_posts).post(create_new_post));

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn db_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    sqlx::sqlite::SqlitePool::connect("sqlite:blog.db").await
}

async fn get_home_html() -> Html<&'static str>  {
    Html(templates::HOME_TEMPLATE)
}

async fn get_all_posts() -> Json<Vec<BlogPost>> {
    match db_connect().await {
        Ok(pool) => {
            let res = query_as
                ::<_, BlogPost>("SELECT * FROM blog_posts")
                .fetch_all(&pool)
                .await;
            match res {
                Ok(posts) => Json(posts),
                Err(_) => Json(Vec::new()),
            }
        },
        Err(_) => Json(Vec::new()),
    }
}

async fn create_new_post() {}
