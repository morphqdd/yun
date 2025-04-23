pub mod shell;

use std::io;
use anyhow::Result;
use crate::interpreter::shell::Shell;

pub struct Interpreter {

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

            let mut buf_line = String::new();
            if let Err(err) = io::stdin().read_line(&mut buf_line) {
                print!("{}", err);
            }

            shell.set_command(buf_line);

            println!("@> Entered command: {}", shell_ref.get_command())
        }
    }
}