mod utils;

use rlox_interpreter::interpreter::{scanner::Scanner, token::Token};

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
pub struct Session {
    tokens: Option<Vec<Token>>,
}

#[wasm_bindgen]
impl Session {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Session {
        Session { tokens: None }
    }

    pub fn scan(&mut self, input: &str) -> String {
        let scanner = Scanner::new(input);
        let (tokens, _error_code) = scanner.scan_tokens();
        self.tokens = Some(tokens.clone());

        tokens
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
