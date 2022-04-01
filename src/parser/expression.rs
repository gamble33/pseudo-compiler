use crate::parser::Parser;
use crate::lexer::tokens::Token;
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
    Literal { value: Literal},
}