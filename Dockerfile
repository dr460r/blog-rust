# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.82.0

FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /build

RUN apk add --no-cache pkgconfig clang lld git musl-dev openssl-dev openssl-libs-static sqlite

ENV OPENSSL_STATIC=1

RUN mkdir -p /build/result/data

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=public,target=public \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=init.sql,target=init.sql \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    sqlite3 database.db < init.sql && \
    cp ./target/release/blog /build/result/blog && \
    cp ./database.db /build/result/data/database.db && \
    cp -r ./public /build/result/public


FROM alpine:3.18 AS final
WORKDIR /app

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

COPY --from=build /build/result/blog /app
COPY --from=build /build/result/data/ /app
COPY --from=build /build/result/public/ /app

VOLUME "/app/data"

EXPOSE 3000
CMD ["./blog"]
