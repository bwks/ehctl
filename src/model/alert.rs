use std::collections::HashMap;

use serde::Deserialize;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Alert {
    #[serde(deserialize_with = "null_to_default")]
    pub apply_all: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub disabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub field_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub field_name2: String,
    #[serde(deserialize_with = "null_to_default")]
    pub field_op: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub interval_length: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub notify_snmp: bool,
    #[serde(default)]
    pub object_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub operand: String,
    #[serde(deserialize_with = "null_to_default")]
    pub operator: String,
    #[serde(default)]
    pub param: HashMap<String, String>,
    #[serde(default)]
    pub param2: HashMap<String, String>,
    #[serde(default)]
    pub protocols: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub refire_interval: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub severity: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub stat_name: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub units: String,
}
