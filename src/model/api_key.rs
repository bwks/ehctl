use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct ApiKey {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub time_added: i64,
    #[serde(default)]
    #[serde(deserialize_with = "null_to_default")]
    pub user_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub username: String,
}
