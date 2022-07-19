use super::{
    ast,
    error::{ParsingError, Result},
};
use crate::lexer::{Lexer, Token, TokenKind};
use std::iter;

macro_rules! match_token {
    ( ($expr:expr) { $( $variant:path => $branch:expr),* $(,)? } ) => {
        {
            let value = ($expr);
            match value {
                $( $variant => Ok(($branch)), )*
                _ => Err(ParsingError::new(value, [ $($variant),* ])),
            }
        }
    };
}

const TYPE_TOKENS_EXPECTED: [TokenKind; 2] = [TokenKind::KeywordInt, TokenKind::Ident];

fn token_to_type(token: Token) -> Option<ast::Type> {
    match token.kind() {
        TokenKind::KeywordInt => Some(ast::Type::Int),
        TokenKind::Ident => Some(ast::Type::Custom { name: token.string() }),
        _ => None,
    }
}

pub struct TokenSeq<'c> {
    lexer: Lexer<'c>,
    buf: Option<Token<'c>>,
}

impl<'c> TokenSeq<'c> {
    pub fn new(lexer: Lexer<'c>) -> TokenSeq<'c> {
        TokenSeq { lexer, buf: None }
    }

    pub fn current_token(&mut self) -> Token<'c> {
        match self.buf {
            Some(t) => t,
            None => {
                let next = self.lexer.next().expect("failed to match token");
                self.buf = Some(next);
                next
            }
        }
    }

    pub fn go_next(&mut self) {
        self.buf = None;
    }

    pub fn next_token(&mut self) -> Token<'c> {
        let token = self.current_token();
        self.go_next();
        token
    }

    pub fn solid_token(&mut self) -> Token<'c> {
        loop {
            let token = self.current_token();
            if matches!(token.kind(), TokenKind::Spaces | TokenKind::LineBreak) {
                self.go_next();
                continue;
            }
            return token;
        }
    }

    pub fn next_solid_token(&mut self) -> Token<'c> {
        let token = self.solid_token();
        self.go_next();
        token
    }

    pub fn expect_type(&mut self) -> Result<ast::Type<'c>> {
        let token = self.next_solid_token();
        token_to_type(token).ok_or_else(|| ParsingError::new(token.kind(), TYPE_TOKENS_EXPECTED))
    }

    pub fn expect_type_or_void(&mut self) -> Result<ast::TypeOrVoid<'c>> {
        let token = self.next_solid_token();
        match token_to_type(token) {
            Some(t) => Ok(ast::TypeOrVoid::Type(t)),
            None => match token.kind() {
                TokenKind::KeywordVoid => Ok(ast::TypeOrVoid::Void),
                _ => {
                    let expected = TYPE_TOKENS_EXPECTED.into_iter().chain(iter::once(TokenKind::KeywordVoid));
                    Err(ParsingError::new(token.kind(), expected))
                }
            },
        }
    }

    pub fn expect_ident(&mut self) -> Result<ast::Ident<'c>> {
        let token = self.next_solid_token();
        match_token!((token.kind()) {
            TokenKind::Ident => ast::Ident { name: token.string() },
        })
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Result<()> {
        let token = self.next_solid_token();
        if kind == token.kind() {
            Ok(())
        } else {
            Err(ParsingError::new(token.kind(), [kind]))
        }
    }

    // TODO: change error handling here
    pub fn expect_literal(&mut self) -> Result<ast::Literal<'c>> {
        let token = self.next_solid_token();
        match_token!((token.kind()) {
            TokenKind::IntegerLiteral => {
                let value = token.string().parse::<i32>().expect("failed to parse integer literal");
                ast::Literal::integer(value)
            },
            TokenKind::StringLiteral => {
                let string = token.string();
                let value = string.get(1..(string.len() - 1)).expect("failed to parse string literal");
                ast::Literal::string(value)
            },
        })
    }

    pub fn expect_end(&mut self) -> Result<()> {
        match_token!((self.current_token().kind()) {
            TokenKind::LineBreak => (),
            TokenKind::Eof => (),
        })?;
        self.go_next();
        Ok(())
    }
}
