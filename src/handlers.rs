use std::sync::Arc;
use axum::extract;
use axum::response;
use axum::response::Html;
use sqlx::{query_as, Error, Pool, Sqlite};
use std::fs;
use axum::Extension;
use axum::http::StatusCode;
use axum::Json;
use crate::models::{BlogPost, NewBlogPost};

pub async fn get_home_html(Extension(pool): Extension<Arc<Pool<Sqlite>>>) -> Html<String>  {
    let file_content = fs::read_to_string("index.html")
        .expect("Something went wrong reading the file");
    Html(file_content)
}

pub async fn get_all_posts(Extension(pool): Extension<Arc<Pool<Sqlite>>>) -> Json<Vec<BlogPost>> {
    let res = query_as
        ::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&*pool)
        .await;
    match res {
        Ok(posts) => Json(posts),
        Err(_) => Json(Vec::new()),
    }
}

#[axum_macros::debug_handler]
pub async fn create_new_post(
    Extension(pool): Extension<Arc<Pool<Sqlite>>>,
    Json(new_post): Json<NewBlogPost>,
) -> Result<StatusCode, StatusCode> {
    let p: BlogPost = new_post.to_db_model();
    let sql: &str = "INSERT INTO blog_posts (text, use_name, avatar_path) VALUES ($1, $2, $3)";
    let res = sqlx::query(&sql)
        .bind(p.text)
        .bind(p.user_name)
        .bind(p.avatar_path)
        .execute(&*pool)
        .await;
    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
