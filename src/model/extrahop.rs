use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
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
