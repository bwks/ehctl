use std::fmt;

use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CustomDevice {
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    #[serde(deserialize_with = "null_to_default")]
    pub criteria: Vec<CustomDeviceCriteria>,
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
    pub name: String,
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
    pub vlan_min: i64,
}

impl fmt::Display for CustomDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut criterias = vec![];
        for c in self.criteria.iter() {
            let tmp = format!(
                "
  dst_port_max:     {}
  dst_port_min:     {}
  ipaddr:           {}
  ipaddr_direction: {}
  ipaddr_peer:      {}
  src_port_max:     {}
  src_port_min:     {}
  vlan_max:         {}
  vlan_min:         {}",
                c.dst_port_max,
                c.dst_port_min,
                c.ipaddr,
                c.ipaddr_direction,
                c.ipaddr_peer,
                c.src_port_max,
                c.src_port_min,
                c.vlan_max,
                c.vlan_min,
            );
            criterias.push(tmp);
        }
        let output = format!(
            "--
author:      {}
description: {}
disabled:    {}
extrahop_id: {}
id:          {}
mod_time:    {}
name:        {}
criteria:    {}",
            self.author,
            self.description,
            self.disabled,
            self.extrahop_id,
            self.id,
            self.mod_time,
            self.name,
            criterias.join("\n"),
        );
        write!(f, "{}", output)
    }
}

impl fmt::Display for CustomDeviceCriteria {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "
dst_port_max:     {}
dst_port_min:     {}
ipaddr:           {}
ipaddr_direction: {}
ipaddr_peer:      {}
src_port_max:     {}
src_port_min:     {}
vlan_max:         {}
vlan_min:         {}
",
            self.dst_port_max,
            self.dst_port_min,
            self.ipaddr,
            self.ipaddr_direction,
            self.ipaddr_peer,
            self.src_port_max,
            self.src_port_min,
            self.vlan_max,
            self.vlan_min,
        );
        write!(f, "{}", output)
    }
}
