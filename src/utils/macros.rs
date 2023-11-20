#[macro_export]
macro_rules! str {
    () => {
        String::from("")
    };
    ($($text:tt)*) => {
        // format!("{}", $($text)*)
        String::from($($text)*)
    };
}