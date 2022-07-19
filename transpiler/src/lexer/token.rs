use std::array;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenKind {
    Spaces,
    LineBreak,
    KeywordVoid,
    KeywordInt,
    Ident,
    Assign,
    IntegerLiteral,
    StringLiteral,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    Sep,
    Eof,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Token<'c> {
    kind: TokenKind,
    string: &'c str,
}

impl<'c> Token<'c> {
    pub const fn of(kind: TokenKind, string: &'c str) -> Token {
        Token { kind, string }
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub const fn string(&self) -> &'c str {
        self.string
    }
}

pub const TOKEN_EOF: Token = Token::of(TokenKind::Eof, "");

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! token_matcher {
    ( $(
        $variant:ident => {
            fn check($check_buf:tt : &str, $check_char_count:tt : usize, $check_character:tt : char) -> bool $check_body:block
            fn emit($emit_buf:tt : &str) -> Token $emit_body:block
        }
       ),* $(,)? ) => {
        #[derive(Eq, PartialEq, Copy, Clone, Debug)]
        pub enum TokenMatcher {
            Reset,
            $( $variant ),*
        }

        const TOKEN_MATCHER_VARIANT_COUNT: usize = count!($( $variant )*);

        impl TokenMatcher {
            pub fn all() -> array::IntoIter<TokenMatcher, TOKEN_MATCHER_VARIANT_COUNT> {
                [
                    $( TokenMatcher::$variant ),*
                ].into_iter()
            }

            pub fn check(self, buf: &str, char_count: usize, character: char) -> bool {
                match self {
                    TokenMatcher::Reset => false,
                    $( TokenMatcher::$variant => {
                        fn check_($check_buf: &str, $check_char_count: usize, $check_character: char) -> bool $check_body
                        check_(buf, char_count, character)
                    } ),*
                }
            }

            pub fn emit(self, buf: &str) -> Token {
                match self {
                    TokenMatcher::Reset => unreachable!(),
                    $( TokenMatcher::$variant => {
                        fn emit_($emit_buf: &str) -> Token $emit_body
                        emit_(buf)
                    } ),*
                }
            }
        }
    };
}

const fn is_line_break(char: char) -> bool {
    char == '\n' || char == '\r'
}

token_matcher! {
    Spaces => {
        fn check(_: &str, _: usize, char: char) -> bool {
            char == ' '
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::Spaces, buf)
        }
    },
    LineBreak => {
        fn check(_: &str, _: usize, char: char) -> bool {
            is_line_break(char)
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::LineBreak, buf)
        }
    },
    KeywordOrIdent => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            if buf.is_empty() {
                char.is_ascii_alphabetic()
            } else {
                char.is_ascii_alphanumeric()
            }
        }
        fn emit(buf: &str) -> Token {
            let kind = match buf {
                "void" => TokenKind::KeywordVoid,
                "int" => TokenKind::KeywordInt,
                _ => TokenKind::Ident,
            };
            Token::of(kind, buf)
        }
    },
    Assign => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == '='
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::Assign, buf)
        }
    },
    IntegerLiteral => {
        fn check(_: &str, _: usize, char: char) -> bool {
            char.is_ascii_digit()
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::IntegerLiteral, buf)
        }
    },
    StringLiteral => {
        fn check(buf: &str, char_count: usize, char: char) -> bool {
            match buf.chars().next_back() {
                Some(last_char) if char_count != 1 => last_char != '"', // buf.len() > 1
                None => char == '"', // buf is empty
                _ => true, // buf.len() == 1
            }
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::StringLiteral, buf)
        }
    },
    ParenOpen => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == '('
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::ParenOpen, buf)
        }
    },
    ParenClose => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == ')'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::ParenClose, buf)
        }
    },
    BraceOpen => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == '{'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::BraceOpen, buf)
        }
    },
    BraceClose => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == '}'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::BraceClose, buf)
        }
    },
    Sep => {
        fn check(buf: &str, _: usize, char: char) -> bool {
            buf.is_empty() && char == ','
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::Sep, buf)
        }
    },
}
