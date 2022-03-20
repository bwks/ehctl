use serde::Deserialize;
use std::fmt;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Device {
    #[serde(default)]
    pub activity: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub analysis: String,
    #[serde(deserialize_with = "null_to_default")]
    pub analysis_level: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub auto_role: String,
    #[serde(deserialize_with = "null_to_default")]
    pub cdp_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub cloud_account: String,
    #[serde(deserialize_with = "null_to_default")]
    pub cloud_instance_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub cloud_instance_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub cloud_instance_type: String,
    pub critical: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_criticality: String,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_make: String,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_model: String,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub default_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub device_class: String,
    #[serde(deserialize_with = "null_to_default")]
    pub dhcp_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub discover_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub discovery_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub display_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub dns_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub extrahop_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr4: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr6: String,
    pub is_l3: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub last_seen_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub macaddr: String,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub model: String,
    #[serde(deserialize_with = "null_to_default")]
    pub model_override: String,
    #[serde(deserialize_with = "null_to_default")]
    pub netbios_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub node_id: i64,
    pub on_watchlist: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub parent_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
    #[serde(deserialize_with = "null_to_default")]
    pub subnet_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub user_mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vendor: String,
    #[serde(deserialize_with = "null_to_default")]
    pub vlanid: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vpc_id: String,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut activities = vec![];
        for a in self.activity.iter() {
            let tmp = format!(
                "
  {a}"
            );
            activities.push(tmp);
        }

        let output = format!(
            "--
activities:          {}
analysis:            {}
analysis_level:      {}
auto_role:           {}
cdp_name:            {}
cloud_account:       {}
cloud_instance_id:   {}
cloud_instance_name: {}
cloud_instance_type: {}
critical:            {}
custom_criticality:  {}
custom_make:         {}
custom_model:        {}
custom_name:         {}
custom_type:         {}
default_name:        {}
description:         {}
device_class:        {}
dhcp_name:           {}
discover_time:       {}
discovery_id:        {}
display_name:        {}
dns_name:            {}
extrahop_id:         {}
id:                  {}
ipaddr4:             {}
ipaddr6:             {}
is_l3:               {}
last_seen_time:      {}
macaddr:             {}
mod_time:            {}
model:               {}
model_override:      {}
netbios_name:        {}
node_id:             {}
on_watchlist:        {}
parent_id:           {}
role:                {}
subnet_id:           {}
user_mod_time:       {}
vendor:              {}
vlanid:              {}
vpc_id:              {}",
            self.analysis,
            self.analysis_level,
            self.auto_role,
            self.cdp_name,
            self.cloud_account,
            self.cloud_instance_id,
            self.cloud_instance_name,
            self.cloud_instance_type,
            self.critical,
            self.custom_criticality,
            self.custom_make,
            self.custom_model,
            self.custom_name,
            self.custom_type,
            self.default_name,
            self.description,
            self.device_class,
            self.dhcp_name,
            self.discover_time,
            self.discovery_id,
            self.display_name,
            self.dns_name,
            self.extrahop_id,
            self.id,
            self.ipaddr4,
            self.ipaddr6,
            self.is_l3,
            self.last_seen_time,
            self.macaddr,
            self.mod_time,
            self.model,
            self.model_override,
            self.netbios_name,
            self.node_id,
            self.on_watchlist,
            self.parent_id,
            self.role,
            self.subnet_id,
            self.user_mod_time,
            self.vendor,
            self.vlanid,
            self.vpc_id,
            activities.join("\n"),
        );
        write!(f, "{}", output)
    }
}
