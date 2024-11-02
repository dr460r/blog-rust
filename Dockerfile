ARG RUST_VERSION=1.82.0

FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /build

RUN apk add --no-cache pkgconfig clang lld git musl-dev openssl-dev openssl-libs-static sqlite tree

ENV OPENSSL_STATIC=1

ADD ./avatar.png /build/data/images/
ADD ./public /build/public/

#    --mount=type=cache,target=/usr/local/cargo/git/db \
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=init.sql,target=init.sql \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    sqlite3 database.db < init.sql && \
    cp ./target/release/blog /build/blog && \
    cp ./database.db /build/data/database.db

RUN tree /build


FROM alpine:3.18 AS final
WORKDIR /app

#ARG UID=10001
#RUN adduser \
#    --disabled-password \
#    --gecos "" \
#    --home "/nonexistent" \
#    --shell "/sbin/nologin" \
#    --no-create-home \
#    --uid "${UID}" \
#    appuser
#USER appuser

#RUN mkdir -p /app/data/images

#COPY --from=build /build/result/blog /app/
#COPY --from=build /build/result/data /app/data/
#COPY --from=build /build/result/data/images /app/data/images/
#COPY --from=build /build/result/public /app/public/

COPY --from=build /build/blog /app/
COPY --from=build /build/data /app/data/
COPY --from=build /build/public /app/public/

#VOLUME "/app/data"

EXPOSE 3000
CMD ["./blog"]
