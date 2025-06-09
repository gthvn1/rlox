#!/bin/sh

cargo build
(cd rlox-web;  wasm-pack build --target web)
cargo run -p rlox-server
