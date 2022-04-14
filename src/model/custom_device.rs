use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CustomDevices {
    pub custom_devices: Vec<CustomDevice>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Tabled, Default, Deserialize)]
#[serde(default)]
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
            self.author.to_string(),
            self.description.to_string(),
            self.disabled.to_string(),
            self.extrahop_id.to_string(),
            self.id.to_string(),
            self.mod_time.to_string(),
            self.name.to_string(),
            criterias.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author".to_string(),
            "description".to_string(),
            "disabled".to_string(),
            "extrahop_id".to_string(),
            "id".to_string(),
            "mod_time".to_string(),
            "name".to_string(),
            "criteria".to_string(),
        ]
    }
}
