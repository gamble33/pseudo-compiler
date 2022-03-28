#[derive(Debug)]
pub enum Token {
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
