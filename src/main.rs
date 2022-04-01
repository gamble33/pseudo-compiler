
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
        .filter_map(|t| match t.token_kind {
            Ok(_) => Some(t),
            Err(err) => {
                println!("\n\n| {}", src.lines().enumerate().find(|line| line.0 == (t.line_number - 1) as usize).unwrap().1);
                let mut space_buf: String = String::new();
                for _ in 1..t.column_number {
                    space_buf.push(' ');
                }
                println!("| {}^", space_buf);
                println!("| {}, {}:{}:{}", err, source_path, t.line_number, t.column_number);
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
