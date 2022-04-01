use crate::parser::Parser;
use crate::lexer::tokens::Literal;

impl Parser {
    pub fn parse_expression(&mut self) -> Expression {
        // TODO: Parse expressions
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal { value: Literal},
}