use super::{
    ast,
    error::{ParsingError, Result},
    seq::TokenSeq,
};
use crate::lexer::{Lexer, TokenKind, TOKEN_EOF};
use ast::stmt;

pub struct Parser<'c> {
    seq: TokenSeq<'c>,
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Parser<'c> {
        Parser {
            seq: TokenSeq::new(lexer),
        }
    }

    fn parse_variable(&mut self) -> Result<stmt::VariableDeclaration<'c>> {
        let variable_type = self.seq.expect_type()?;
        let variable_name = self.seq.expect_ident()?;
        self.seq.expect_token(TokenKind::Assign)?;
        let literal = self.seq.expect_literal()?;
        self.seq.expect_end()?;
        Ok(stmt::VariableDeclaration::new(variable_type, variable_name, literal))
    }

    fn parse_function_call(&mut self) -> Result<stmt::FunctionCall<'c>> {
        let function_name = self.seq.expect_ident()?;

        self.seq.expect_token(TokenKind::ParenOpen)?;
        let mut args = Vec::new();
        let mut first = true;
        loop {
            let kind = self.seq.solid_token().kind();
            match kind {
                // )
                TokenKind::ParenClose => {
                    self.seq.go_next();
                    break;
                }
                // ...,
                TokenKind::Sep if !first => self.seq.go_next(),
                // (...
                // TokenKind::Sep if first => Err()
                _ if first => {}
                // ... ...
                _ => return Err(ParsingError::new(kind, [TokenKind::Sep, TokenKind::ParenClose])),
            }
            let literal = self.seq.expect_literal()?;
            args.push(literal);
        }

        self.seq.expect_end()?;

        Ok(stmt::FunctionCall::new(function_name, args))
    }

    fn parse_statement(&mut self) -> Result<stmt::Statement<'c>> {
        let stmt = match self.seq.solid_token().kind() {
            TokenKind::Ident => stmt::Statement::function_call(self.parse_function_call()?),
            _ => stmt::Statement::var_decl(self.parse_variable()?),
        };
        Ok(stmt)
    }

    fn parse_function(&mut self) -> Result<ast::FunctionDeclaration<'c>> {
        let return_type = self.seq.expect_type_or_void()?;
        let function_name = self.seq.expect_ident()?;

        // parse args
        self.seq.expect_token(TokenKind::ParenOpen)?;
        let mut args = Vec::<ast::FunctionArg>::new();
        let mut first = true;
        loop {
            let kind = self.seq.solid_token().kind();
            match kind {
                TokenKind::ParenClose => {
                    self.seq.go_next();
                    break;
                }
                TokenKind::Sep if !first => self.seq.go_next(),
                _ if first => {}
                _ => return Err(ParsingError::new(kind, [TokenKind::Sep])),
            }
            let arg_type = self.seq.expect_type()?;
            let arg_name = self.seq.expect_ident()?;
            let arg = ast::FunctionArg::new(arg_type, arg_name);
            args.push(arg);
            first = false;
        }

        // parse body
        self.seq.expect_token(TokenKind::BraceOpen)?;
        let mut stmts = Vec::new();
        loop {
            let token = self.seq.solid_token();
            if token.kind() == TokenKind::BraceClose {
                self.seq.go_next();
                break;
            }
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }

        self.seq.expect_end()?;

        return Ok(ast::FunctionDeclaration::new(return_type, function_name, args, stmts));
    }

    pub fn parse(&mut self) -> Result<ast::AST<'c>> {
        let mut declarations = Vec::new();
        loop {
            let token = self.seq.solid_token();
            if token == TOKEN_EOF {
                return Ok(ast::AST::new(declarations));
            }
            let declaration = self.parse_function()?;
            declarations.push(declaration);
        }
    }
}
