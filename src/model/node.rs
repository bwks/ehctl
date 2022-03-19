use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Node {
    #[serde(deserialize_with = "null_to_default")]
    add_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    display_name: String,
    enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    firmware_version: String,
    #[serde(deserialize_with = "null_to_default")]
    hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    id: i64,
    #[serde(deserialize_with = "null_to_default")]
    license_status: String,
    #[serde(deserialize_with = "null_to_default")]
    nickname: String,
    ntp_sync: bool,
    #[serde(deserialize_with = "null_to_default")]
    product_key: String,
    #[serde(deserialize_with = "null_to_default")]
    status_code: String,
    #[serde(deserialize_with = "null_to_default")]
    status_message: String,
    #[serde(deserialize_with = "null_to_default")]
    time_added: i64,
    #[serde(deserialize_with = "null_to_default")]
    time_offset: i64,
    #[serde(deserialize_with = "null_to_default")]
    uuid: String,
}
