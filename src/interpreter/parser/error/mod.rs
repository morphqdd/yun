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
    ExpectedVariableName,
    ExpectedSemicolonAfterVarDecl,
    UndefinedVariable(String),
    InvalidAssignmentTarget,
    ExpectedMatchingBrace,
    ExpectedLeftParenAfterIf,
    ExpectedRightParenAfterIfCondition,
    ExpectedRightParenAfterForStatement,
    ExpectedLeftParenAfterFor,
    ExpectedLeftParenAfterWhile,
    ExpectedRightParenAfterWhileStatement,
    ExpectedRightParenAfterArguments,
    CountOfArgsGreaterThen255,
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorType::ExpectedMatchingParens => write!(f, "Expect ')', after expression!"),
            ParserErrorType::ExpectedExpression => write!(f, "Expected expression!"),
            ParserErrorType::ExpectedSemicolon => write!(f, "Expected ';' after expression!"),
            ParserErrorType::ExpectedVariableName => write!(f, "Expected variable name!"),
            ParserErrorType::ExpectedSemicolonAfterVarDecl => {
                write!(f, "Expected ';' after variable declaration!")
            }
            ParserErrorType::UndefinedVariable(v) => write!(f, "Undefined variable: `{}`!", v),
            ParserErrorType::InvalidAssignmentTarget => write!(f, "Invalid assignment target!"),
            ParserErrorType::ExpectedMatchingBrace => write!(f, "Expected '}}' after block!"),
            ParserErrorType::ExpectedLeftParenAfterIf => write!(f, "Expected '(' after if!"),
            ParserErrorType::ExpectedRightParenAfterIfCondition => {
                write!(f, "Expected ')' after if condition!")
            }
            ParserErrorType::ExpectedRightParenAfterForStatement => {
                write!(f, "Expected ')' after for statement!")
            }
            ParserErrorType::ExpectedLeftParenAfterFor => write!(f, "Expected '( after for!"),
            ParserErrorType::ExpectedLeftParenAfterWhile => write!(f, "Expected '( after while!"),
            ParserErrorType::ExpectedRightParenAfterWhileStatement => {
                write!(f, "Expected ')' after while statement!")
            }
            ParserErrorType::ExpectedRightParenAfterArguments => {
                write!(f, "Expected ')' after arguments!")
            }
            ParserErrorType::CountOfArgsGreaterThen255 => {
                write!(f, "Number of arguments greater than 255!")
            }
        }
    }
}
