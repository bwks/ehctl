#[macro_export]
/// Converts a &str to a String
macro_rules! s{
    ($x:expr) => { String::from($x) }
}