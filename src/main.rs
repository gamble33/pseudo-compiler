use pseudo_compiler::lexer::Lexer;

fn main() {
    // Lexical Analysis
    let lexer: Lexer = Lexer::from_file("test.pseudo").unwrap();
    println!("{:?}", lexer.collect::<Vec<_>>());
}
