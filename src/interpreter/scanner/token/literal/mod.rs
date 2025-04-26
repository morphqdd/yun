use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Neg for Object {
    type Output = Result<Object>;

    fn neg(self) -> Self::Output {
        match self {
            Object::Number(n) => Ok(Object::Number(-n)),
            _ => Err(anyhow!("Cannot negate this type")),
        }
    }
}

impl Not for Object {
    type Output = Result<Object>;

    fn not(self) -> Self::Output {
        match self {
            Object::Bool(b) => Ok(Object::Bool(!b)),
            Object::String(_) => Ok(Object::Bool(true)),
            Object::Number(_) => Ok(Object::Bool(true)),
            Object::Nil => Ok(Object::Bool(false)),
        }
    }
}

impl Add for Object {
    type Output = Result<Object>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
            (Object::String(a), Object::String(b)) => Ok(Object::String(a + &b)),
            _ => Err(anyhow!("Cannot add this types")),
        }
    }
}

impl Sub for Object {
    type Output = Result<Object>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
            _ => Err(anyhow!("Cannot sub this types")),
        }
    }
}

impl Mul for Object {
    type Output = Result<Object>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
            _ => Err(anyhow!("Cannot add this types")),
        }
    }
}

impl Div for Object {
    type Output = Result<Object>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
            _ => Err(anyhow!("Cannot add this types")),
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
        let str = match self {
            Object::String(str) => format!("\"{}\"", str),
            Object::Number(num) => num.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Nil => "nil".to_string(),
        };
        write!(f, "{}", str)
    }
}
