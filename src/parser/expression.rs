use crate::parser::Parser;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::Literal;
use crate::Result;

impl Parser {
    /// \<expression\> ::= <term> + <term> | <term>
    ///
    pub fn parse_expression(&mut self) -> Result<Expression> {
        // TODO: Parse expressions
        self.parse_literal()
    }

    pub fn parse_literal(&mut self) -> Result<Expression> {
        match self.tokens.peek() {
            Some(crate::lexer::tokens::Token { token_kind: Ok(TokenKind::Literal(l)), .. }) => {
                Ok(Expression::Literal {
                    value: l.to_owned(),
                })
            }
            _ => Err(String::from("Expected Literal")),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    /// See grammar [`here`]
    ///
    /// [`here`]: crate::lexer::tokens::Literal
    Literal { value: Literal },
}