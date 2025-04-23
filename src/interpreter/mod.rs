pub mod shell;

use crate::interpreter::shell::Shell;
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

pub struct Interpreter {}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_shell(self) -> Result<()> {
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

            println!("@> Entered command: {}", shell_ref.get_command())
        }
    }

    pub fn run_script(self, path: &PathBuf) -> Result<()> {
        let code = fs::read_to_string(path)?;
        println!("Source: {}", code);

        Ok(())
    }
}
