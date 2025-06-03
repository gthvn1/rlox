use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String, // raw string find in source. As we will implement a REPL we use String because we're not sure about the lifetime of the source in a REPL
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: &str, line: usize) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
            line,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = if self.literal.is_empty() {
            "null"
        } else {
            self.literal.as_str()
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}
