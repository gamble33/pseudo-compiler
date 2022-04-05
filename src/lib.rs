//! Front-End Compiler for Cambridge's pseudo-code.

pub mod lexer;
pub mod parser;

pub type Result<T> = std::result::Result<T, String>;

/// Print's a rustc inspired error message. If a [`Token`] is provided, then the line, line number and
/// column number are printed along with the error message. Otherwise, the error message alone is
/// printed.
///
/// [`Token`]: crate::lexer::tokens::Token
pub fn print_error(token: Option<&crate::lexer::tokens::Token>, message: &str, src: &str, src_path: &str) {
    if let Some(t) = token {
        println!("| {}", src.lines().enumerate().find(|line| line.0 == (t.line_number - 1) as usize).unwrap().1);
        let space_buf: String = whitespace_string(t.column_number);
        println!("| {}^", space_buf);
        println!("| {}, {}:{}:{}", message, src_path, t.line_number, t.column_number);
    } else {
        println!("| {}, {}", message, src_path);
    }
}

fn whitespace_string(n: u32) -> String {
    let mut buf: String = String::new();
    for _ in 1..n {
        buf.push(' ');
    }
    return buf;
}