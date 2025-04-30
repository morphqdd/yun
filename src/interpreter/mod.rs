pub mod ast;
pub mod environment;
pub mod error;
pub mod native_functions;
pub mod parser;
pub mod scanner;
pub mod shell;
pub mod yun_callable;

use crate::interpreter::ast::expr::assignment::Assign;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::call::Call;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::logical::Logical;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::variable::Variable;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::ast::stmt::block::Block;
use crate::interpreter::ast::stmt::if_stmt::If;
use crate::interpreter::ast::stmt::let_stmt::Let;
use crate::interpreter::ast::stmt::print::Print;
use crate::interpreter::ast::stmt::stmt_expr::StmtExpr;
use crate::interpreter::ast::stmt::while_stmt::While;
use crate::interpreter::ast::stmt::{Stmt, StmtVisitor};
use crate::interpreter::environment::Environment;
use crate::interpreter::error::{RuntimeError, RuntimeErrorType};
use crate::interpreter::parser::Parser;
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::scanner::token::token_type::TokenType;
use crate::interpreter::scanner::token::Token;
use crate::interpreter::scanner::Scanner;
use crate::interpreter::shell::Shell;
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use std::{fs, io};

pub struct Interpreter {
    env: Option<Rc<RefCell<Environment>>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut env = Environment::default();
        env.define(
            "clock",
            Some(Object::Callable {
                call: |_interpreter, _arguments| -> Result<Object> {
                    Ok(Object::Number(
                        std::time::SystemTime::now()
                            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                            .as_micros() as f64,
                    ))
                },
                arity: || 0,
            }),
        );

        env.define(
            "panic",
            Some(Object::Callable {
                call: |_, args| {
                    Err(anyhow!(
                        RuntimeErrorType::UserPanicWithMsg(args[0].clone(),)
                    ))
                },
                arity: || 1,
            }),
        );

        env.define(
            "string",
            Some(Object::Callable {
                call: |_, args| Ok(Object::String(args[0].clone().to_string())),
                arity: || 1,
            }),
        );

        Self {
            env: Some(Rc::new(RefCell::new(env))),
        }
    }
}

impl Interpreter {
    pub fn run_shell(mut self) -> Result<()> {
        let mut shell = Shell::new();
        let shell_ref = shell.as_mut();
        loop {
            print!("@> ");
            io::stdout().flush()?;
            let mut buf_line = String::new();
            if let Err(err) = io::stdin().read_line(&mut buf_line) {
                print!("{}", err);
            }

            shell_ref.set_command(buf_line.trim().to_string());

            match self.run(shell_ref.get_command()) {
                Ok(res) => match res {
                    Object::Void => {}
                    _ => println!("{}", res),
                },
                Err(err) => print!("{}", err),
            }
        }
    }

    pub fn run_script(mut self, path: &PathBuf) -> Result<()> {
        let code = fs::read_to_string(path)?;
        if let Err(err) = self.run(&code) {
            println!("{}", err);
            exit(65)
        };
        Ok(())
    }

    fn run(&mut self, code: &str) -> Result<Object> {
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let res = self.interpret(ast)?;

        Ok(res)
    }

    fn interpret(&mut self, statements: Vec<Box<dyn Stmt<Result<Object>>>>) -> Result<Object> {
        let mut res = Object::Void;
        for stmt in statements {
            res = self.execute(stmt.deref())?;
        }
        Ok(res)
    }

