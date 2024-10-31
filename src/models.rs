#[derive(Debug, sqlx::FromRow)]
pub struct BlogPost {
    text: String,
    date: String,
    image: String,
    user: String,
    avatar: String,
}