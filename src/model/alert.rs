use std::collections::HashMap;

use serde::Deserialize;

use crate::deserialize::null_to_default;

#[derive(Debug, Deserialize)]
pub struct Alerts {
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Alert {
    #[serde(deserialize_with = "null_to_default")]
    pub apply_all: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    pub categories: Vec<String>,
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
    pub object_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub operand: String,
    #[serde(deserialize_with = "null_to_default")]
    pub operator: String,
    pub param: HashMap<String, String>,
    pub param2: HashMap<String, String>,
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

impl Default for Alert {
    fn default() -> Self {
        let param: HashMap<String, String> = HashMap::new();
        let param2: HashMap<String, String> = HashMap::new();
        Self {
            apply_all: false,
            author: "".to_string(),
            categories: vec![],
            cc: vec![],
            description: "".to_string(),
            disabled: false,
            field_name: "".to_string(),
            field_name2: "".to_string(),
            field_op: "".to_string(),
            id: 0,
            interval_length: 0,
            mod_time: 0,
            name: "".to_string(),
            notify_snmp: false,
            object_type: "".to_string(),
            operand: "".to_string(),
            operator: "".to_string(),
            param,
            param2,
            protocols: vec![],
            refire_interval: 0,
            severity: 0,
            stat_name: "".to_string(),
            _type: "".to_string(),
            units: "".to_string(),
        }
    }
}
