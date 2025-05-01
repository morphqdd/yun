#[macro_export]
macro_rules! b {
    ($e: expr) => {
        Box::new($e)
    };
}

#[macro_export]
macro_rules! rc {
    ($e: expr) => {
        Rc::new($e)
    };
}
