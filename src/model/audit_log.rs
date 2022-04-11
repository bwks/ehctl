use std::collections::HashMap;

use serde::Deserialize;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AuditLog {
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub occur_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub time: String,
}
