use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Bundles {
    pub bundles: Vec<Bundle>,
}

#[derive(Debug, Default, Tabled, Deserialize)]
#[serde(default)]
pub struct Bundle {
    pub built_in: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub created_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
}
