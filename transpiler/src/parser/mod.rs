pub mod ast;

use crate::lexer::{Lexer, Token, TokenKind, TOKEN_EOF};

pub struct Parser<'c> {
    lexer: Lexer<'c>,
    buf: Option<Token<'c>>,
    // token_buf: Vec<Token<'c>>,
    // pos: usize,
}

#[derive(Debug)]
pub struct ParsingError;

fn token_to_type(token: Token) -> Option<ast::Type> {
    match token.kind() {
        TokenKind::KeywordInt => Some(ast::Type::Int),
        TokenKind::Ident => Some(ast::Type::Custom { name: token.string() }),
        _ => None,
    }
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Parser<'c> {
        Parser {
            lexer,
            buf: None,
            // token_buf: Vec::new(),
            // pos: 0,
        }
    }

    // fn current_token(&mut self) -> Token<'c> {
    //     if self.pos == self.token_buf.len() {
    //         let token = self.lexer.next().expect("failed to match token");
    //         self.token_buf.push(token);
    //         token
    //     } else {
    //         self.token_buf[self.pos]
    //     }
    // }
    //

    fn current_token(&mut self) -> Token<'c> {
        match self.buf {
            Some(t) => t,
            None => {
                let next = self.lexer.next().expect("failed to match token");
                self.buf = Some(next);
                next
            }
        }
    }

    fn go_next(&mut self) {
        self.buf = None;
    }

    fn next_nonsep_token(&mut self) -> Token<'c> {
        loop {
            let token = self.current_token();
            if !matches!(token.kind(), TokenKind::Spaces | TokenKind::LineBreak) {
                self.go_next();
                return token;
            }
            self.go_next();
        }
    }

    fn except_type(&mut self) -> Result<ast::Type<'c>, ParsingError> {
        token_to_type(self.next_nonsep_token()).ok_or(ParsingError)
    }

    fn except_type_or_void(&mut self) -> Result<ast::TypeOrVoid<'c>, ParsingError> {
        let token = self.next_nonsep_token();
        match token_to_type(token) {
            Some(t) => Ok(ast::TypeOrVoid::Type(t)),
            None => match token.kind() {
                TokenKind::KeywordVoid => Ok(ast::TypeOrVoid::Void),
                _ => Err(ParsingError),
            },
        }
    }

    fn except_ident(&mut self) -> Result<ast::Ident<'c>, ParsingError> {
        let token = self.next_nonsep_token();
        match token.kind() {
            TokenKind::Ident => Ok(ast::Ident { name: token.string() }),
            _ => Err(ParsingError),
        }
    }

    fn except_token(&mut self, kind: TokenKind) -> Result<(), ParsingError> {
        let token = self.next_nonsep_token();
        if kind == token.kind() {
            Ok(())
        } else {
            Err(ParsingError)
        }
    }

    fn except_integer_literal(&mut self) -> Result<ast::IntegerLiteral, ParsingError> {
        let token = self.next_nonsep_token();
        if token.kind() == TokenKind::LiteralInteger {
            let value = token.string().parse::<i32>().expect("failed to parse literal");
            Ok(ast::IntegerLiteral::new(value))
        } else {
            Err(ParsingError)
        }
    }

    fn except_end(&mut self) -> Result<(), ParsingError> {
        let token = self.current_token();
        if matches!(token.kind(), TokenKind::LineBreak | TokenKind::Eof) {
            self.go_next();
            Ok(())
        } else {
            Err(ParsingError)
        }
    }

    // fn parse_function(&mut self) -> Result<(), ParsingError> {
    //     let return_type = self.except_type_or_void().ok_or(ParsingError::NoMatch)?;
    //     let function_name = self.except_ident().ok_or(ParsingError::NoMatch)?;
    //     self.except_token(TokenKind::ParenOpen).ok_or(ParsingError::BadConstruction)?;
    //
    //     return Ok(());
    // }

    fn parse_variable(&mut self) -> Result<ast::VariableDeclaration<'c>, ParsingError> {
        let variable_type = self.except_type()?;
        let variable_name = self.except_ident()?;
        self.except_token(TokenKind::Assign)?;
        let int_literal = self.except_integer_literal()?;
        self.except_end()?;
        Ok(ast::VariableDeclaration::new(variable_type, variable_name, int_literal))
    }

    pub fn parse(&mut self) -> Result<ast::AST<'c>, ParsingError> {
        let mut declarations = Vec::new();
        loop {
            let token = self.current_token();
            if token == TOKEN_EOF {
                return Ok(ast::AST::new(declarations));
            }
            let declaration = self.parse_variable()?;
            declarations.push(declaration);
        }
    }
}

pub fn build_ast(lexer: Lexer) -> Result<ast::AST, ParsingError> {
    Parser::new(lexer).parse()
}
