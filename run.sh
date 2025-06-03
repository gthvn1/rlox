#!/bin/sh

cargo build
(cd rlox-web;  wasm-pack build --target web)
python3 -m http.server
