# shiro-frontend
The wallet UI for the RGB assets (that is defined by LNP/BP)

# Prequisite

* [Rust](https://www.rust-lang.org/)
* [Trunk](https://trunkrs.dev/)
* [wasm32-unknown-unknown](https://doc.rust-lang.org/rustc/platform-support/wasm64-unknown-unknown.html)

# How to run

## Docker
TBA

## From sources
```
git clone https://github.com/diamondhands-dev/shiro-frontend.git
cd shiro-frontend
trunk serve --port=3000

// or you can specify the backend endpoint as following
// API_ROOT=http:localhost:8080 trunk serve --port=3000
```
