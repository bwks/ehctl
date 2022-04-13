use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct ExtraHop {
    #[serde(deserialize_with = "null_to_default")]
    pub display_host: String,
    #[serde(deserialize_with = "null_to_default")]
    pub external_hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub mgmt_ipaddr: String,
    #[serde(deserialize_with = "null_to_default")]
    pub platform: String,
    #[serde(deserialize_with = "null_to_default")]
    pub version: String,
}
