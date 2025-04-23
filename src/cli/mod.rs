use std::path::PathBuf;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    path: Option<PathBuf>
}

impl Cli {
    pub fn get_path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}