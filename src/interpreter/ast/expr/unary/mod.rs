use crate::interpreter::ast::expr::node::Expr;
use crate::interpreter::ast::expr::ExprVisitor;
use crate::interpreter::scanner::token::Token;

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_unary(self);
    }
}
