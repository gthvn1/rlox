#!/bin/sh

cargo build
(cd rlox-web;  wasm-pack build --target web)
cp -f ./rlox-web/pkg/rlox_web.js ./static/rlox_web.js
cp -f ./rlox-web/pkg/rlox_web_bg.wasm ./static/rlox_web_bg.wasm
cargo run -p rlox-server
