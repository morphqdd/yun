use crate::interpreter::error::Result;
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::Interpreter;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

type CallFn = Rc<dyn Fn(&mut Interpreter, Vec<Object>) -> Result<Object>>;

#[derive(Clone)]
pub struct Callable {
    call: CallFn,
    arity: Rc<dyn Fn() -> usize>,
    to_string: Rc<dyn Fn() -> String>,
}

impl Callable {
    pub fn new(
        call: CallFn,
        arity: Rc<dyn Fn() -> usize>,
        to_string: Rc<dyn Fn() -> String>,
    ) -> Self {
        Self {
            call,
            arity,
            to_string,
        }
    }

    pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object> {
        (self.call)(interpreter, arguments)
    }

    pub fn arity(&self) -> usize {
        (self.arity)()
    }

    pub fn get_string(&self) -> String {
        (self.to_string)()
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ <Callable {{ ... }}> }}", self.get_string())
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function {}>", self.get_string())
    }
}
