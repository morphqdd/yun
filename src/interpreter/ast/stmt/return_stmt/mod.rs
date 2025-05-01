use crate::interpreter::ast::expr::Expr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::error::InterpreterError;
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::scanner::token::Token;
use crate::interpreter::error::Result;

#[derive(Clone)]
pub struct Return<T: 'static> {
    token: Token,
    expr: Option<Box<dyn Expr<T>>>,
}

impl<T> Return<T> {
    pub fn new(token: Token, expr: Option<Box<dyn Expr<T>>>) -> Self {
        Self { token, expr }
    }

    pub fn extract(self) -> (Token, Option<Box<dyn Expr<T>>>) {
        (self.token, self.expr)
    }
}

impl<T: 'static + Clone> Stmt<T> for Return<T> {
    fn accept(self: Box<Self>, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_return(self)
    }
}