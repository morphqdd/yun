use crate::interpreter::ast::expr::node::Expr;
use crate::interpreter::ast::expr::ExprVisitor;

pub struct Grouping {
    expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_grouping(self)
    }
}
