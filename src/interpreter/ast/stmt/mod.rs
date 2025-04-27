use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;

pub mod let_stmt;
pub mod print;
pub mod stmt_expr;

pub trait StmtVisitor<T> {
    fn visit_expr(&mut self, stmt: &StmtExpr<T>) -> T;
    fn visit_print(&mut self, stmt: &Print<T>) -> T;
    fn visit_let(&mut self, stmt: &Let<T>) -> T;
}

pub trait Stmt<T> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<T>) -> T;
}
