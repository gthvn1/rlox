use crate::interpreter::error::report_error;
use crate::interpreter::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    source: &'a str,
    char_iter: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    line: usize, // track the source line
    error_code: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            char_iter: source.char_indices().peekable(),
            tokens: Vec::new(),
            line: 1,
            error_code: 0,
        }
    }

    // Move the ownership of tokens to the caller
    pub fn scan_tokens(mut self) -> (Vec<Token>, i32) {
        while let Some((idx, c)) = self.char_iter.next() {
            match c {
                '(' => self.push_token(TokenType::LeftParen, idx, 1),
                ')' => self.push_token(TokenType::RightParen, idx, 1),
                '{' => self.push_token(TokenType::LeftBrace, idx, 1),
                '}' => self.push_token(TokenType::RightBrace, idx, 1),
                ',' => self.push_token(TokenType::Comma, idx, 1),
                '.' => self.push_token(TokenType::Dot, idx, 1),
                '-' => self.push_token(TokenType::Minus, idx, 1),
                '+' => self.push_token(TokenType::Plus, idx, 1),
                ';' => self.push_token(TokenType::Semicolon, idx, 1),
                '*' => self.push_token(TokenType::Star, idx, 1),
                '/' => {
                    if let Some((_, '/')) = self.char_iter.peek() {
                        self.skip_until_eol()
                    } else {
                        self.push_token(TokenType::Slash, idx, 1)
                    }
                }
                '!' => {
                    if let Some((_, '=')) = self.char_iter.peek() {
                        let _ = self.char_iter.next();
                        self.push_token(TokenType::BangEqual, idx, 2)
                    } else {
                        self.push_token(TokenType::Bang, idx, 1)
                    }
                }
                '=' => {
                    // Need to peek the next character to check if it is Equal or EqualEqual
                    if let Some((_, '=')) = self.char_iter.peek() {
                        let _ = self.char_iter.next();
                        self.push_token(TokenType::EqualEqual, idx, 2)
                    } else {
                        self.push_token(TokenType::Equal, idx, 1)
                    }
                }
                '"' => self.read_string(idx),
                c if c.is_ascii_digit() => self.read_number(c),
                c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(c),
                ' ' | '\t' | '\r' => {} // ignore whitespace.
                '\n' => {
                    self.line += 1;
                }
                _ => {
                    report_error(self.line, format!("unknown character {}", c).as_str());
                    self.error_code = 65;
                }
            };
        }

        // Before returning add the EOF token
        self.tokens
            .push(Token::new(TokenType::Eof, "", "", self.line));

        (self.tokens, self.error_code)
    }

    fn push_token(&mut self, token_type: TokenType, lex_start: usize, lex_size: usize) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[lex_start..lex_start + lex_size],
            "",
            self.line,
        ))
    }

    fn skip_until_eol(&mut self) {
        match self.char_iter.peek() {
            None | Some((_, '\n')) => {
                // We don't consume \n so the line will be incremented in the main scan loop
            }
            _ => {
                let _ = self.char_iter.next();
                self.skip_until_eol();
            }
        }
    }

    // String is all character until we reach then closing quote.
    fn read_string(&mut self, start_idx: usize) {
        loop {
            match self.char_iter.next() {
                Some((idx, '"')) => {
                    self.push_token(TokenType::String, start_idx, idx - start_idx + 1);
                    break;
                }
                Some((_, '\n')) => {
                    self.line += 1;
                }
                Some(_) => {}
                None => {
                    report_error(self.line, "unterminated string");
                    self.error_code = 65;
                }
            }
        }
    }

    fn read_number(&mut self, first_digit: char) {
        let mut num_lex = first_digit.to_string();
        let mut has_decimal = false;

        // First read the integer part
        loop {
            match self.char_iter.peek() {
                Some((_, c)) if c.is_ascii_digit() => {
                    num_lex.push(*c);
                    self.char_iter.next(); // consume the character
                }
                Some((_, '.')) => {
                    num_lex.push('.');
                    self.char_iter.next(); // consume the character
                    has_decimal = true;
                    break;
                }
                Some(_) | None => break,
            }
        }

        if has_decimal {
            loop {
                match self.char_iter.peek() {
                    Some((_, c)) if c.is_ascii_digit() => {
                        num_lex.push(*c);
                        self.char_iter.next(); // consume the character
                    }
                    Some(_) | None => break,
                }
            }
        }

        // remove all extras 0s
        let fnum: f64 = num_lex.parse::<f64>().unwrap();
        let mut num_lit = fnum.to_string();
        // For integer add a floating part
        if !num_lit.contains('.') {
            num_lit.push_str(".0");
        }

        self.tokens
            .push(Token::new(TokenType::Number, &num_lex, &num_lit, self.line));
    }

    fn read_identifier(&mut self, first_char: char) {
        let mut ident = first_char.to_string();

        loop {
            match self.char_iter.peek() {
                Some((_, c)) if c.is_ascii_digit() || c.is_ascii_alphabetic() || *c == '_' => {
                    ident.push(*c);
                    self.char_iter.next();
                }
                _ => break,
            }
        }

        // We need to check if it is a keyword
        let token_type = match ident.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.tokens
            .push(Token::new(token_type, &ident, "", self.line));
    }
}
