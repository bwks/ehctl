use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Networks {
    pub networks: Vec<Network>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct Network {
    #[serde(deserialize_with = "null_to_default")]
    pub appliance_uuid: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub idle: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub node_id: i64,
}
