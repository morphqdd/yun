use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::unary::Unary;

pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, binary: &Binary<T>) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping<T>) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary<T>) -> T;
}

pub trait Expr<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}
