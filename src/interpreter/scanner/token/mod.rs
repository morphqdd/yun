use crate::interpreter::scanner::token::literal::Object;
use crate::interpreter::scanner::token::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub mod literal;
pub mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Object>, line: usize) -> Token {
        Self {
            token_type,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
