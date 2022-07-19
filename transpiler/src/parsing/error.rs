use crate::lexer::TokenKind;
use smallvec::SmallVec;

#[derive(Debug)]
pub struct ParsingError {
    found: TokenKind,
    expected: SmallVec<[TokenKind; 4]>,
}

impl ParsingError {
    pub fn new<I: IntoIterator<Item = TokenKind>>(found: TokenKind, expected: I) -> ParsingError {
        ParsingError {
            found,
            expected: SmallVec::from_iter(expected),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParsingError>;
