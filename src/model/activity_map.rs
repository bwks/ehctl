use std::collections::HashMap;

use serde::Deserialize;

use crate::deserialize::null_to_default;

#[derive(Debug, Deserialize)]
pub struct ActivityMaps {
    pub activity_maps: Vec<ActivityMap>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
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
    pub walks: Vec<HashMap<String, serde_json::Value>>,
}
impl Default for ActivityMap {
    fn default() -> Self {
        Self {
            description: "".to_string(),
            id: 0,
            mod_time: 0,
            mode: "".to_string(),
            name: "".to_string(),
            owner: "".to_string(),
            rights: vec![],
            short_code: "".to_string(),
            show_alert_status: false,
            weighting: "".to_string(),
            walks: vec![],
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Walks {
    pub origins: Vec<ObjectIdType>,
    pub steps: Vec<Steps>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ObjectIdType {
    #[serde(deserialize_with = "null_to_default")]
    pub object_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_type: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ProtocolRole {
    #[serde(deserialize_with = "null_to_default")]
    pub protocol: String,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Steps {
    pub peer_in: Vec<ObjectIdType>,
    pub peer_not_in: Vec<ObjectIdType>,
    pub relationships: Vec<ProtocolRole>,
}
