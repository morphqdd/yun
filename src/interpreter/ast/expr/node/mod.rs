use crate::interpreter::ast::expr::ExprVisitor;

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor);
}
