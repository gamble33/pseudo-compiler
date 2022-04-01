use crate::parser::Parser;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::Literal;
use crate::Result;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expression> {
        // TODO: Parse expressions
        unimplemented!()
    }

    pub fn parse_literal(&mut self) -> Expression {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Expression {
    /// See grammar [`here`]
    ///
    /// [`here`]: crate::lexer::tokens::Literal
    Literal { value: Literal},
}