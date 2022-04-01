pub mod tokens;

use crate::lexer::tokens::*;
use crate::Result;

use std::iter::Peekable;
use std::vec::IntoIter;
use regex::Regex;

/// Lexer (or tokenizer) which creates a list of tokens defined in the [`Token`] enum
/// by implementing the Iterator trait
///
/// [`Token`]: ./tokens/enum.Token.html
pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Iterator for Lexer {
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        let token: Self::Item;
        let mut text: String = String::new();

        loop {
            match self.raw_data.peek() {
                Some(c) if c.is_whitespace() => {
                    self.raw_data.next();
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

        // Integer Literal
        if let Some(t) = Regex::new(r#"^\d+"#).unwrap().find(text.as_str()) {
            for _ in 0..t.end() {
                self.raw_data.next();
            }
            let value = t.as_str().parse::<i32>();
            token = match value {
                Ok(i) => Ok(Token::Literal(Literal::Integer(i))),
                _ => Err(format!("Invalid Integer: {}", t.as_str())),
            }
        }

        // String Literals
        else if let Some(t) = Regex::new(r#"^"[^"]*""#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            for _ in 0..t.end() {
                s.push(self.raw_data.next().unwrap());
            }
            let s = &s[1..s.len() - 1];
            token = Ok(Token::Literal(Literal::Str(s.to_owned())));
        }

        // Comments
        else if let Some(t) = Regex::new(r#"^//.*"#).unwrap().find(text.as_str()) {
            for _ in 0..t.end() {
                self.raw_data.next().unwrap();
            }
            token = self.next()?;
        }

        // Symbols
        else if let Some(t) = Regex::new(r#"^(<-|=)"#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            for _ in 0..t.end() {
                s.push(self.raw_data.next().unwrap());
            }
            token = Ok(Token::Symbol(s));
        }

        // Identifiers
        else if let Some(t) = Regex::new(r#"^[_a-zA-Z][_a-zA-Z0-9]*"#).unwrap().find(text.as_str()) {
            let mut s: String = String::new();
            for _ in 0..t.end() {
                self.raw_data.next().unwrap();
            }
            token = Ok(Token::Identifier(s));
        }
        else {
            token = Err(format!("Unexpected Token: {}", self.raw_data.next().unwrap()));
        }
        println!("{:?}", Some(&token));
        Some(token)
    }
}

impl Lexer {
    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<char>>().into_iter().peekable()
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        Ok(Self::from_text(&std::fs::read_to_string(path)?))
    }
}