use crate::b;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::Stmt;
use crate::interpreter::parser::error::{ParserError, ParserErrorType};
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::scanner::token::token_type::TokenType;
use crate::interpreter::scanner::token::Token;
use anyhow::{anyhow, Result};
use std::marker::PhantomData;

pub mod error;

pub struct Parser<T> {
    phantom_data: PhantomData<T>,
    tokens: Vec<Token>,
    current: usize,
}

impl<T> Parser<T>
where
    T: 'static,
{
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            phantom_data: Default::default(),
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Stmt<T>>>> {
        let mut statements = vec![];

        let mut error_stack = vec![];

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    error_stack.push(err);
                    self.synchronize()
                }
            }
        }

        if error_stack.is_empty() {
            return Ok(statements);
        }

        Err(anyhow!(
            "{}",
            error_stack
                .into_iter()
                .map(|err| err.to_string())
                .collect::<String>()
        ))
    }

    fn declaration(&mut self) -> Result<Box<dyn Stmt<T>>> {
        if self._match(vec![TokenType::Let]) {
            return self.let_declaration();
        }
        self.statement()
    }

    fn let_declaration(&self) -> Result<Box<dyn Stmt<T>>> {
        todo!()
    }

    fn statement(&mut self) -> Result<Box<dyn Stmt<T>>> {
        if self._match(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expr_statement()
    }

    fn print_statement(&mut self) -> Result<Box<dyn Stmt<T>>> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, ParserErrorType::ExpectedSemicolon)?;
        Ok(b!(Print::new(value)))
    }

    fn expr_statement(&mut self) -> Result<Box<dyn Stmt<T>>> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, ParserErrorType::ExpectedSemicolon)?;
        Ok(b!(StmtExpr::new(expr)))
    }

    pub fn expression(&mut self) -> Result<Box<dyn Expr<T>>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.comparison()?;

        while self._match(vec![TokenType::BangEqual, TokenType::Equal]) {
            let token = self.previous();
            let right = self.comparison()?;
            expr = b!(Binary::new(expr, token, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.term()?;

        while self._match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let token = self.previous();
            let right = self.term()?;
            expr = b!(Binary::new(expr, token, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.factor()?;

        while self._match(vec![TokenType::Minus, TokenType::Plus]) {
            let token = self.previous();
            let right = self.factor()?;
            expr = b!(Binary::new(expr, token, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.unary()?;

        while self._match(vec![TokenType::Star, TokenType::Slash]) {
            let token = self.previous();
            let right = self.unary()?;
            expr = b!(Binary::new(expr, token, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<dyn Expr<T>>> {
        if self._match(vec![TokenType::Bang, TokenType::Minus]) {
            let token = self.previous();
            let right = self.primary()?;
            return Ok(b!(Unary::new(token, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Expr<T>>> {
        if self._match(vec![TokenType::False]) {
            return Ok(b!(Literal::new(Some(Object::Bool(false)))));
        }
        if self._match(vec![TokenType::True]) {
            return Ok(b!(Literal::new(Some(Object::Bool(true)))));
        }
        if self._match(vec![TokenType::Nil]) {
            return Ok(b!(Literal::new(Some(Object::Nil))));
        }

        if self._match(vec![TokenType::Number, TokenType::String]) {
            return Ok(b!(Literal::new(self.previous().get_lit())));
        }

        if self._match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                ParserErrorType::ExpectedMatchingParens,
            )?;
            return Ok(b!(Grouping::new(expr)));
        }

        Err(anyhow!(
            self.error(self.peek(), ParserErrorType::ExpectedExpression)
        ))
    }

    fn _match(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ty: TokenType, error_ty: ParserErrorType) -> Result<Token> {
        if self.check(ty) {
            return Ok(self.advance());
        }
        Err(anyhow!(self.error(self.peek(), error_ty)))
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().get_type().eq(&ty)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().get_type().eq(&TokenType::Eof)
    }

    fn error(&self, token: Token, error_ty: ParserErrorType) -> ParserError {
        ParserError::new(token, error_ty)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().get_type().eq(&TokenType::Semicolon) {
                return;
            }

            match self.peek().get_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
