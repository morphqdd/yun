use crate::interpreter::error::RuntimeErrorType;
use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Void,
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::String(_) => "string".into(),
            Object::Number(_) => "number".into(),
            Object::Bool(_) => "boolean".into(),
            Object::Nil => "nil".into(),
            Object::Void => "void".into(),
        }
    }
}

impl Neg for Object {
    type Output = Result<Object>;

    fn neg(self) -> Self::Output {
        match self {
            Object::Number(n) => Ok(Object::Number(-n)),
            _ => Err(anyhow!(RuntimeErrorType::CannotNegateType(self.get_type()))),
        }
    }
}

impl Not for Object {
    type Output = Result<Object>;

    fn not(self) -> Self::Output {
        match self {
            Object::Bool(b) => Ok(Object::Bool(!b)),
            Object::String(_) => Ok(Object::Bool(false)),
            Object::Number(_) => Ok(Object::Bool(false)),
            Object::Nil => Ok(Object::Bool(true)),
            Object::Void => Ok(Object::Bool(true)),
        }
    }
}

impl<'a> Not for &'a Object {
    type Output = Result<Object>;

    fn not(self) -> Self::Output {
        match self {
            Object::Bool(b) => Ok(Object::Bool(!b)),
            Object::String(_) => Ok(Object::Bool(false)),
            Object::Number(_) => Ok(Object::Bool(false)),
            Object::Nil => Ok(Object::Bool(true)),
            Object::Void => Ok(Object::Bool(true)),
        }
    }
}

impl Add for Object {
    type Output = Result<Object>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
            (Object::String(a), Object::String(b)) => Ok(Object::String(a.to_owned() + b)),
            _ => Err(anyhow!(RuntimeErrorType::CannotAddTypes(
                self.get_type(),
                rhs.get_type()
            ))),
        }
    }
}

impl Sub for Object {
    type Output = Result<Object>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
            _ => Err(anyhow!(RuntimeErrorType::CannotSubtractTypes(
                self.get_type(),
                rhs.get_type()
            ))),
        }
    }
}

impl Mul for Object {
    type Output = Result<Object>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
            _ => Err(anyhow!(RuntimeErrorType::CannotMultiplyTypes(
                self.get_type(),
                rhs.get_type()
            ))),
        }
    }
}

impl Div for Object {
    type Output = Result<Object>;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
            _ => Err(anyhow!(RuntimeErrorType::CannotDivideTypes(
                self.get_type(),
                rhs.get_type()
            ))),
        }
    }
}

impl PartialEq<Self> for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            (Object::Void, Object::Void) => true,
            _ => false,
        }
    }
}

impl PartialOrd<Self> for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a.partial_cmp(b),
            (Object::String(a), Object::String(b)) => a.partial_cmp(b),
            (Object::Bool(a), Object::Bool(b)) => a.partial_cmp(b),
            (Object::Nil, Object::Nil) => None,
            _ => Some(Ordering::Equal),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(str) => write!(f, "{}", str),
            Object::Number(num) => write!(f, "{}", num),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
            Object::Void => write!(f, ""),
        }
    }
}
