use crate::interpreter::Interpreter;
use crate::interpreter::ast::expr::assignment::Assign;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::call::Call;
use crate::interpreter::ast::expr::get::Get;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::logical::Logical;
use crate::interpreter::ast::expr::set::Set;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::variable::Variable;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::ast::stmt::block::Block;
use crate::interpreter::ast::stmt::class::Class;
use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::ast::stmt::if_stmt::If;
use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::return_stmt::Return;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::while_stmt::While;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::error::Result;
use crate::interpreter::parser::error::{ParserError, ParserErrorType};
use crate::interpreter::scanner::token::Token;
use crate::interpreter::scanner::token::object::Object;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum FunctionType {
    Function,
    None,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    stack: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a>
where
    Resolver<'a>: ExprVisitor<Result<Object>> + StmtVisitor<Result<Object>>,
{
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            stack: vec![],
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, stmts: Vec<&dyn Stmt<Result<Object>>>) -> Result<()> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &dyn Stmt<Result<Object>>) -> Result<Object> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &dyn Expr<Result<Object>>) -> Result<Object> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.stack.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.stack.is_empty() {
            return;
        }

        let scope = self.stack.last_mut().unwrap();
        scope.insert(name.get_lexeme().to_string(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.stack.is_empty() {
            return;
        }

        let scope = self.stack.last_mut().unwrap();
        scope.insert(name.get_lexeme().to_string(), true);
    }

    fn resolve_local(&mut self, expr: &dyn Expr<Result<Object>>, name: &Token) {
        if self.stack.is_empty() {
            return;
        }

        for i in (0..=self.stack.len().saturating_sub(1)).rev() {
            if self.stack.get(i).unwrap().contains_key(name.get_lexeme()) {
                self.interpreter
                    .resolve(expr.clone_expr(), self.stack.len() - i - 1);
                return;
            }
        }
    }

    fn resolve_function(&mut self, func: &Fun<Result<Object>>, ty: FunctionType) -> Result<()> {
        let enclosing_func = self.current_function;
        self.current_function = ty;

        self.begin_scope();
        let (_, _, params, body) = func.clone().extract();
        for param in params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve(body.iter().map(AsRef::as_ref).collect::<Vec<_>>())?;
        self.end_scope();

        self.current_function = enclosing_func;
        Ok(())
    }
}

impl ExprVisitor<Result<Object>> for Resolver<'_> {
    fn visit_binary(&mut self, binary: &Binary<Result<Object>>) -> Result<Object> {
        self.resolve_expr(binary.get_left())?;
        self.resolve_expr(binary.get_right())?;
        Ok(Object::Nil)
    }

    fn visit_grouping(&mut self, grouping: &Grouping<Result<Object>>) -> Result<Object> {
        self.resolve_expr(grouping.get_expr())?;
        Ok(Object::Nil)
    }

    fn visit_literal(&mut self, _literal: &Literal) -> Result<Object> {
        Ok(Object::Nil)
    }

    fn visit_unary(&mut self, unary: &Unary<Result<Object>>) -> Result<Object> {
        self.resolve_expr(unary.get_right())?;
        Ok(Object::Nil)
    }

    fn visit_variable(&mut self, variable: &Variable) -> Result<Object> {
        let name = variable.get_token();

        if let Some(scope) = self.stack.last() {
            if let Some(value) = scope.get(name.get_lexeme()) {
                if !(*value) {
                    return Err(ParserError::new(
                        name,
                        ParserErrorType::CantReadLocalVariableInItsOwnInit,
                    )
                    .into());
                }
            }
        }

        self.resolve_local(variable, &name);
        Ok(Object::Nil)
    }

    fn visit_assign(&mut self, assign: &Assign<Result<Object>>) -> Result<Object> {
        self.resolve_expr(assign.get_value())?;
        self.resolve_local(assign, &assign.get_token());
        Ok(Object::Nil)
    }

    fn visit_logical(&mut self, logical: &Logical<Result<Object>>) -> Result<Object> {
        self.resolve_expr(logical.get_left())?;
        self.resolve_expr(logical.get_right())?;
        Ok(Object::Nil)
    }

    fn visit_call(&mut self, call: &Call<Result<Object>>) -> Result<Object> {
        self.resolve_expr(call.get_callable())?;
        for arg in call.get_args() {
            self.resolve_expr(arg)?;
        }
        Ok(Object::Nil)
    }

    fn visit_get(&mut self, get: &Get<Result<Object>>) -> Result<Object> {
        self.resolve_expr(get.extract().1)?;
        Ok(Object::Nil)
    }

    fn visit_set(&mut self, set: &Set<Result<Object>>) -> Result<Object> {
        let (_, obj, value) = set.extract();
        self.resolve_expr(obj)?;
        self.resolve_expr(value)?;
        Ok(Object::Nil)
    }
}

impl StmtVisitor<Result<Object>> for Resolver<'_> {
    fn visit_expr(&mut self, stmt: &StmtExpr<Result<Object>>) -> Result<Object> {
        self.resolve_expr(stmt.expr())?;
        Ok(Object::Nil)
    }

    fn visit_print(&mut self, stmt: &Print<Result<Object>>) -> Result<Object> {
        self.resolve_expr(stmt.expr())?;
        Ok(Object::Nil)
    }

    fn visit_let(&mut self, stmt: &Let<Result<Object>>) -> Result<Object> {
        let name = stmt.get_ident();
        self.declare(&name);
        if let Some(initializer) = stmt.get_initializer() {
            self.resolve_expr(initializer)?;
        }
        self.define(&name);
        Ok(Object::Nil)
    }

    fn visit_block(&mut self, stmt: &Block<Result<Object>>) -> Result<Object> {
        self.begin_scope();
        self.resolve(stmt.get_stmts())?;
        self.end_scope();
        Ok(Object::Nil)
    }

    fn visit_if(&mut self, stmt: &If<Result<Object>>) -> Result<Object> {
        let (cond, then, else_) = stmt.extract();
        self.resolve_expr(cond)?;
        self.resolve_stmt(then)?;
        if let Some(else_) = else_ {
            self.resolve_stmt(else_)?;
        }
        Ok(Object::Nil)
    }

    fn visit_while(&mut self, stmt: &While<Result<Object>>) -> Result<Object> {
        let (cond, body) = stmt.extract();
        self.resolve_expr(cond)?;
        self.resolve_stmt(body)?;
        Ok(Object::Nil)
    }

    fn visit_fun(&mut self, stmt: &Fun<Result<Object>>) -> Result<Object> {
        let name = stmt.get_name();
        self.declare(&name);
        self.define(&name);
        self.resolve_function(stmt, FunctionType::Function)?;
        Ok(Object::Nil)
    }

    fn visit_return(&mut self, stmt: &Return<Result<Object>>) -> Result<Object> {
        let (name, expr) = stmt.extract();

        if self.current_function == FunctionType::None {
            return Err(ParserError::new(
                name.clone(),
                ParserErrorType::CantReturnFromTopLevelCode,
            )
            .into());
        }

        if let Some(expr) = expr {
            self.resolve_expr(expr)?;
        }
        Ok(Object::Nil)
    }

    fn visit_class(&mut self, class: &Class<Result<Object>>) -> Result<Object> {
        let (name, methods) = class.extract();
        self.define(name);
        self.declare(name);
        Ok(Object::Nil)
    }
}
