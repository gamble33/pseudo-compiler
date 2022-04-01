use pseudo_compiler::lexer::Lexer;
use pseudo_compiler::parser::Parser;

fn main() {
    let source_path = "test.pseudo";


    // Lexical Analysis
    let lexer: Lexer = Lexer::from_file(source_path).unwrap();
    let tokens = lexer
        .filter_map(|t| match t.token_kind {
            Ok(_) => Some(t),
            Err(err) => {
                // TODO: Add line number to error
                println!("{}, {}:{}", err, source_path, t.line_number);
                std::process::exit(1);
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .peekable();

    // Syntax Analysis
    let mut parser: Parser = Parser::new(tokens);
    println!("{:?}", parser.parse());
}
