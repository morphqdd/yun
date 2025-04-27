use crate::interpreter::Interpreter;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParserError {
    line: usize,
    pos_in_line: usize,
    ty: ParserErrorType,
}

impl ParserError {
    pub fn new(line: usize, pos_in_line: usize, ty: ParserErrorType) -> Self {
        Self {
            line,
            pos_in_line,
            ty,
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Interpreter::error(self.line, self.pos_in_line, &self.ty.to_string())
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
