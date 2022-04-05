use peek_nth::IteratorExt;
use pseudo_compiler::lexer::Lexer;
use pseudo_compiler::parser::Parser;

fn main() {
    let source_path = "test.pseudo";
    let src = std::fs::read_to_string(source_path).unwrap_or_else(|e| {
        println!("File Error: {}, (path: {})", e, source_path);
        std::process::exit(1);
    });
    let mut exit_due_to_err: bool = false;

 
    // Lexical Analysis
    let lexer: Lexer = Lexer::from_text(&src);
    let tokens = lexer
        .filter_map(|t| match t.clone().token_kind {
                Ok(_) => Some(t),
            Err(err) => {
                pseudo_compiler::print_error(Some(&t), &err, &src, source_path);
                exit_due_to_err = true;
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .peekable_nth();

    if exit_due_to_err { std::process::exit(1); }

    // Syntax Analysis
    let mut parser: Parser = Parser::new(tokens);
    println!("{:?}", parser.parse());
}
