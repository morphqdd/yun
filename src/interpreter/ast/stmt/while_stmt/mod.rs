use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};

#[derive(Clone)]
pub struct While<T: 'static> {
    cond: Box<dyn Expr<T>>,
    stmt: Box<dyn Stmt<T>>,
}

impl<T> While<T> {
    pub fn new(cond: Box<dyn Expr<T>>, stmt: Box<dyn Stmt<T>>) -> Self {
        Self { cond, stmt }
    }

    pub fn extract(self) -> (Box<dyn Expr<T>>, Box<dyn Stmt<T>>) {
        (self.cond, self.stmt)
    }
}

impl<T: 'static + Clone> Stmt<T> for While<T> {
    fn accept(self: Box<While<T>>, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_while(self)
    }
}
