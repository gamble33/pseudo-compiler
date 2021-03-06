use crate::parser::Parser;
use crate::parser::expression::Expression;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::Token;
use crate::Result;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement> {
        match self.tokens.peek() {
            Some( Token { token_kind: Ok(TokenKind::EndLine), .. }) => {
                self.tokens.next();
                self.parse_statement()
            }
            _ => self.parse_variable_assignment(),
        }
    }

    pub fn parse_variable_assignment(&mut self) -> Result<Statement> {
        let identifier = match self.tokens.next() {
            Some( Token { token_kind: Ok(TokenKind::Identifier(s)), .. }) => s,
            Some(t) => return Err(format!("Expected token identifier, found: {:?}.", t.token_kind)),
            _ => return Err(String::from("Expected token identifier, found: EOF.")),
        };

        match self.tokens.peek() {
            Some( Token{ token_kind: Ok(TokenKind::Symbol(s)), .. }) => match s.as_str() {
                "<-" => self.tokens.next(),
                _ => return Err(format!("Expected symbol: '<-', found: '{}'", s)),
            },
            _ => return Err(String::from("Expected symbol: '<-'.")),
        };

        let value: Expression = match self.parse_expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

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
