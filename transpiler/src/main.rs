mod lexer;

use lexer::{Lexer, TOKEN_EOF};
use std::env;
use std::fs;

fn main() {
    let file_to_parse = env::args().skip(1).next().expect("nothing to parse");
    // // let out_file = env::args().next().expect("no output file");

    let to_parse = fs::read_to_string(file_to_parse).expect("failed to read file");

    let mut lexer = Lexer::new(to_parse.as_str());
    loop {
        let token = lexer.next().expect("failed to tokenize");
        println!("{:?}", token);
        if token == TOKEN_EOF {
            break;
        }
    }
}
