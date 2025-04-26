pub mod ast;
pub mod parser;
pub mod scanner;
pub mod shell;

use crate::interpreter::ast::printer::AstPrinter;
use crate::interpreter::parser::Parser;
use crate::interpreter::scanner::token::token_type::TokenType;
use crate::interpreter::scanner::token::Token;
use crate::interpreter::scanner::Scanner;
use crate::interpreter::shell::Shell;
use anyhow::Result;
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

    pub fn run_script(self, path: &PathBuf) -> Result<()> {
        let code = fs::read_to_string(path)?;
        if let Err(err) = self.run(&code) {
            println!("{}", err);
            exit(65)
        };
        Ok(())
    }

    fn run(&self, code: &str) -> Result<()> {
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        println!("{}", AstPrinter.print(ast.deref()));

        Ok(())
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
