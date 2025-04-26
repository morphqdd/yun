use crate::interpreter::ast::expr::node::Expr;
use crate::interpreter::ast::expr::ExprVisitor;
use crate::interpreter::scanner::token::Token;

pub struct Binary {
    left: Box<dyn Expr>,
    operation: Token,
    right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operation: Token, right: Box<dyn Expr>) -> Binary {
        Binary {
            left,
            operation,
            right,
        }
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_binary(self);
    }
}
