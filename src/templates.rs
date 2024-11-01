pub const HOME_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Blog</title>
</head>
<body>
<h1>Home</h1>
<button onclick="fetchPosts()">Fetch</button>
<div id="posts-root">
</div>
</body>
<script>
    let postsRoot = document.getElementById('posts-root')
    const fetchPosts = () => {
        postsRoot.innerHTML = '<b>Loading...</b>'
        fetch('http://localhost:3000/api/posts').then((res) => {
            res.json().then((posts) => {
                postsRoot.innerHTML = ''
                posts.forEach((post) => {
                    postsRoot.innerHTML += `<div>${post.text}</div>`
                })
            }).catch(() => postsRoot.innerHTML = `<b>Couldn't fetch posts.</b>`)
        }).catch(() => postsRoot.innerHTML = `<b>Couldn't fetch posts.</b>`)
    }

    fetchPosts()
</script>
</html>
"#;