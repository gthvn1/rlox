# Rlox — A Lox Interpreter in Rust

- Rlox is an implementation of the Lox language (from [Crafting Interpreters](https://craftinginterpreters.com)) written in Rust. It includes a command-line interpreter as well as a WebAssembly-based REPL that runs in the browser.
- Some examples of Lox code are [here](https://github.com/munificent/craftinginterpreters/tree/01e6f5b8f3e5dfa65674c2f9cf4700d73ab41cf8/test/scanning)

## Project Structure

This is a Cargo workspace with the following members:

- `rlox-interpreter`: Core Lox interpreter logic
- `rlox-web`: WebAssembly REPL for running Lox in the browser
- `rlox-server` (WIP): Lightweight HTTP server in Rust

## Getting Started

### Build & Run

- We are using [wasm-pack](https://github.com/rustwasm/wasm-pack) to build the wasm module.

```bash
cargo build
(cd rlox-web;  wasm-pack build --target web)
python3 -m http.server
```

- You should be able to open your browser at `http://localhost:8000` (maybe hard reload it...).
- While our `rlox-server` is a work in progress we use the python module http.
