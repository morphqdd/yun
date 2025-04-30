use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::Interpreter;
use anyhow::Result;
pub trait YunCallable {
    fn call(interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object>
    where
        Self: Sized;
    fn arity() -> usize
    where
        Self: Sized;
}
