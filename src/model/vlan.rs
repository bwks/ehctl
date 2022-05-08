use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Vlans {
    pub vlans: Vec<Vlan>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct Vlan {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub network_id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub node_id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlanid: u16,
}
