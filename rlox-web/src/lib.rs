mod utils;

use rlox_interpreter::interpreter::scanner::Scanner;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rlox-web!");
}

#[wasm_bindgen]
pub fn scan(input: &str) -> String {
    let mut output = String::new();

    let scanner = Scanner::new(input);
    let (tokens, _error_code) = scanner.scan_tokens();
    for tok in tokens {
        output.push_str(&tok.to_string());
        output.push('\n');
    }

    output
}
