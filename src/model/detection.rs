use serde::Deserialize;
use std::collections::HashMap;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Detections {
    pub detections: Vec<Detection>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Detection {
    #[serde(deserialize_with = "null_to_default")]
    pub appliance_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub assignee: String,
    pub categories: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub end_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub is_user_created: bool,
    pub mitre_tactics: Vec<MitreField>,
    pub mitre_techniques: Vec<MitreField>,
    pub participants: Vec<Participant>,
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
    pub ticket_url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub title: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub update_time: i64,
}

impl Default for Detection {
    fn default() -> Self {
        let properties: HashMap<String, serde_json::Value> = HashMap::new();
        Self {
            appliance_id: 0,
            assignee: "".to_string(),
            categories: vec![],
            description: "".to_string(),
            end_time: 0,
            id: 0,
            is_user_created: false,
            mitre_tactics: vec![],
            mitre_techniques: vec![],
            participants: vec![],
            properties,
            resolution: "".to_string(),
            risk_score: 0,
            start_time: 0,
            status: "".to_string(),
            ticket_id: "".to_string(),
            ticket_url: "".to_string(),
            title: "".to_string(),
            _type: "".to_string(),
            update_time: 0,
        }
    }
}

impl Tabled for Detection {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        vec![
            self.appliance_id.to_string(),
            self.assignee.to_string(),
            // self.categories.to_string(),
            self.description.to_string(),
            self.end_time.to_string(),
            self.id.to_string(),
            self.is_user_created.to_string(),
            // self.mitre_tactics.to_string(),
            // self.mitre_techniques.to_string(),
            // self.participants.to_string(),
            // self.properties.to_string(),
            self.resolution.to_string(),
            self.risk_score.to_string(),
            self.start_time.to_string(),
            self.status.to_string(),
            self.ticket_id.to_string(),
            self.ticket_url.to_string(),
            self.title.to_string(),
            self._type.to_string(),
            self.update_time.to_string(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "appliance_id".to_string(),
            "assignee".to_string(),
            // "categories".to_string(),
            "description".to_string(),
            "end_time".to_string(),
            "id".to_string(),
            "is_user_created".to_string(),
            // "mitre_tactics".to_string(),
            // "mitre_techniques".to_string(),
            // "participants".to_string(),
            // "properties".to_string(),
            "resolution".to_string(),
            "risk_score".to_string(),
            "start_time".to_string(),
            "status".to_string(),
            "ticket_id".to_string(),
            "ticket_url".to_string(),
            "title".to_string(),
            "type".to_string(),
            "update_time".to_string(),
        ]
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Participant {
    #[serde(deserialize_with = "null_to_default")]
    pub external: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub object_value: String,
    pub origins: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
    pub usernames: Vec<String>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MitreField {
    #[serde(deserialize_with = "null_to_default")]
    pub id: String,
    pub legacy_ids: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub url: String,
}
