# Simple Rust Blog

## Run with Docker

### Get image

Pull image from GitHub Container Registry:
```sh
docker pull ghcr.io/dr460r/blog-rust:main
```

or download repo and build it locally:
```sh
docker build . -t my-image-name
```

### Run container

Port `3000` is the one that app listens on, so it needs to be bound to local port.

_Optionally_, to persist data you can create local volume and bind it to `/app/data` directory inside container.

_Example_. Run container with added volume `myvol`, and listen on port `80`:
```sh
docker run -v myvol:/app/data -p 80:3000 ghcr.io/dr460r/blog-rust:main
```
or, if built locally:
```sh
docker run -v myvol:/app/data -p 80:3000 my-image-name
```
