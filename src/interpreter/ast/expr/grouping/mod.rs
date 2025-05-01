use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use std::ops::Deref;

#[derive(Clone)]
pub struct Grouping<T: 'static> {
    expression: Box<dyn Expr<T>>,
}

impl<T> Grouping<T> {
    #[inline]
    pub fn new(expression: Box<dyn Expr<T>>) -> Self {
        Self { expression }
    }

    #[inline]
    pub fn get_expr(&self) -> &dyn Expr<T> {
        self.expression.deref()
    }
}

impl<T: 'static + Clone> Expr<T> for Grouping<T> {
    #[inline]
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_grouping(self)
    }
}
