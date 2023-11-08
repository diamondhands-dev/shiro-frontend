FROM rust:1.72-bullseye as builder

RUN apt-get update \
 && apt-get install clang -y \
 && rustup target add wasm32-unknown-unknown \
 && CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install trunk@0.16.0
COPY ./ ./
RUN trunk build

FROM ghcr.io/diamondhands-dev/shiro-backend:0.4.7
COPY --from=builder dist/ app/
