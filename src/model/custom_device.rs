use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Tabled, Deserialize)]
pub struct CustomDevice {
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    // #[serde(deserialize_with = "null_to_default")]
    // pub criteria: Vec<CustomDeviceCriteria>,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub disabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub extrahop_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String
}

#[allow(dead_code)]
#[derive(Debug, Tabled, Deserialize)]
pub struct CustomDeviceCriteria {
    #[serde(deserialize_with = "null_to_default")]
    pub dst_port_max: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub dst_port_min: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr_direction: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr_peer: String,
    #[serde(deserialize_with = "null_to_default")]
    pub src_port_max: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub src_port_min: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlan_max: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlan_min: i64    
}
