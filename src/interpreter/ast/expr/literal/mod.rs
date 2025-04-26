use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::scanner::token::literal::Object;

pub struct Literal {
    value: Option<Object>,
}

impl Literal {
    #[inline]
    pub fn new(value: Option<Object>) -> Self {
        Self { value }
    }

    #[inline]
    pub fn get_value(&self) -> Option<&Object> {
        self.value.as_ref()
    }
}

impl<T> Expr<T> for Literal {
    #[inline]
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match &self.value {
            None => "nil".to_string(),
            Some(lit) => lit.to_string(),
        }
    }
}