    fn execute_block(
        &mut self,
        statements: &[Box<dyn Stmt<Result<Object>>>],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object> {
        let previous = self.env.replace(environment);
        for stmt in statements {
            if let Err(err) = self.execute(stmt.deref()) {
                self.env.replace(previous.unwrap());
                return Err(err);
            }
        }
        self.env.replace(previous.unwrap());
        Ok(Object::Void)
    }

    #[inline]
    fn execute(&mut self, statement: &dyn Stmt<Result<Object>>) -> Result<Object> {
        statement.accept(self)
    }

    #[inline]
    fn evaluate(&mut self, expr: &dyn Expr<Result<Object>>) -> Result<Object> {
        expr.accept(self)
    }

    #[inline]
    fn handle_runtime_error(token: Token, res: Result<Object>) -> Result<Object> {
        if let Err(err) = res {
            if let Some(err_ty) = err.downcast_ref::<RuntimeErrorType>() {
                return match err_ty {
                    RuntimeErrorType::UserPanicWithMsg(msg) => {
                        Err(anyhow!(Interpreter::panic_handler(token, &msg.to_string())))
                    }
                    _ => Err(anyhow!(RuntimeError::new(token, err_ty.clone()))),
                };
            }
            return Err(err);
        }
        res
    }

    #[inline]
    fn panic_handler(token: Token, msg: &str) -> String {
        Interpreter::report("Panic", token.get_line(), token.get_pos_in_line(), "", msg)
    }

    fn is_truly(&self, obj: &Object) -> Result<bool> {
        Ok((!(!obj)?)? == Object::Bool(true))
    }

    pub fn error_by_token(token: Token, msg: &str) -> String {
        if token.get_type().eq(&TokenType::Eof) {
            Interpreter::report(
                "Error",
                token.get_line(),
                token.get_pos_in_line(),
                "at end",
                msg,
            )
        } else {
            Interpreter::report(
                "Error",
                token.get_line(),
                token.get_pos_in_line(),
                &format!("at '{}'", token.get_lexeme()),
                msg,
            )
        }
    }

    fn error(line: usize, pos_in_line: usize, msg: &str) -> String {
        Interpreter::report("Error", line, pos_in_line, "", msg)
    }

    fn report(report_ty: &str, line: usize, pos_in_line: usize, _where: &str, msg: &str) -> String {
        format!(
            "[{}:{}] {}{}: {}\n",
            line,
            pos_in_line,
            report_ty,
            if _where.is_empty() {
                "".to_owned()
            } else {
                " ".to_owned() + _where
            },
            msg
        )
    }
}

impl ExprVisitor<Result<Object>> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary<Result<Object>>) -> Result<Object> {
        let left = self.evaluate(binary.get_left())?;
        let right = self.evaluate(binary.get_right())?;

        let res = match binary.get_op_type() {
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::Greater => Ok(Object::Bool(left > right)),
            TokenType::Less => Ok(Object::Bool(left < right)),
            TokenType::GreaterEqual => Ok(Object::Bool(left >= right)),
            TokenType::LessEqual => Ok(Object::Bool(left <= right)),
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            _ => Err(anyhow!(RuntimeError::new(
                binary.get_token(),
                RuntimeErrorType::UnsupportedBinaryOperator(binary.get_op_lexeme().into())
            ))),
        };

