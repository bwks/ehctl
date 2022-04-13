use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Deserialize)]
pub struct Tags {
    pub tags: Vec<Tag>,
}

#[derive(Tabled, Deserialize)]
#[serde(default)]
pub struct Tag {
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: 0,
            mod_time: 0,
            name: "".to_string(),
        }
    }
}
