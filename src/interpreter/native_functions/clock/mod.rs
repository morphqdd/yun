use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::yun_callable::YunCallable;
use crate::interpreter::Interpreter;
use anyhow::Result;

pub struct ClockFunc;

impl YunCallable for ClockFunc {
    fn call(_interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object> {
        Ok(Object::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs_f64(),
        ))
    }

    fn arity() -> usize {
        0
    }
}
