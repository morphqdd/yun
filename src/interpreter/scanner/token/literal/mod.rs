use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
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
