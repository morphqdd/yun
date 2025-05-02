use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::scanner::token::Token;

#[derive(Clone)]
pub struct Fun<T: 'static> {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<dyn Stmt<T>>>,
}

impl<T> Fun<T> {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Box<dyn Stmt<T>>>) -> Self {
        Self { name, params, body }
    }

    pub fn extract(self) -> (Token, Vec<Token>, Vec<Box<dyn Stmt<T>>>) {
        (self.name, self.params, self.body)
    }

    pub fn get_name(&self) -> Token {
        self.name.clone()
    }
}

impl<T: 'static + Clone> Stmt<T> for Fun<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_fun(self)
    }
}
