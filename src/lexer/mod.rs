pub mod tokens;

use crate::lexer::tokens::*;
use crate::Result;

use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Iterator for Lexer {
    type Item = Result<Token>; 
    fn next(&mut self) -> Option<Self::Item> {
        let token: Self::Item;

        let first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                    break;
                }
                None => return None,
            }
        }

        // Identifier or Keyword
        if Self::is_identifier(first_char) && !first_char.is_numeric() {
            let mut name: String = first_char.to_string();
            self.get_next_char_while(&mut name, Self::is_identifier);

            if KEYWORDS.contains(&&name[..]) {
                token = Ok(Token::Keyword(name));
            } else {
                token = Ok(Token::Identifier(name));
            }
        }  

        // Literal
        else if first_char.is_numeric() {
            let mut value: String = first_char.to_string();
            self.get_next_char_while(&mut value, |c| c.is_numeric());
            token = match value.parse::<i32>() {
                Ok(i) => Ok(Token::Literal(Literal::Integer(i))),
                Err(_) => Err(format!("Integer literal {} is invalid", value)),
            };
        } 
        
        // Symbol
        else {
            let mut symbol: String = first_char.to_string();
            loop {
                if let Some(peek) = self.raw_data.peek() {
                    symbol.push(*peek);
                } else {
                    break;
                }

                if VALID_SYMBOLS.contains(&&symbol[..]) {
                    self.raw_data.next();
                } else {
                    symbol.pop();
                    break;
                }
            }

            token = match &symbol[..] {
                s if s == "//" => {
                    self.get_next_char_while(&mut String::new(), |c| c != '\n');
                    self.next()?
                },
                s if VALID_SYMBOLS.contains(&s) => Ok(Token::Symbol(symbol)),
                _ => Err(format!("Unkown token: {}", symbol)),
            };
        }

        Some(token)
    }
}

impl Lexer {
    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        Ok(Self::from_text(&std::fs::read_to_string(path)?))
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, condition: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if condition(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                },
                _ => break,
            }
        }
    }

    // Variable & function names (identifiers) can only be alphanumeric characters.
    fn is_identifier(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }
}
