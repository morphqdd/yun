use crate::interpreter::ast::expr::node::Expr;
use crate::interpreter::ast::expr::ExprVisitor;
use crate::interpreter::scanner::token::literal::Object;

pub struct Literal {
    value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_literal(self);
    }
}
