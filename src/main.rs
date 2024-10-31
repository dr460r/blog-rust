mod templates;
mod models;

use axum::{
    routing::get,
    response::Html,
    Router,
};
use sqlx::{query_as, Pool, Sqlite};
use tokio::net::TcpListener;
use crate::models::BlogPost;

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/home", get(get_home_html).post(|| async {""}))
        .route( "/api/posts", get(get_all_posts).post(create_new_post));

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn db_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    sqlx::sqlite::SqlitePool::connect("sqlite:blog.db").await
}

async fn get_home_html() -> Html<&'static str>  {
    match db_connect().await {
        Ok(pool) => {
            let blog_post = query_as::<_, BlogPost>("SELECT * FROM blog_posts").fetch_all(&pool).await.unwrap();
        },
        Err(_) => { println!("db connection failed"); },
    };
    Html(templates::HOME_TEMPLATE)
}

async fn get_all_posts() {}

async fn create_new_post() {}
