use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use std::ops::Deref;

pub struct If<T> {
    cond: Box<dyn Expr<T>>,
    then_stmt: Box<dyn Stmt<T>>,
    else_stmt: Option<Box<dyn Stmt<T>>>,
}

impl<T> If<T> {
    pub fn new(
        cond: Box<dyn Expr<T>>,
        then_stmt: Box<dyn Stmt<T>>,
        else_stmt: Option<Box<dyn Stmt<T>>>,
    ) -> Self {
        Self {
            cond,
            then_stmt,
            else_stmt,
        }
    }

    pub fn get_cond(&self) -> &dyn Expr<T> {
        self.cond.deref()
    }

    pub fn get_then_stmt(&self) -> &dyn Stmt<T> {
        self.then_stmt.deref()
    }

    pub fn get_else_stmt(&self) -> Option<&dyn Stmt<T>> {
        self.else_stmt.as_deref()
    }
}

impl<T> Stmt<T> for If<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_if(self)
    }
}
