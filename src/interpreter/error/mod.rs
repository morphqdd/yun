use crate::interpreter::scanner::token::Token;
use crate::interpreter::Interpreter;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    ty: RuntimeErrorType,
}

impl RuntimeError {
    pub fn new(token: Token, ty: RuntimeErrorType) -> Self {
        Self { token, ty }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Interpreter::error_by_token(self.token.clone(), &self.ty.to_string())
        )
    }
}

impl Error for RuntimeError {}

#[derive(Debug, Clone)]
pub enum RuntimeErrorType {
    CannotAddTypes(String, String),
    CannotSubtractTypes(String, String),
    CannotMultiplyTypes(String, String),
    CannotDivideTypes(String, String),
    CannotNegateType(String),
    UnsupportedUnaryOperator(String),
    UnsupportedBinaryOperator(String),
}

impl Display for RuntimeErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorType::CannotAddTypes(ty1, ty2) => {
                write!(f, "Cannot add types '{}' and '{}'", ty1, ty2)
            }
            RuntimeErrorType::CannotSubtractTypes(ty1, ty2) => {
                write!(f, "Cannot subtract '{}' and '{}'", ty1, ty2)
            }
            RuntimeErrorType::CannotMultiplyTypes(ty1, ty2) => {
                write!(f, "Cannot multiply '{}' and '{}'", ty1, ty2)
            }
            RuntimeErrorType::CannotDivideTypes(ty1, ty2) => {
                write!(f, "Cannot divide '{}' and '{}'", ty1, ty2)
            }
            RuntimeErrorType::CannotNegateType(ty) => write!(f, "Cannot negate '{}'", ty),
            RuntimeErrorType::UnsupportedUnaryOperator(op) => {
                write!(f, "Unsupported unary operator '{}'", op)
            }
            RuntimeErrorType::UnsupportedBinaryOperator(op) => {
                write!(f, "Unsupported binary operator '{}'", op)
            }
        }
    }
}

impl Error for RuntimeErrorType {}
