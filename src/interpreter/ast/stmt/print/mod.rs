use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use std::ops::Deref;

pub struct Print<T> {
    expr: Box<dyn Expr<T>>,
}

impl<T> Print<T> {
    pub fn new(expr: Box<dyn Expr<T>>) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &dyn Expr<T> {
        self.expr.deref()
    }
}

impl<T> Stmt<T> for Print<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_print(self)
    }
}
