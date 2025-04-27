use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};

pub struct Block<T> {
    stmts: Vec<Box<dyn Stmt<T>>>,
}

impl<T> Block<T> {
    pub fn new(stmts: Vec<Box<dyn Stmt<T>>>) -> Self {
        Self { stmts }
    }

    pub fn get_stmts(&self) -> &[Box<dyn Stmt<T>>] {
        &self.stmts
    }
}

impl<T> Stmt<T> for Block<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_block(self)
    }
}
