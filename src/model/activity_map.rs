use serde::Deserialize;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ActivityMaps {
    pub activity_maps: Vec<ActivityMap>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ActivityMap {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mode: String,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub owner: String,
    #[serde(deserialize_with = "null_to_default")]
    pub rights: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub short_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub show_alert_status: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub weighting: String,
    // TODO: Find variants for walks
    // #[serde(deserialize_with = "null_to_default")]
    // #[serde(default)]
    // pub walks: Vec<Walks>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Walks {
    #[serde(deserialize_with = "null_to_default")]
    pub origins: Vec<ObjectIdType>,
    #[serde(deserialize_with = "null_to_default")]
    pub steps: Vec<Steps>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ObjectIdType {
    #[serde(deserialize_with = "null_to_default")]
    pub object_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ProtocolRole {
    #[serde(deserialize_with = "null_to_default")]
    pub protocol: String,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Steps {
    #[serde(deserialize_with = "null_to_default")]
    pub peer_in: Vec<ObjectIdType>,
    #[serde(deserialize_with = "null_to_default")]
    pub peer_not_in: Vec<ObjectIdType>,
    #[serde(deserialize_with = "null_to_default")]
    pub relationships: Vec<ProtocolRole>,
}
