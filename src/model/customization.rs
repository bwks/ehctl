use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Tabled, Deserialize)]
pub struct Customization {
    pub auto: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub create_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    pub recovered: bool,
}
