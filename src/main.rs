use pseudo_compiler::lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::from_file("test.pseudo").unwrap();
    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    };
}
