use crate::lexer::TokenKind;
use smallvec::SmallVec;
use std::error;
use std::fmt;
use std::fmt::Write;

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

fn format_expected(expected: &[TokenKind]) -> String {
    let mut iter = expected.iter();
    match iter.next() {
        Some(first) => match iter.next_back() {
            Some(last) => {
                let mut string = format!("{:?} | ", first);
                for kind in iter {
                    string.push_str(format!("{:?} | ", kind).as_str());
                }
                string.push_str(format!("{:?}", last).as_str());
                string
            }
            None => format!("{:?}", first),
        },
        None => String::new(),
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.expected.is_empty() {
            f.write_fmt(format_args!("unexpected token {:?}", self.found))?;
        } else {
            let expected = format_expected(self.expected.as_slice());
            f.write_fmt(format_args!("expected {}, but found {:?}", expected, self.found))?;
        }
        Ok(())
    }
}

impl error::Error for ParsingError {}

pub type Result<T> = std::result::Result<T, ParsingError>;
