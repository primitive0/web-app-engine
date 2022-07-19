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

    fn parse_until<N, F>(&mut self, needle: TokenKind, parse: F) -> Result<Vec<N>>
    where
        F: Fn(&mut Parser<'c>) -> Result<N>,
    {
        let mut result = Vec::<N>::new();
        loop {
            let token = self.seq.solid_token();
            if token.kind() == needle {
                self.seq.go_next();
                break;
            }
            let node = parse(self)?;
            result.push(node);
        }
        Ok(result)
    }

    fn parse_list<N, F>(&mut self, needle: TokenKind, parse: F) -> Result<Vec<N>>
    where
        F: Fn(&mut Parser<'c>) -> Result<N>,
    {
        debug_assert!(needle != TokenKind::Sep);

        let mut result = Vec::<N>::new();
        let mut first = true;
        loop {
            let kind = self.seq.solid_token().kind();
            if kind == needle {
                self.seq.go_next();
                break;
            }
            if !first {
                if kind == TokenKind::Sep {
                    self.seq.go_next();
                } else {
                    return Err(ParsingError::new(kind, [TokenKind::Sep, needle]));
                }
            }
            let node = parse(self)?;
            result.push(node);
            first = false;
        }
        Ok(result)
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
        let args = self.parse_list(TokenKind::ParenClose, |parser| parser.seq.expect_literal())?;
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
        let args = self.parse_list(TokenKind::ParenClose, |parser| {
            let arg_type = parser.seq.expect_type()?;
            let arg_name = parser.seq.expect_ident()?;
            Ok(ast::FunctionArg::new(arg_type, arg_name))
        })?;
        // parse body
        self.seq.expect_token(TokenKind::BraceOpen)?;
        let stmts = self.parse_until(TokenKind::BraceClose, |parser| parser.parse_statement())?;

        self.seq.expect_end()?;

        return Ok(ast::FunctionDeclaration::new(return_type, function_name, args, stmts));
    }
}
