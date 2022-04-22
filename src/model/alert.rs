use std::collections::HashMap;

use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize, Tabled)]
#[serde(default)]
pub struct AlertBrief {
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub severity: i64,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize)]
pub struct Alerts {
    pub alerts: Vec<Alert>,
}

#[derive(Default, Deserialize)]
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

impl Alert {
    pub fn brief(&self) -> AlertBrief {
        AlertBrief {
            name: self.name.clone(),
            severity: self.severity,
            mod_time: self.mod_time,
            _type: self._type.clone(),
        }
    }
}

impl Tabled for Alert {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let mut param = vec![];
        for (k, v) in self.param.iter() {
            param.push(format!("{}: {}", k, v));
        }
        let mut param2 = vec![];
        for (k, v) in self.param2.iter() {
            param2.push(format!("{}: {}", k, v));
        }
        vec![
            self.apply_all.to_string(),
            self.author.clone(),
            self.categories.join("\n"),
            self.cc.join("\n"),
            self.description.clone(),
            self.disabled.to_string(),
            self.field_name.clone(),
            self.field_name2.clone(),
            self.field_op.clone(),
            self.id.to_string(),
            self.interval_length.to_string(),
            self.mod_time.to_string(),
            self.name.clone(),
            self.notify_snmp.to_string(),
            self.object_type.clone(),
            self.operand.clone(),
            self.operator.clone(),
            param.join("\n"),
            param2.join("\n"),
            self.protocols.join("\n"),
            self.refire_interval.to_string(),
            self.severity.to_string(),
            self.stat_name.clone(),
            self._type.clone(),
            self.units.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "apply_all".to_string(),
            "author".to_string(),
            "categories".to_string(),
            "cc".to_string(),
            "description".to_string(),
            "disabled".to_string(),
            "field_name".to_string(),
            "field_name2".to_string(),
            "field_op".to_string(),
            "id".to_string(),
            "interval_length".to_string(),
            "mod_time".to_string(),
            "name".to_string(),
            "notify_snmp".to_string(),
            "object_type".to_string(),
            "operand".to_string(),
            "operator".to_string(),
            "param".to_string(),
            "param2".to_string(),
            "protocols".to_string(),
            "refire_interval".to_string(),
            "severity".to_string(),
            "stat_name".to_string(),
            "type".to_string(),
            "units".to_string(),
        ]
    }
}
