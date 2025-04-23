use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use yun_lib::interpreter::Interpreter;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.get_path() {
        None => Interpreter::new().run_shell(),
        Some(path_to_script) => Interpreter::new().run_script(path_to_script),
    }
}
