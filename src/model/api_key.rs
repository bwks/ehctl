use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct ApiKeys {
    pub api_keys: Vec<ApiKey>,
}

#[derive(Tabled, Default, Debug, Deserialize)]
#[serde(default)]
pub struct ApiKey {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub time_added: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub user_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub username: String,
}
