use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::Token;
use std::ops::Deref;

#[derive(Clone)]
pub struct Logical<T: 'static> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T> Logical<T> {
    pub fn new(left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn get_left(&self) -> &dyn Expr<T> {
        self.left.deref()
    }

    pub fn get_operator(&self) -> Token {
        self.operator.clone()
    }

    pub fn get_right(&self) -> &dyn Expr<T> {
        self.right.deref()
    }
}

impl<T: 'static + Clone> Expr<T> for Logical<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_logical(self)
    }
}
