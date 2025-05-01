use crate::interpreter::ast::stmt::block::Block;
use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::ast::stmt::if_stmt::If;
use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::while_stmt::While;

pub mod block;
pub mod fun_stmt;
pub mod if_stmt;
pub mod let_stmt;
pub mod print;
pub mod stmt_expr;
pub mod while_stmt;

pub trait StmtVisitor<T> {
    fn visit_expr(&mut self, stmt: Box<StmtExpr<T>>) -> T;
    fn visit_print(&mut self, stmt: Box<Print<T>>) -> T;
    fn visit_let(&mut self, stmt: Box<Let<T>>) -> T;
    fn visit_block(&mut self, stmt: Box<Block<T>>) -> T;
    fn visit_if(&mut self, stmt: Box<If<T>>) -> T;
    fn visit_while(&mut self, stmt: Box<While<T>>) -> T;
    fn visit_fun(&mut self, stmt: Box<Fun<T>>) -> T;
}

pub trait CloneStmt<T> {
    fn clone_box(&self) -> Box<dyn Stmt<T>>;
}

pub trait Stmt<T>: CloneStmt<T> {
    fn accept(self: Box<Self>, visitor: &mut dyn StmtVisitor<T>) -> T;
}

impl<T, R> CloneStmt<T> for R
where
    R: 'static + Stmt<T> + Clone,
    T: 'static + Clone,
{
    fn clone_box(&self) -> Box<dyn Stmt<T>> {
        Box::new(self.clone())
    }
}

impl<T> Clone for Box<dyn Stmt<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
