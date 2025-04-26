#[macro_export]
macro_rules! b {
    ($e: expr) => {
        Box::new($e)
    };
}
