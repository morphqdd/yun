use std::path::PathBuf;
use clap::Parser;
use yun_lib::interpreter::Interpreter;
use crate::cli::Cli;
use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.get_path() {
        None => Interpreter::new().run_shell(),
        Some(_) => Ok(())
    }
}