use crate::Result;

#[derive(Debug)]
pub struct Token {
    pub token_kind: Result<TokenKind>,
    pub line_number: u32,
}

impl Token {
    pub fn new(token_kind: Result<TokenKind>, line_number: u32) -> Token {
        Token {
            token_kind,
            line_number
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Keyword(String),
    Literal(Literal),
    Symbol(String),
    EndLine,
}

#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    Str(String),
}

pub const VALID_SYMBOLS: &[&str] = &[
    "=", "+", "-", "*", "/", "<", ">", "<=", ">=", "<>", "<-", "//",
];

pub const KEYWORDS: &[&str] = &[
    "FOR", "TO", "NEXT",
];
