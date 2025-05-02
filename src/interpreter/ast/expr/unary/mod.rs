use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::token_type::TokenType;
use crate::interpreter::scanner::token::Token;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use crate::utils::NEXT_ID;

#[derive(Clone)]
pub struct Unary<T: 'static> {
    id: u64,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T> Unary<T> {
    #[inline]
    pub fn new(operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), operator, right }
    }

    #[inline]
    pub fn get_op_lexeme(&self) -> &str {
        self.operator.get_lexeme()
    }

    #[inline]
    pub fn get_right(&self) -> &dyn Expr<T> {
        self.right.deref()
    }

    #[inline]
    pub fn get_op_type(&self) -> TokenType {
        self.operator.get_type()
    }

    #[inline]
    pub fn get_token(&self) -> Token {
        self.operator.clone()
    }
}

impl<T: 'static + Clone> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_unary(self)
    }

    fn id(&self) -> u64 {
        self.id
    }
}
