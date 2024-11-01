use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BlogPost {
    pub text: String,
    pub post_date: String,
    pub user_name: String,
    pub image_path: String,
    pub avatar_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlogPost {
    pub text: String,
    pub image: String,
    pub user_name: String,
    pub avatar_url: String,
}

impl BlogPost {
    pub fn empty() -> Self {
        BlogPost {
            text: String::from(""),
            post_date: String::from(""),
            user_name: String::from(""),
            image_path: String::from(""),
            avatar_path: String::from(""),
        }
    }
}
