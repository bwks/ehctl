use std::collections::HashMap;

use serde::Deserialize;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MitreField {
    #[serde(deserialize_with = "null_to_default")]
    pub id: String,
    #[serde(default)]
    pub legacy_ids: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Participant {
    #[serde(deserialize_with = "null_to_default")]
    pub external: bool,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(default)]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(default)]
    pub object_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_type: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(default)]
    pub object_value: String,
    #[serde(default)]
    pub origins: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
    #[serde(default)]
    pub usernames: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Detection {
    #[serde(deserialize_with = "null_to_default")]
    pub appliance_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub assignee: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub end_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub is_user_created: bool,
    #[serde(default)]
    pub mitre_tactics: Vec<MitreField>,
    #[serde(default)]
    pub mitre_techniques: Vec<MitreField>,
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    #[serde(deserialize_with = "null_to_default")]
    pub resolution: String,
    #[serde(deserialize_with = "null_to_default")]
    pub risk_score: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub start_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ticket_id: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(default)]
    pub ticket_url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub title: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub update_time: i64,
}
