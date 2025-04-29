use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use std::ops::Deref;

pub struct While<T> {
    cond: Box<dyn Expr<T>>,
    stmt: Box<dyn Stmt<T>>,
}

impl<T> While<T> {
    pub fn new(cond: Box<dyn Expr<T>>, stmt: Box<dyn Stmt<T>>) -> Self {
        Self { cond, stmt }
    }

    pub fn get_cond(&self) -> &dyn Expr<T> {
        self.cond.deref()
    }

    pub fn get_stmt(&self) -> &dyn Stmt<T> {
        self.stmt.deref()
    }
}

impl<T> Stmt<T> for While<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_while(self)
    }
}
