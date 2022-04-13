use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct DeviceGroup {
    #[serde(deserialize_with = "null_to_default")]
    pub built_in: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub dynamic: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub field: String,
    // #[serde(deserialize_with = "null_to_default")]
    // pub filter: {},
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub include_custom_devices: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub value: String,
}
