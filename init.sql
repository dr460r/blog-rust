CREATE TABLE blog_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL,
    post_date TEXT NOT NULL,
    user_name TEXT NOT NULL,
    image_path TEXT,
    avatar_path TEXT
);
