mod utils;

use rlox_interpreter::interpreter::{parser::Parser, scanner::Scanner, token::Token};

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

#[allow(clippy::new_without_default)]
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

    pub fn parse(&mut self) -> String {
        if let Some(tokens) = &self.tokens {
            let _ = Parser::from_tokens(tokens);
            "Work is in progress...".to_string()
        } else {
            "Internal Error: scan not called".to_string()
        }
    }
}
