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

impl Tabled for CustomDevice {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let mut criterias = vec![];
        for c in self.criteria.iter() {
            let tmp = format!(
                "dst_port_max:     {}
dst_port_min:     {}
ipaddr:           {}
ipaddr_direction: {}
ipaddr_peer:      {}
src_port_max:     {}
src_port_min:     {}
vlan_max:         {}
vlan_min:         {}
--",
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
        vec![
            format!("{}", self.author),
            format!("{}", self.description),
            format!("{}", self.disabled),
            format!("{}", self.extrahop_id),
            format!("{}", self.id),
            format!("{}", self.mod_time),
            format!("{}", self.name),
            criterias.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            String::from("author"),
            String::from("description"),
            String::from("disabled"),
            String::from("extrahop_id"),
            String::from("id"),
            String::from("mod_time"),
            String::from("name"),
            String::from("criteria"),
        ]
    }
}
