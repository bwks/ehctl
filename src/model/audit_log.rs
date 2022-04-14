use std::collections::HashMap;

use serde::Deserialize;

use crate::deserialize::null_to_default;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct AuditLogs {
    pub audit_logs: Vec<AuditLog>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct AuditLog {
    pub body: HashMap<String, serde_json::Value>,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub occur_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub time: String,
}
