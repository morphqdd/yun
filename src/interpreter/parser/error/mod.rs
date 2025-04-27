use crate::interpreter::scanner::token::Token;
use crate::interpreter::Interpreter;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParserError {
    token: Token,
    ty: ParserErrorType,
}

impl ParserError {
    pub fn new(token: Token, ty: ParserErrorType) -> Self {
        Self { token, ty }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Interpreter::error_by_token(self.token.clone(), &self.ty.to_string())
        )
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum ParserErrorType {
    ExpectedMatchingParens,
    ExpectedExpression,
    ExpectedSemicolon,
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorType::ExpectedMatchingParens => write!(f, "Expect ')', after expression!"),
            ParserErrorType::ExpectedExpression => write!(f, "Expected expression!"),
            ParserErrorType::ExpectedSemicolon => write!(f, "Expected ';' after expression"),
        }
    }
}
