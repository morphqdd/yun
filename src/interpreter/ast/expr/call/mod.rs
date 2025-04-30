use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::Token;
use std::ops::Deref;

pub struct Call<T> {
    callable: Box<dyn Expr<T>>,
    parens: Token,
    args: Vec<Box<dyn Expr<T>>>,
}

impl<T> Call<T> {
    pub fn new(callable: Box<dyn Expr<T>>, parens: Token, args: Vec<Box<dyn Expr<T>>>) -> Self {
        Self {
            callable,
            parens,
            args,
        }
    }

    pub fn get_callable(&self) -> &dyn Expr<T> {
        self.callable.deref()
    }

    pub fn get_args(&self) -> Vec<&dyn Expr<T>> {
        self.args.iter().map(|arg| arg.deref()).collect()
    }

    pub fn get_token(&self) -> Token {
        self.parens.clone()
    }
}

impl<T: 'static> Expr<T> for Call<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_call(self)
    }
}
