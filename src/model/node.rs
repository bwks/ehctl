use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct Node {
    #[serde(deserialize_with = "null_to_default")]
    pub add_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub firmware_version: String,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub license_status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub nickname: String,
    pub ntp_sync: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub product_key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status_message: String,
    #[serde(deserialize_with = "null_to_default")]
    pub time_added: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub time_offset: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub uuid: String,
}
