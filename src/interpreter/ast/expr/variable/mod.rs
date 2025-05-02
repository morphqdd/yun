use std::sync::atomic::Ordering;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::Token;
use crate::utils::NEXT_ID;

#[derive(Debug, Clone)]
pub struct Variable {
    id: u64,
    token: Token,
}

impl Variable {
    pub fn new(token: Token) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), token }
    }

    #[inline]
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }
}

impl<T> Expr<T> for Variable {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_variable(self)
    }

    fn id(&self) -> u64 {
        self.id
    }
}
