//! Front-End Compiler for Cambridge's pseudo-code.

pub mod lexer;
pub mod parser;

pub type Result<T> = std::result::Result<T, String>;
