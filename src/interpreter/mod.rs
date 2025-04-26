pub mod ast;
pub mod parser;
pub mod scanner;
pub mod shell;

use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};
use crate::interpreter::parser::Parser;
use crate::interpreter::scanner::token::literal::Object;
use crate::interpreter::scanner::token::token_type::TokenType;
use crate::interpreter::scanner::token::Token;
use crate::interpreter::scanner::Scanner;
use crate::interpreter::shell::Shell;
use anyhow::{anyhow, Result};
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};

#[derive(Default)]
pub struct Interpreter {
    has_error: bool,
}

impl Interpreter {
    pub fn run_shell(mut self) -> Result<()> {
        let mut shell = Shell::new();
        let shell_ref = shell.as_mut();
        loop {
            self.has_error = false;
            print!("@> ");
            io::stdout().flush()?;
            let mut buf_line = String::new();
            if let Err(err) = io::stdin().read_line(&mut buf_line) {
                print!("{}", err);
            }

            shell_ref.set_command(buf_line.trim().to_string());

            if let Err(err) = self.run(shell_ref.get_command()) {
                print!("{}", err);
            };
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

    fn run(&mut self, code: &str) -> Result<()> {
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        if let Err(err) = self.interpret(ast.deref()) {
            println!("{}", err)
        };

        Ok(())
    }

    fn interpret(&mut self, expr: &dyn Expr<Result<Object>>) -> Result<()> {
        match self.evaluate(expr) {
            Ok(value) => println!("{}", value),
            Err(err) => return Err(err),
        }
        Ok(())
    }

    #[inline]
    fn evaluate(&mut self, expr: &dyn Expr<Result<Object>>) -> Result<Object> {
        expr.accept(self)
    }

    pub fn error_by_token(token: Token, msg: &str) -> String {
        if token.get_type().eq(&TokenType::Eof) {
            Interpreter::report(token.get_line(), token.get_pos_in_line(), "at end", msg)
        } else {
            Interpreter::report(
                token.get_line(),
                token.get_pos_in_line(),
                &format!("at '{}'", token.get_lexeme()),
                msg,
            )
        }
    }

    fn error(line: usize, pos_in_line: usize, msg: &str) -> String {
        Interpreter::report(line, pos_in_line, "", msg)
    }

    fn report(line: usize, pos_in_line: usize, _where: &str, msg: &str) -> String {
        format!(
            "[{}:{}] Error{}: {}\n",
            line,
            pos_in_line,
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

        match binary.get_op_type() {
            TokenType::Equal => Ok(Object::Bool(left == right)),
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::Greater => Ok(Object::Bool(left > right)),
            TokenType::Less => Ok(Object::Bool(left < right)),
            TokenType::GreaterEqual => Ok(Object::Bool(left >= right)),
            TokenType::LessEqual => Ok(Object::Bool(left <= right)),
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            _ => Err(anyhow!(Interpreter::error_by_token(
                binary.get_token(),
                "Unsupported binary operator"
            ))),
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping<Result<Object>>) -> Result<Object> {
        self.evaluate(grouping.get_expr())
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<Object> {
        Ok(literal.get_value().unwrap().clone())
    }

    fn visit_unary(&mut self, unary: &Unary<Result<Object>>) -> Result<Object> {
        let obj = self.evaluate(unary.get_right())?;
        match unary.get_op_type() {
            TokenType::Minus => -obj,
            TokenType::Bang => !obj,
            _ => Err(anyhow!(Interpreter::error_by_token(
                unary.get_token(),
                "Unsupported unary operator"
            ))),
        }
    }
}
