pub mod token;
use crate::interpreter::scanner::token::Token;
use anyhow::Result;
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>> {
        Ok(vec![])
    }
}
