use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Trigger {
    // #[serde(deserialize_with = "null_to_default")]
    // pub apply_all: bool,
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
    // #[serde(deserialize_with = "null_to_default")]
    // pub hints: TriggerHints,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        //         let hints_str = format!(
        //             "flow_client_port_min: {}
        // flow_client_bytes:    {}
        // flow_client_port_max: {}
        // flow_server_bytes:    {}
        // flow_payload_turn:    {}
        // flow_server_portmin: {}
        // flow_server_port_max: {}",
        //             self.hints.flow_client_portmin,
        //             self.hints.flow_client_bytes,
        //             self.hints.flow_client_port_max,
        //             self.hints.flow_server_bytes,
        //             self.hints.flow_payload_turn,
        //             self.hints.flow_server_port_min,
        //             self.hints.flow_server_port_max
        //         );
        vec![
            // format!("{}", self.apply_all),
            self.author.to_string(),
            format!("{}", self.debug),
            self.description.to_string(),
            format!("{}", self.disabled),
            self.events.join(", "),
            format!("{}", self.id),
            format!("{}", self.mod_time),
            self.name.to_string(),
            // format!("{}", self.script),
            // hints_str,
        ]
    }
    fn headers() -> Vec<String> {
        vec![
            // String::from("apply_all"),
            String::from("author"),
            String::from("debug"),
            String::from("description"),
            String::from("disabled"),
            String::from("events"),
            String::from("id"),
            String::from("mod_time"),
            String::from("name"),
            // String::from("script"),
            // String::from("hints"),
        ]
    }
}