        Interpreter::handle_runtime_error(binary.get_token(), res)
    }

    fn visit_grouping(&mut self, grouping: &Grouping<Result<Object>>) -> Result<Object> {
        self.evaluate(grouping.get_expr())
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<Object> {
        Ok(literal.get_value().unwrap().clone())
    }

    fn visit_unary(&mut self, unary: &Unary<Result<Object>>) -> Result<Object> {
        let obj = self.evaluate(unary.get_right())?;
        let res = match unary.get_op_type() {
            TokenType::Minus => -obj,
            TokenType::Bang => !obj,
            _ => Err(anyhow!(RuntimeError::new(
                unary.get_token(),
                RuntimeErrorType::UnsupportedBinaryOperator(unary.get_op_lexeme().into())
            ))),
        };

        Interpreter::handle_runtime_error(unary.get_token(), res)
    }

    fn visit_variable(&mut self, variable: &Variable) -> Result<Object> {
        if let Some(env) = &self.env {
            return env.borrow().get(&variable.get_token());
        }
        Err(anyhow!(RuntimeError::new(
            variable.get_token(),
            RuntimeErrorType::BugEnvironmentNotInit
        )))
    }

    fn visit_assign(&mut self, assign: &Assign<Result<Object>>) -> Result<Object> {
        let value = self.evaluate(assign.get_value())?;
        if let Some(env) = &self.env {
            return env.borrow_mut().assign(&assign.get_token(), value.clone());
        }
        Err(anyhow!(RuntimeError::new(
            assign.get_token(),
            RuntimeErrorType::BugEnvironmentNotInit
        )))
    }

    fn visit_logical(&mut self, logical: &Logical<Result<Object>>) -> Result<Object> {
        let left = self.evaluate(logical.get_left())?;
        if logical.get_operator().get_type().eq(&TokenType::Or) {
            if self.is_truly(&left)? {
                return Ok(left);
            }
        } else if !self.is_truly(&left)? {
            return Ok(left);
        }

        self.evaluate(logical.get_right())
    }

    fn visit_call(&mut self, call_: &Call<Result<Object>>) -> Result<Object> {
        let callable = self.evaluate(call_.get_callable())?;
        let mut args: Vec<Object> = Vec::new();
        for arg in call_.get_args() {
            args.push(self.evaluate(arg)?);
        }

        match callable {
            Object::Callable { call, arity } => {
                if args.len() != arity() {
                    return Err(anyhow!(RuntimeError::new(
                        call_.get_token(),
                        RuntimeErrorType::ArityOfFuncNotEqSizeOfArgs
                    )));
                }
                Interpreter::handle_runtime_error(call_.get_token(), call(self, args))
            }
            _ => Err(anyhow!(RuntimeError::new(
                call_.get_token(),
                RuntimeErrorType::NotCallable
            ))),
        }
    }
}

impl StmtVisitor<Result<Object>> for Interpreter {
    fn visit_expr(&mut self, stmt: &StmtExpr<Result<Object>>) -> Result<Object> {
        self.evaluate(stmt.expr())
    }

    fn visit_print(&mut self, stmt: &Print<Result<Object>>) -> Result<Object> {
        let value = self.evaluate(stmt.expr())?;
        println!("{}", value);
        Ok(Object::Void)
    }

    fn visit_let(&mut self, stmt: &Let<Result<Object>>) -> Result<Object> {
        match stmt.get_initializer() {
            Some(initializer) => {
                let value = self.evaluate(initializer)?;

                match &self.env {
                    None => {
                        return Err(anyhow!(RuntimeError::new(
                            stmt.get_ident(),
                            RuntimeErrorType::BugEnvironmentNotInit
                        )));
                    }
                    Some(env) => {
                        env.borrow_mut()
                            .define(stmt.get_ident().get_lexeme(), Some(value));
                    }
                }
            }
            None => match &self.env {
                None => {
                    return Err(anyhow!(RuntimeError::new(
                        stmt.get_ident(),
                        RuntimeErrorType::BugEnvironmentNotInit
                    )));
                }
                Some(env) => {
                    env.borrow_mut().define(stmt.get_ident().get_lexeme(), None);
                }
            },
        }
        Ok(Object::Void)
    }

    fn visit_block(&mut self, stmt: &Block<Result<Object>>) -> Result<Object> {
        self.execute_block(
            stmt.get_stmts(),
            Rc::new(RefCell::new(Environment::new(self.env.clone()))),
        )?;
        Ok(Object::Void)
    }

    fn visit_if(&mut self, stmt: &If<Result<Object>>) -> Result<Object> {
        if self.evaluate(stmt.get_cond())? == Object::Bool(true) {
            self.execute(stmt.get_then_stmt())?;
        } else if let Some(else_stmt) = stmt.get_else_stmt() {
            self.execute(else_stmt)?;
        }

        Ok(Object::Void)
    }

    fn visit_while(&mut self, stmt: &While<Result<Object>>) -> Result<Object> {
        let mut cond = self.evaluate(stmt.get_cond())?;
        while self.is_truly(&cond)? {
            self.execute(stmt.get_stmt())?;
            cond = self.evaluate(stmt.get_cond())?;
        }
        Ok(Object::Void)
    }
}
