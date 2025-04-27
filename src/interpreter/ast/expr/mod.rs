use crate::interpreter::ast::expr::assignment::Assign;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::variable::Variable;
use downcast_rs::{impl_downcast, Downcast};

pub mod assignment;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;
pub mod variable;

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, binary: &Binary<T>) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping<T>) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary<T>) -> T;
    fn visit_variable(&mut self, variable: &Variable) -> T;
    fn visit_assign(&mut self, assign: &Assign<T>) -> T;
}

pub trait Expr<T>: Downcast {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}

impl_downcast!(Expr<T>);
