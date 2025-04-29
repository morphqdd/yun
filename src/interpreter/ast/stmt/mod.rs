use crate::interpreter::ast::stmt::block::Block;
use crate::interpreter::ast::stmt::if_stmt::If;
use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::while_stmt::While;

pub mod block;
pub mod if_stmt;
pub mod let_stmt;
pub mod print;
pub mod stmt_expr;
pub mod while_stmt;

pub trait StmtVisitor<T> {
    fn visit_expr(&mut self, stmt: &StmtExpr<T>) -> T;
    fn visit_print(&mut self, stmt: &Print<T>) -> T;
    fn visit_let(&mut self, stmt: &Let<T>) -> T;
    fn visit_block(&mut self, stmt: &Block<T>) -> T;
    fn visit_if(&mut self, stmt: &If<T>) -> T;
    fn visit_while(&mut self, stmt: &While<T>) -> T;
}

pub trait Stmt<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T;
}
