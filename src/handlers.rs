use std::sync::Arc;

use tokio::fs::File;

use axum::{
    Extension,
    Json,
    http::StatusCode,
};

use axum::extract::Multipart;

use sqlx::{query_as, Pool, Sqlite};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::IMG_DIR;
use crate::models::{BlogPost};

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

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        let file_ext = field
            .file_name()
            .and_then(|filename| filename.split('.').last())
            .map(|ext| ext.to_string());

        let data = field.bytes().await.unwrap().clone();
        let img_uuid = Uuid::new_v4().to_string();
        let img_dir = IMG_DIR;

        match name.as_str() {
            "image" => {
                if data.len() > 0 {
                    let filename = format!("post_image_{}.{}", img_uuid, file_ext.unwrap());
                    let filepath = format!("{}/{}", img_dir, filename);
                    bpost.image_path = filename;

                    let mut file = File::create(filepath).await.unwrap();
                    file.write_all(&data).await.unwrap();
                }
            },
            "avatar_url" => {
                let filename = format!("avatar_image_{}", img_uuid);
                let filepath = format!("{}/{}", img_dir, filename);
                bpost.avatar_path = filename;

                let url = String::from_utf8(data.to_vec()).unwrap();
                let response = reqwest::get(url).await.unwrap();
                if response.status().is_success() {
                    let mut file = File::create(filepath).await.unwrap();
                    let content = response.bytes().await.unwrap();
                    file.write_all(&content).await.unwrap();
                } else {
                    println!("Failed to download file: {}", response.status());
                }
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
    bpost.post_date = chrono::Utc::now().to_rfc3339();

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
