use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct BlogPost {
    id: i32,
    text: String,
    post_date: String,
    user_name: String,
    image_path: String,
    avatar_path: String,
}