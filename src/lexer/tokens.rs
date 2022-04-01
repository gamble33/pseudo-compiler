use crate::Result;

#[derive(Debug)]
pub struct Token {
    pub token_kind: Result<TokenKind>,
}

impl Token {
    pub fn new(token_kind: Result<TokenKind>) -> Token {
        Token {
            token_kind
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Keyword(String),
    Literal(Literal),
    Symbol(String)
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
