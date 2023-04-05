FROM rust:1.68-slim-bullseye as builder

RUN apt-get update \
 && apt-get install clang git-core -y \
 && rustup target add wasm32-unknown-unknown \
 && cargo install --config net.git-fetch-with-cli=true trunk
COPY ./ ./
RUN trunk build

FROM ghcr.io/diamondhands-dev/shiro-backend:0.4.4
COPY --from=builder dist/ app/
