use std::sync::Arc;
use axum::extract::State;
use axum::response::Json;
use axum::response::Html;
use sqlx::{query_as, Pool, Sqlite};
use std::fs;

use crate::models::BlogPost;

pub async fn get_home_html(State(pool): State<Arc<Pool<Sqlite>>>) -> Html<String>  {
    let file_content = fs::read_to_string("index.html")
        .expect("Something went wrong reading the file");
    Html(file_content)
}

pub async fn get_all_posts(State(pool): State<Arc<Pool<Sqlite>>>) -> Json<Vec<BlogPost>> {
    let res = query_as
        ::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&*pool)
        .await;
    match res {
        Ok(posts) => Json(posts),
        Err(_) => Json(Vec::new()),
    }
}

pub async fn create_new_post() {}
