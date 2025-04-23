use crate::interpreter::scanner::token::literal::Literal;
use crate::interpreter::scanner::token::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub mod literal;
pub mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Self {
            token_type,
            lexeme: lexeme.into(),
            literal,
            line,
        }
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
