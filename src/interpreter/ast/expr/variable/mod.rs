use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::Token;

#[derive(Debug, Clone)]
pub struct Variable {
    token: Token,
}

impl Variable {
    pub fn new(token: Token) -> Self {
        Self { token }
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
}
