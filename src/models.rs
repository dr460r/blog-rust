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

impl NewBlogPost {
    pub fn to_db_model(self) -> BlogPost {
        BlogPost {
            text: self.text,
            post_date: "".to_string(),
            user_name: "".to_string(),
            image_path: "".to_string(),
            avatar_path: "".to_string(),
        }
    }
}