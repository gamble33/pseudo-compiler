use pseudo_compiler::lexer::Lexer;
use pseudo_compiler::parser::Parser;

fn main() {

    // Lexical Analysis
    let lexer: Lexer = Lexer::from_file("test.pseudo").unwrap();
    let tokens = lexer
        .map(|t| t.unwrap_or_else(|err| {
            println!("{}", err);
            std::process::exit(1);
        }))
        .collect::<Vec<_>>()
        .into_iter()
        .peekable();

    // Syntax Analysis
    let mut parser: Parser = Parser::new(tokens);
    println!("{:?}", parser.parse());
}
