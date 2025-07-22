#[macro_export]
macro_rules! unwrap_checked {
    ($expr:expr) => {
        $expr.unwrap_or_else(|_| unreachable!())
    };
}
