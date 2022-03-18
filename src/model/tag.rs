use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Tag {
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
}
