use crate::interpreter::parser::error::{ParserError, ParserErrorType};
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::scanner::token::Token;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        if let Some(value) = self.values.get(name.get_lexeme()) {
            return Ok(value.clone());
        }

        Err(anyhow!(ParserError::new(
            name.clone(),
            ParserErrorType::UndefinedVariable(name.get_lexeme().to_string())
        )))
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object> {
        if self.values.contains_key(name.get_lexeme()) {
            self.values
                .insert(name.get_lexeme().to_string(), value.clone());
            return Ok(value);
        }

        Err(anyhow!(ParserError::new(
            name.clone(),
            ParserErrorType::UndefinedVariable(name.get_lexeme().to_string())
        )))
    }
}
