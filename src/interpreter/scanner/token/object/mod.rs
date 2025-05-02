use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::environment::Environment;
use crate::interpreter::error::RuntimeErrorType;
use crate::interpreter::error::{InterpreterError, Result};
use crate::interpreter::scanner::token::object::callable::Callable;
use crate::interpreter::scanner::token::object::class::Class;
use crate::rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::rc::Rc;

pub mod callable;
pub mod class;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Callable(Callable),
    Class(Class),
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
            Object::Callable { .. } => "<callable>".into(),
            Object::Class(class) => class.to_string(),
        }
    }

    pub fn function(stmt: Fun<Result<Object>>, closure: Option<Rc<RefCell<Environment>>>) -> Self {
        let (name, params, body) = stmt.extract();
        let arity = params.len();

        Self::Callable(Callable::new(
            rc!(move |interpreter, args| {
                let body = body.clone();
                let mut env = Environment::new(closure.clone());
                for i in 0..arity {
                    env.define(params[i].get_lexeme(), Some(args[i].clone()));
                }
                match interpreter.execute_block(
                    body.iter().map(AsRef::as_ref).collect(),
                    Rc::new(RefCell::new(env)),
                ) {
                    Ok(value) => Ok(value),
                    Err(err) => match err {
                        InterpreterError::Return(value) => Ok(value),
                        _ => Err(err),
                    },
                }
            }),
            rc!(move || arity),
            rc!(move || name.get_lexeme().into()),
        ))
    }

    pub fn class(name: &str) -> Self {
        Self::Class(Class::new(name.to_string()))
    }
}

impl Neg for Object {
    type Output = Result<Object>;

    fn neg(self) -> Self::Output {
        match self {
            Object::Number(n) => Ok(Object::Number(-n)),
            _ => Err(RuntimeErrorType::CannotNegateType(self.get_type()).into()),
        }
    }
}

impl Not for Object {
    type Output = Result<Object>;

    fn not(self) -> Self::Output {
        match self {
            Object::Bool(b) => Ok(Object::Bool(!b)),
            Object::Nil => Ok(Object::Bool(true)),
            Object::Void => Ok(Object::Bool(true)),
            _ => Ok(Object::Bool(false)),
        }
    }
}

impl Not for &Object {
    type Output = Result<Object>;

    fn not(self) -> Self::Output {
        match self {
            Object::Bool(b) => Ok(Object::Bool(!b)),
            Object::Nil => Ok(Object::Bool(true)),
            Object::Void => Ok(Object::Bool(true)),
            _ => Ok(Object::Bool(false)),
        }
    }
}

impl Add for Object {
    type Output = Result<Object>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
            (Object::String(a), Object::String(b)) => Ok(Object::String(a.to_owned() + b)),
            _ => Err(RuntimeErrorType::CannotAddTypes(self.get_type(), rhs.get_type()).into()),
        }
    }
}

impl Sub for Object {
    type Output = Result<Object>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
            _ => Err(RuntimeErrorType::CannotSubtractTypes(self.get_type(), rhs.get_type()).into()),
        }
    }
}

impl Mul for Object {
    type Output = Result<Object>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
            _ => Err(RuntimeErrorType::CannotMultiplyTypes(self.get_type(), rhs.get_type()).into()),
        }
    }
}

impl Div for Object {
    type Output = Result<Object>;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
            _ => Err(RuntimeErrorType::CannotDivideTypes(self.get_type(), rhs.get_type()).into()),
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
            Object::Callable(callable) => write!(f, "{}", callable),
            Object::Class(class) => write!(f, "{}", class),
        }
    }
}

impl From<Object> for Result<i32> {
    fn from(value: Object) -> Self {
        match value {
            Object::Number(n) => Ok(n as i32),
            _ => Err(RuntimeErrorType::CantToNum(value.get_type()).into()),
        }
    }
}

impl From<Object> for InterpreterError {
    fn from(value: Object) -> Self {
        InterpreterError::Return(value)
    }
}
