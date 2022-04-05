//! Front-End Compiler for Cambridge's pseudo-code.

pub mod lexer;
pub mod parser;

pub type Result<T> = std::result::Result<T, String>;

pub struct ErrorInfo<'a> {
    src: &'a str,
    line_number: u32,
    column_number: u32,
}

impl<'a> ErrorInfo<'a> {
    pub fn new(src: &'a str, line_number: u32, column_number: u32) -> ErrorInfo<'a> {
        ErrorInfo {
            src,
            line_number,
            column_number,
        }
    }
}

/// Print's a rustc inspired error message. If a [`Token`] is provided, then the line, line number and
/// column number are printed along with the error message. Otherwise, the error message alone is
/// printed.
///
/// [`Token`]: crate::lexer::tokens::Token
pub fn print_error(message: &str, src_path: &str, err_info: Option<ErrorInfo>) {
    if let Some(err) = err_info {
        let number_space: String = whitespace_string((err.line_number.to_string().len() + 2) as u32);
        println!("{} | {}", err.line_number, err.src.lines().enumerate().find(|line| line.0 == (err.line_number - 1) as usize).unwrap().1);
        let space_buf: String = whitespace_string(err.column_number);
        println!("{}| {}^", number_space, space_buf);
        println!("{}| {}, {}:{}:{}", number_space, message, src_path, err.line_number, err.column_number);
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