use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use std::ops::Deref;

pub struct Let<T> {
    ident: String,
    initializer: Box<dyn Expr<T>>,
}

impl<T> Let<T> {
    pub fn new(ident: &str, initializer: Box<dyn Expr<T>>) -> Self {
        Self {
            ident: ident.into(),
            initializer,
        }
    }

    pub fn get_ident(&self) -> String {
        self.ident.clone()
    }

    pub fn get_initializer(&self) -> &dyn Expr<T> {
        self.initializer.deref()
    }
}

impl<T> Stmt<T> for Let<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_let(self)
    }
}
