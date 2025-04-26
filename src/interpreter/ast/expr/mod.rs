use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::unary::Unary;

pub mod binary;
pub mod grouping;
pub mod literal;
pub mod node;
pub mod unary;

pub trait ExprVisitor {
    fn visit_binary(&mut self, binary: &Binary);
    fn visit_grouping(&mut self, grouping: &Grouping);
    fn visit_literal(&mut self, literal: &Literal);
    fn visit_unary(&mut self, unary: &Unary);
}
