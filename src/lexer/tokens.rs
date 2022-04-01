use crate::Result;

/// Token struct contains [`token kind`] and meta data about the token
///
/// [`token kind`]: crate::lexer::tokens::TokenKind
#[derive(Debug)]
pub struct Token {
    pub token_kind: Result<TokenKind>,
    pub line_number: u32,
    pub column_number: u32,
}

impl Token {
    pub fn new(token_kind: Result<TokenKind>, line_number: u32, column_number: u32) -> Token {
        Token {
            token_kind,
            line_number,
            column_number,
        }
    }

    /// Intended for writing tests and debugging.
    /// Creates a Token from the [`TokenKind`] enum and sets all other
    /// fields to default values.
    ///
    /// [`TokenKind`]: crate::lexer::tokens::TokenKind
    pub fn from_kind(token_kind: TokenKind) -> Token {
        Token {
            token_kind: Ok(token_kind),
            line_number: 0,
            column_number: 0,
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

/// # Grammar
/// \<literal\> ::=
///     " {\<character\>} "
///     | {\<digit\>}
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
