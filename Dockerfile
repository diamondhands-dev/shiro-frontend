FROM rust:1.66-slim-bullseye AS buildbase

RUN apt-get update \
 && apt-get install -y libcrypt1-dev libssl-dev clang pkg-config git \
 && rm -fr /var/lib/apt/lists/* \
 && cargo install trunk \
 && rustup target add wasm32-unknown-unknown

FROM buildbase AS build
ARG API_ROOT=http://localhost:3000

WORKDIR /usr/src/myapp
COPY . .

ENV API_ROOT=${API_ROOT}
ENTRYPOINT ["trunk", "serve", "--port=3000", "--release", "--address", "0.0.0.0"]
