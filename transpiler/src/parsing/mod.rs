mod error;
mod parser;
mod seq;

pub mod ast;

use crate::lexer::Lexer;
use error::Result;

pub fn build_ast(lexer: Lexer) -> Result<ast::AST> {
    parser::Parser::new(lexer).parse()
}
