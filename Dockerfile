FROM rust:1.66-slim-bullseye AS buildbase

RUN apt-get update \
 && apt-get install -y libcrypt1-dev libssl-dev clang pkg-config git \
 && rm -fr /var/lib/apt/lists/* \
 && cargo install trunk \
 && rustup target add wasm32-unknown-unknown

FROM buildbase AS build
WORKDIR /usr/src/myapp
COPY . .

EXPOSE 3000
ENTRYPOINT ["trunk", "serve", "--port=3000"]
