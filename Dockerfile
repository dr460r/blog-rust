# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.82.0
ARG APP_NAME=blog

################################################################################
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache pkgconfig clang lld git musl-dev openssl-dev openssl-libs-static sqlite

ENV OPENSSL_STATIC=1

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=public,target=public \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=init.sql,target=init.sql \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    sqlite3 database.db < init.sql && \
    cp ./target/release/$APP_NAME /bin/server && \
    cp ./database.db /bin/database.db && \
    cp -r ./public /bin/public

################################################################################
FROM alpine:3.18 AS final
WORKDIR /bin

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/
COPY --from=build /bin/database.db /bin/
COPY --from=build /bin/public /bin/

EXPOSE 3000
CMD ["./server"]
