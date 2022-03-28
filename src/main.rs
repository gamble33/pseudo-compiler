use pseudo_compiler::lexer::Lexer;
use pseudo_compiler::parser::Parser;

use std::process;

fn main() {

    // Lexical Analysis
    let mut lexer: Lexer = Lexer::from_file("test.pseudo").unwrap();
    let tokens = lexer
        .map(|token| token.unwrap_or_else(|x| {
            println!("Lexing Error: {:?}", x);
            process::exit(1);
        }))
        .collect::<Vec<_>>();
    
    // Syntactic Analysis
    let parser: Parser = Parser::new(tokens.into_iter().peekable());
}
