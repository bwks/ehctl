use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Node {
    #[serde(deserialize_with = "null_to_default")]
    pub add_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub firmware_version: String,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub license_status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub nickname: String,
    ntp_sync: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub product_key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status_message: String,
    #[serde(deserialize_with = "null_to_default")]
    pub time_added: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub time_offset: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub uuid: String,
}
