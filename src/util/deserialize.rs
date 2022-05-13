use serde::{Deserialize, Deserializer};

// Stolen from a beautiful stranger @:
// https://users.rust-lang.org/t/how-can-i-skip-deserializing-and-use-default-if-field-is-null-without-wrapping-thing-in-an-option-type-in-serde/35792
pub fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
