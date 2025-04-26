#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::String(str) => format!("\"{}\"", str),
            Object::Number(num) => num.to_string(),
        }
    }
}
