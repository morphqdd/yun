use std::collections::HashMap;
use crate::interpreter::scanner::token::object::callable::Callable;
use crate::interpreter::scanner::token::object::instance::Instance;
use crate::interpreter::scanner::token::object::Object;
use crate::rc;
use crate::utils::next_id;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    id: u64,
    name: String,
    methods: HashMap<String, Object>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Object>) -> Self {
        Self {
            id: next_id(),
            name,
            methods,
        }
    }
    
    pub fn find_method(&self, name: &str) -> Option<Object> {
        self.methods.get(name).cloned()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Class> for Callable {
    fn from(value: Class) -> Self {
        let name = value.name.clone();
        Callable::build(
            value.id,
            None,
            None,
            rc!(move |_interpreter, _args| {
                let instance = Instance::new(value.clone());
                Ok(Object::Instance(instance))
            }),
            rc!(|| 0),
            rc!(move || name.clone()),
        )
    }
}
