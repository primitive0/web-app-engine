pub mod ast;
use ast::stmt;

use crate::lexer::{Lexer, Token, TokenKind, TOKEN_EOF};

pub struct Parser<'c> {
    lexer: Lexer<'c>,
    buf: Option<Token<'c>>,
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

// TODO: refactor logic (parsing around types, not validation)
impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Parser<'c> {
        Parser { lexer, buf: None }
    }

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

    fn next_token(&mut self) -> Token<'c> {
        let token = self.current_token();
        self.go_next();
        token
    }

    fn solid_token(&mut self) -> Token<'c> {
        loop {
            let token = self.current_token();
            if matches!(token.kind(), TokenKind::Spaces | TokenKind::LineBreak) {
                self.go_next();
                continue;
            }
            return token;
        }
    }

    fn next_solid_token(&mut self) -> Token<'c> {
        let token = self.solid_token();
        self.go_next();
        token
    }

    fn except_type(&mut self) -> Result<ast::Type<'c>, ParsingError> {
        token_to_type(self.next_solid_token()).ok_or(ParsingError)
    }

    fn except_type_or_void(&mut self) -> Result<ast::TypeOrVoid<'c>, ParsingError> {
        let token = self.next_solid_token();
        match token_to_type(token) {
            Some(t) => Ok(ast::TypeOrVoid::Type(t)),
            None => match token.kind() {
                TokenKind::KeywordVoid => Ok(ast::TypeOrVoid::Void),
                _ => Err(ParsingError),
            },
        }
    }

    fn except_ident(&mut self) -> Result<ast::Ident<'c>, ParsingError> {
        let token = self.next_solid_token();
        match token.kind() {
            TokenKind::Ident => Ok(ast::Ident { name: token.string() }),
            _ => Err(ParsingError),
        }
    }

    fn except_token(&mut self, kind: TokenKind) -> Result<(), ParsingError> {
        let token = self.next_solid_token();
        if kind == token.kind() {
            Ok(())
        } else {
            Err(ParsingError)
        }
    }

    fn except_literal(&mut self) -> Result<ast::Literal<'c>, ParsingError> {
        let token = self.next_solid_token();
        match token.kind() {
            TokenKind::IntegerLiteral => {
                let value = token.string().parse::<i32>().expect("failed to parse literal");
                Ok(ast::Literal::integer(value))
            }
            TokenKind::StringLiteral => {
                let string = token.string();
                let value = string.get(1..(string.len() - 1)).ok_or(ParsingError)?;
                Ok(ast::Literal::string(value))
            }
            _ => Err(ParsingError),
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

    fn parse_variable(&mut self) -> Result<stmt::VariableDeclaration<'c>, ParsingError> {
        let variable_type = self.except_type()?;
        let variable_name = self.except_ident()?;
        self.except_token(TokenKind::Assign)?;
        let literal = self.except_literal()?;
        self.except_end()?;
        Ok(stmt::VariableDeclaration::new(variable_type, variable_name, literal))
    }

    fn parse_function_call(&mut self) -> Result<stmt::FunctionCall<'c>, ParsingError> {
        let function_name = self.except_ident()?;

        self.except_token(TokenKind::ParenOpen)?;
        let mut args = Vec::<ast::Literal>::new();
        let mut first = true;
        loop {
            match self.solid_token().kind() {
                TokenKind::ParenClose => { // )
                    self.go_next();
                    break;
                }
                TokenKind::Sep if !first => self.go_next(), // ...,
                _ if first => {} // ...
                _ => return Err(ParsingError), // ... ...
            }
            let literal = self.except_literal()?;
            args.push(literal);
        }

        self.except_end()?;

        Ok(stmt::FunctionCall::new(function_name, args))
    }

    fn parse_statement(&mut self) -> Result<stmt::Statement<'c>, ParsingError> {
        let stmt = match self.solid_token().kind() {
            TokenKind::Ident => stmt::Statement::function_call(self.parse_function_call()?),
            _ => stmt::Statement::var_decl(self.parse_variable()?),
        };
        Ok(stmt)
    }

    fn parse_function(&mut self) -> Result<ast::FunctionDeclaration<'c>, ParsingError> {
        let return_type = self.except_type_or_void()?;
        let function_name = self.except_ident()?;

        // parse args
        self.except_token(TokenKind::ParenOpen)?;
        let mut args = Vec::<ast::FunctionArg>::new();
        let mut first = true;
        loop {
            match self.solid_token().kind() {
                TokenKind::ParenClose => {
                    self.go_next();
                    break;
                }
                TokenKind::Sep if !first => self.go_next(),
                _ if first => {}
                _ => return Err(ParsingError),
            }
            let arg_type = self.except_type()?;
            let arg_name = self.except_ident()?;
            let arg = ast::FunctionArg::new(arg_type, arg_name);
            args.push(arg);
            first = false;
        }

        // parse body
        self.except_token(TokenKind::BraceOpen)?;
        let mut stmts = Vec::new();
        loop {
            let token = self.solid_token();
            if token.kind() == TokenKind::BraceClose {
                self.go_next();
                break;
            }
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }

        self.except_end()?;

        return Ok(ast::FunctionDeclaration::new(return_type, function_name, args, stmts));
    }

    pub fn parse(&mut self) -> Result<ast::AST<'c>, ParsingError> {
        let mut declarations = Vec::new();
        loop {
            let token = self.solid_token();
            if token == TOKEN_EOF {
                return Ok(ast::AST::new(declarations));
            }
            let declaration = self.parse_function()?;
            declarations.push(declaration);
        }
    }
}

pub fn build_ast(lexer: Lexer) -> Result<ast::AST, ParsingError> {
    Parser::new(lexer).parse()
}
