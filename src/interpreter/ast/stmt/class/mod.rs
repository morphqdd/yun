use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::scanner::token::Token;

#[derive(Clone)]
pub struct Class<T: 'static> {
    name: Token,
    methods: Vec<Box<Fun<T>>>,
}

impl<T> Class<T> {
    pub fn new(name: Token, methods: Vec<Box<Fun<T>>>) -> Self {
        Self { name, methods }
    }

    pub fn extract(&self) -> (&Token, &Vec<Box<Fun<T>>>) {
        (&self.name, &self.methods)
    }
}

impl<T: 'static + Clone> Stmt<T> for Class<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_class(&self)
    }
}
