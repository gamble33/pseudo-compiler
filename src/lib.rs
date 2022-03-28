//! Front-End Compiler for Cambdridge's psuedo-code.

pub mod lexer;
pub mod parser;

pub type Result<T> = std::result::Result<T, String>;
