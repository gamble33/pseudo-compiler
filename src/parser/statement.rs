use crate::parser::Parser;
use crate::parser::expression::Expression;
use crate::lexer::tokens::Token;
use crate::Result;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement> {
        match self.tokens.peek() {
            _ => self.parse_variable_assignment(),
        }
    }

    pub fn parse_variable_assignment(&mut self) -> Result<Statement> {
        let identifier = match self.tokens.next() {
            Some(Token::Identifier(id)) => id,
            t => return Err(format!("Expected token identifier, found: {:?}.", t)),
        };

        match self.tokens.peek() {
            Some(Token::Symbol(s)) => match s.as_str() {
                "<-" => self.tokens.next(),
                _ => return Err(format!("Expected symbol: '<-', found: '{}'", s)),
            },
            _ => return Err(String::from("Expected symbol: '<-'.")),
        };

        let value: Expression = self.parse_expression();

        Ok(Statement::VariableAssignment {
            identifier,
            value,
        })
    }
}

#[derive(Debug)]
pub enum Statement {
    /// # Grammar
    /// \<variable-assignment\> ::= \<identifier\> "<-" \<[`expression`]\>
    ///
    /// [`expression`]: ../expression/enum.Expression.html
    VariableAssignment {
        identifier: String,
        value: Expression,
    },
}
