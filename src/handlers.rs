use std::sync::Arc;
use std::fs;

use axum::{
    Extension,
    Json,
    response::Html,
    http::StatusCode,
};

use axum::extract::Multipart;

use sqlx::{query_as, Pool, Sqlite};
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

pub async fn create_new_post(
    Extension(pool): Extension<Arc<Pool<Sqlite>>>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {

    let mut bpost = BlogPost::empty();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        let file_ext = field
            .file_name()
            .and_then(|filename| filename.split('.').last())
            .map(|ext| ext.to_string());

        let data = field.bytes().await.unwrap().clone();

        match name.as_str() {
            "image" => {
                if data.len() > 0 {
                    // TODO: Save image to file from provided data
                    bpost.image_path = file_ext.unwrap();
                }
            },
            "avatar_url" => {
                // TODO: Download avatar from provided data
                bpost.avatar_path = String::from_utf8(data.to_vec()).unwrap();
            },
            "text" => {
                bpost.text = String::from_utf8(data.to_vec()).unwrap();
            },
            "user_name" => {
                bpost.user_name = String::from_utf8(data.to_vec()).unwrap();
            },
            _ => {}
        }
    }

    let sql: &str = "INSERT INTO blog_posts (text, user_name, post_date, image_path, avatar_path) VALUES ($1, $2, $3, $4, $5)";
    let res = sqlx::query(&sql)
        .bind(bpost.text)
        .bind(bpost.user_name)
        .bind(bpost.post_date)
        .bind(bpost.image_path)
        .bind(bpost.avatar_path)
        .execute(&*pool)
        .await;
    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            println!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        },
    }
}
