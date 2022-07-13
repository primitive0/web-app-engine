use std::array;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenKind {
    Keyword,
    Ident,
    Sep,
    Assign,
    LiteralInteger,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
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

    pub fn string(&self) -> &'c str {
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
            fn check($check_buf:tt : &str, $check_character:tt : char) -> bool $check_body:block
            fn emit($emit_buf:tt : &str) -> Token $emit_body:block
        }
       ),* ) => {
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

            pub fn check(self, buf: &str, character: char) -> bool {
                match self {
                    TokenMatcher::Reset => false,
                    $( TokenMatcher::$variant => {
                        fn check_($check_buf: &str, $check_character: char) -> bool $check_body
                        check_(buf, character)
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
    Sep => {
        fn check(_: &str, char: char) -> bool {
            char == ' ' || is_line_break(char)
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::Sep, buf)
        }
    },
    KeywordOrIdent => {
        fn check(buf: &str, char: char) -> bool {
            if buf.is_empty() {
                char.is_ascii_alphabetic()
            } else {
                char.is_ascii_alphanumeric()
            }
        }
        fn emit(buf: &str) -> Token {
            let kind = match buf {
                "void" | "int" => TokenKind::Keyword,
                _ => TokenKind::Ident,
            };
            Token::of(kind, buf)
        }
    },
    Assign => {
        fn check(buf: &str, char: char) -> bool {
            buf.is_empty() && char == '='
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::Assign, buf)
        }
    },
    LiteralInteger => {
        fn check(_: &str, char: char) -> bool {
            char.is_ascii_digit()
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::LiteralInteger, buf)
        }
    },
    ParenOpen => {
        fn check(buf: &str, char: char) -> bool {
            buf.is_empty() && char == '('
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::ParenOpen, buf)
        }
    },
    ParenClose => {
        fn check(buf: &str, char: char) -> bool {
            buf.is_empty() && char == ')'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::ParenClose, buf)
        }
    },
    BraceOpen => {
        fn check(buf: &str, char: char) -> bool {
            buf.is_empty() && char == '{'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::BraceOpen, buf)
        }
    },
    BraceClose => {
        fn check(buf: &str, char: char) -> bool {
            buf.is_empty() && char == '}'
        }
        fn emit(buf: &str) -> Token {
            Token::of(TokenKind::BraceClose, buf)
        }
    }
}
