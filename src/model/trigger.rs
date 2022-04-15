use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Triggers {
    pub triggers: Vec<Trigger>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Trigger {
    #[serde(deserialize_with = "null_to_default")]
    pub apply_all: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    #[serde(deserialize_with = "null_to_default")]
    pub debug: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub disabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub events: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub script: String,
    pub hints: TriggerHints,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TriggerHints {
    #[serde(deserialize_with = "null_to_default")]
    pub flow_client_portmin: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_client_bytes: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_client_port_max: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_server_bytes: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_payload_turn: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_server_port_min: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub flow_server_port_max: i64,
}

// TODO: This needs works to get working
impl Tabled for Trigger {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let hints_str = format!(
            "flow_client_port_min : {}
flow_client_bytes    : {}
flow_client_port_max : {}
flow_server_bytes    : {}
flow_payload_turn    : {}
flow_server_portmin  : {}
flow_server_port_max : {}",
            self.hints.flow_client_portmin,
            self.hints.flow_client_bytes,
            self.hints.flow_client_port_max,
            self.hints.flow_server_bytes,
            self.hints.flow_payload_turn,
            self.hints.flow_server_port_min,
            self.hints.flow_server_port_max
        );
        vec![
            format!("{}", self.apply_all),
            self.author.to_string(),
            self.debug.to_string(),
            self.description.to_string(),
            self.disabled.to_string(),
            self.events.join(", "),
            self.id.to_string(),
            self.mod_time.to_string(),
            self.name.to_string(),
            // self.script.to_string(),
            hints_str,
        ]
    }
    fn headers() -> Vec<String> {
        vec![
            "apply_all".to_string(),
            "author".to_string(),
            "debug".to_string(),
            "description".to_string(),
            "disabled".to_string(),
            "events".to_string(),
            "id".to_string(),
            "mod_time".to_string(),
            "name".to_string(),
            // "script".to_string(),
            "hints".to_string(),
        ]
    }
}
