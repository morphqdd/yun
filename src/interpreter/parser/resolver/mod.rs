use std::marker::PhantomData;
use crate::interpreter::ast::expr::assignment::Assign;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::call::Call;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::logical::Logical;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::variable::Variable;
use crate::interpreter::ast::stmt::block::Block;
use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::ast::stmt::if_stmt::If;
use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::return_stmt::Return;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::ast::stmt::while_stmt::While;
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::error::Result;
use crate::interpreter::Interpreter;

pub struct Resolver<T> {
    phantom: PhantomData<T>,
    interpreter: Interpreter,
}

impl<T: 'static> Resolver<T>
where Resolver<T>: ExprVisitor<T> + StmtVisitor<T> {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {phantom: Default::default(),  interpreter }
    }

    pub fn resolve(&mut self, stmts: Vec<Box<dyn Stmt<T>>>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut  self, stmt: Box<dyn Stmt<T>>) {
        stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: Box<dyn Expr<T>>) {
        expr.accept(self);
    }
}

impl ExprVisitor<Result<Object>> for Resolver<Result<Object>> {
    fn visit_binary(&mut self, binary: &Binary<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_grouping(&mut self, grouping: &Grouping<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<Object> {
        todo!()
    }

    fn visit_unary(&mut self, unary: &Unary<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_variable(&mut self, variable: &Variable) -> Result<Object> {
        todo!()
    }

    fn visit_assign(&mut self, assign: &Assign<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_logical(&mut self, logical: &Logical<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_call(&mut self, call: &Call<Result<Object>>) -> Result<Object> {
        todo!()
    }
}

impl StmtVisitor<Result<Object>> for Resolver<Result<Object>> {
    fn visit_expr(&mut self, stmt: &StmtExpr<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_print(&mut self, stmt: &Print<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_let(&mut self, stmt: &Let<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_block(&mut self, stmt: &Block<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_if(&mut self, stmt: &If<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_while(&mut self, stmt: &While<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_fun(&mut self, stmt: &Fun<Result<Object>>) -> Result<Object> {
        todo!()
    }

    fn visit_return(&mut self, stmt: &Return<Result<Object>>) -> Result<Object> {
        todo!()
    }
}