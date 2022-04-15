pub mod tokens;

use crate::lexer::tokens::*;
use crate::Result;

use std::iter::Peekable;
use std::vec::IntoIter;

// TODO: Don't use regex
use regex::Regex;

/// Lexer (or tokenizer) which creates a list of [`Token`]s defined in the [`TokenKind`] enum
/// by implementing the Iterator trait
///
/// [`Token`]: crate::lexer::tokens::Token
/// [`TokenKind`]: crate::lexer::tokens::TokenKind
pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
    line_count: u32,
    column_count: u32,
}

impl Iterator for Lexer {
    type Item = Token;

    /// Returns next [`Token`]
    ///
    /// [`Token`]: crate::lexer::tokens::Token
    fn next(&mut self) -> Option<Self::Item> {
        let token: Token;
        let token_kind: Result<TokenKind>;

        let mut text: String = String::new();

        loop {
            match self.raw_data.peek() {
                Some(c) if *c == ' ' => {
                    self.raw_data.next();
                    self.column_count += 1;
                    continue;
                }
                Some(_) => {
                    break;
                }
                None => return None,
            }
        }

        // TODO: Stop cloning String, allow regex expression matching with &str slice.
        for c in self.raw_data.clone().collect::<Vec<char>>() {
            text.push(c);
        }

        // End Line
        if let Some(t) = Regex::new(r#"^((\r\n)|\n)"#).unwrap().find(text.as_str()) {
            self.eat_and_read_chars(None, t.end());
            token_kind = Ok(TokenKind::EndLine);
            self.line_count += 1;
            self.column_count = 1;
        }

        // Integer Literal
        else if let Some(t) = Regex::new(r#"^\d+"#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            self.eat_and_read_chars(Some(&mut s), t.end());
            let value = s.as_str().parse::<i32>();
            token_kind = match value {
                Ok(i) => Ok(TokenKind::Literal(Literal::Integer(i))),
                Err(err) => Err(format!("Invalid Integer: {}, {}", t.as_str(), err)),
            }
        }

        // String Literals
        else if let Some(t) = Regex::new(r#"^"[^"\r\n]*""#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            self.eat_and_read_chars(Some(&mut s), t.end());
            let s = &s[1..s.len() - 1];
            token_kind = Ok(TokenKind::Literal(Literal::Str(s.to_owned())));
        }

        // Comments
        else if let Some(t) = Regex::new(r#"^//.*"#).unwrap().find(text.as_str()) {
            self.raw_data.next().unwrap();
            self.eat_and_read_chars(None, t.end());
            token_kind = self.next()?.token_kind;
        }

        // Symbols
        else if let Some(t) = Regex::new(r#"^(<-|=)"#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            self.eat_and_read_chars(Some(&mut s), t.end());
            token_kind = Ok(TokenKind::Symbol(s));
        }

        // Identifiers
        else if let Some(t) = Regex::new(r#"^[_a-zA-Z][_a-zA-Z0-9]*"#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            self.eat_and_read_chars(Some(&mut s), t.end());
            token_kind = Ok(TokenKind::Identifier(s));
        } else {
            token_kind = Err(format!("Unexpected Token: `{}`", self.raw_data.next().unwrap()));
        }

        token = Token::new(token_kind, self.line_count, self.column_count);
        Some(token)
    }
}

impl Lexer {
    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<char>>().into_iter().peekable(),
            line_count: 1u32,
            column_count: 1u32,
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        Ok(Self::from_text(&std::fs::read_to_string(path)?))
    }

    fn eat_and_read_chars(&mut self, buf: Option<&mut String>, end: usize) {
        match buf {
            Some(s) => for _ in 0..end {
                s.push(self.raw_data.next().unwrap());
                self.column_count += 1
            },
            None => for _ in 0..end {
                self.raw_data.next();
                self.column_count += 1
            }
        }
    }
}
