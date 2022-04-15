use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ThreatCollections {
    pub threat_collections: Vec<ThreatCollection>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct ThreatCollection {
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub last_updated: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub observables: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub user_key: String,
}
