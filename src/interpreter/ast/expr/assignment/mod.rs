use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::Token;
use std::ops::Deref;

#[derive(Clone)]
pub struct Assign<T: 'static> {
    token: Token,
    value: Box<dyn Expr<T>>,
}

impl<T> Assign<T> {
    pub fn new(token: Token, value: Box<dyn Expr<T>>) -> Self {
        Self { token, value }
    }

    pub fn get_token(&self) -> Token {
        self.token.clone()
    }

    pub fn get_value(&self) -> &dyn Expr<T> {
        self.value.deref()
    }
}

impl<T: 'static + Clone> Expr<T> for Assign<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_assign(self)
    }
}
