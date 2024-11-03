ARG RUST_VERSION=1.82.0

FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /build

RUN apk add --no-cache pkgconfig clang lld git musl-dev openssl-dev openssl-libs-static sqlite tree

ENV OPENSSL_STATIC=1

ADD ./avatar.png /build/data/images/
ADD ./public /build/public/

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=init.sql,target=init.sql \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    sqlite3 database.db < init.sql && \
    cp ./target/release/blog /build/blog && \
    cp ./database.db /build/data/database.db

RUN tree /build


FROM alpine:3.18 AS final
WORKDIR /app

#COPY ./data /app/data_temp/

COPY --from=build /build/blog /app/
COPY --from=build /build/data /app/data/
COPY --from=build /build/public /app/public/

#RUN cp -r ./data_temp/* ./data/
#RUN rm -r ./data_temp

VOLUME "/app/data"
#VOLUME "/app/data/images"

RUN apk add --no-cache tree
RUN tree .

#RUN echo "#!/bin/sh\ncp -r ./data_temp/* ./data/ && ./blog" > run.sh
#RUN chmod +x run.sh

EXPOSE 3000
#CMD ["sh", "run.sh"]
CMD ["./blog"]
