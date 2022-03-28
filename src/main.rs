use pseudo_compiler::lexer::Lexer;

use regex::Regex;
use std::process;

fn main() {
    // Lexical Analysis
    let mut lexer: Lexer = Lexer::from_file("test.pseudo").unwrap();
    println!("{:?}", lexer.collect::<Vec<_>>());
}
