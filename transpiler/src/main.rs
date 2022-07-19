extern crate smallvec;

mod codegen;
mod ir;
mod lexer;
mod parsing;

use lexer::{Lexer, TOKEN_EOF};
use std::env;
use std::fs;
use crate::lexer::TokenKind;

fn main() {
    let mut args = env::args().skip(1);
    let file_to_parse = args.next().expect("nothing to parse");
    let out_file = args.next().expect("no output file");

    let to_parse = fs::read_to_string(file_to_parse).expect("failed to read file");

    let mut lexer = Lexer::new(to_parse.as_str());
    let ast = parsing::build_ast(lexer).unwrap();

    let code = codegen::generate_c_code(&ast);
    fs::write(out_file, code).expect("failed to write into file");
}
