use serde::Deserialize;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct EmailGroup {
    #[serde(deserialize_with = "null_to_default")]
    pub email_addresses: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub group_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub system_notifications: bool,
}
