use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};

type ExtractedIf<T> = (Box<dyn Expr<T>>, Box<dyn Stmt<T>>, Option<Box<dyn Stmt<T>>>);

#[derive(Clone)]
pub struct If<T: 'static> {
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

    pub fn extract(self) -> ExtractedIf<T> {
        (self.cond, self.then_stmt, self.else_stmt)
    }
}

impl<T: 'static + Clone> Stmt<T> for If<T> {
    fn accept(self: Box<If<T>>, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_if(self)
    }
}
