use std::cell::RefCell;
use crate::interpreter::error::{InterpreterError, Result};
use crate::interpreter::scanner::token::object::Object;
use crate::interpreter::Interpreter;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use crate::interpreter::ast::stmt::fun_stmt::Fun;
use crate::interpreter::environment::Environment;
use crate::rc;

type CallFn = Rc<dyn Fn(&mut Interpreter, Vec<Object>) -> Result<Object>>;

#[derive(Clone)]
pub struct Callable {
    id: u64,
    declaration: Option<Rc<RefCell<Fun<Result<Object>>>>>,
    closure: Option<Rc<RefCell<Environment>>>,
    call: CallFn,
    arity: Rc<dyn Fn() -> usize>,
    to_string: Rc<dyn Fn() -> String>,
}

impl Callable {
    pub fn new(
        declaration: Option<Rc<RefCell<Fun<Result<Object>>>>>,
        closure: Option<Rc<RefCell<Environment>>>,
    ) -> Self {
        
        let (id, name, params, body) = declaration.clone().unwrap().borrow().clone().extract();
        let arity = params.len();
        
        Self {
            id,
            declaration,
            closure: closure.clone(),
            call: rc!(move |interpreter, args| {
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
            arity: rc!(move || arity),
            to_string: rc!(move || name.get_lexeme().into()),
        }
    }

    pub fn build(
        id: u64,
        declaration: Option<Rc<RefCell<Fun<Result<Object>>>>>,
        closure: Option<Rc<RefCell<Environment>>>,
        call: CallFn,
        arity: Rc<dyn Fn() -> usize>,
        to_string: Rc<dyn Fn() -> String>,
    ) -> Self {
        Self {
            id,
            declaration,
            closure,
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
    pub fn get_closure(&self) -> Option<Rc<RefCell<Environment>>> {
        self.closure.clone()
    }
    
    pub fn get_declaration(&self) -> Option<Rc<RefCell<Fun<Result<Object>>>>> {
        self.declaration.clone()
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ <Callable {{ ... }}> }}", self.get_string())
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function#{} {}>", self.id, self.get_string())
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
