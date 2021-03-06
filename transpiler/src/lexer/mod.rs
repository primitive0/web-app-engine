mod token;

pub use token::*;

#[derive(Debug)]
pub struct TokenMatcherNotFound;

pub struct Lexer<'c> {
    left: &'c str,
    buf_pos: usize,
    char_count: usize,
    matcher: TokenMatcher,
}

impl<'c> Lexer<'c> {
    pub fn new(left: &str) -> Lexer {
        Lexer {
            left,
            buf_pos: 0,
            char_count: 0,
            matcher: TokenMatcher::Reset,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let mut chars = (&self.left[self.buf_pos..]).chars();
        let char = chars.next()?;
        return Some(char);
    }

    fn buf(&self) -> &'c str {
        &self.left[..self.buf_pos]
    }

    fn inc_buf(&mut self, n: usize) {
        self.buf_pos = (self.buf_pos + n).min(self.left.len());
        self.char_count += 1;
    }

    fn reset_buf(&mut self) {
        self.left = &self.left[self.buf_pos..];
        self.buf_pos = 0;
        self.char_count = 0;
    }

    // looks better without collapsing
    #[allow(clippy::collapsible_else_if)]
    pub fn next(&mut self) -> Result<Token<'c>, TokenMatcherNotFound> {
        while let Some(char) = self.next_char() {
            if self.matcher.check(self.buf(), self.char_count, char) {
                self.inc_buf(char.len_utf8());
            } else {
                if self.matcher != TokenMatcher::Reset {
                    let token = self.matcher.emit(self.buf());
                    self.matcher = TokenMatcher::Reset;
                    return Ok(token);
                } else {
                    let mut matcher_found = false;
                    for matcher in TokenMatcher::all() {
                        if matcher.check("", 0, char) {
                            self.matcher = matcher;
                            matcher_found = true;
                            break;
                        }
                    }
                    if !matcher_found {
                        return Err(TokenMatcherNotFound);
                    }
                    self.reset_buf();
                    self.inc_buf(char.len_utf8());
                }
            }
        }

        if self.matcher != TokenMatcher::Reset {
            let token = self.matcher.emit(self.buf());
            self.matcher = TokenMatcher::Reset;

            Ok(token)
        } else {
            Ok(TOKEN_EOF)
        }
    }
}
