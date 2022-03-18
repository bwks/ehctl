use std::{collections::HashMap, fmt};

use serde::{Deserialize, Deserializer};
use tabled::Tabled;

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Appliance {
    #[serde(deserialize_with = "null_to_default")]
    pub add_time: i64,
    #[serde(default)]
    #[serde(deserialize_with = "null_to_default")]
    pub advanced_analysis_capacity: i64,
    #[serde(default)]
    #[serde(deserialize_with = "null_to_default")]
    pub analysis_levels_managed: bool,
    #[serde(default)]
    #[serde(deserialize_with = "null_to_default")]
    pub total_capacity: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub fingerprint: String,
    #[serde(deserialize_with = "null_to_default")]
    pub connection_type: String,
    pub data_access: bool,
    #[serde(default)]
    #[serde(deserialize_with = "null_to_default")]
    pub display_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub firmware_version: String,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub license_status: String,
    pub managed_by_local: bool,
    pub manages_local: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub nickname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub platform: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status_message: String,
    #[serde(deserialize_with = "null_to_default")]
    pub sync_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub uuid: String,
}

#[allow(dead_code)]
#[derive(Debug, Tabled, Deserialize)]
pub struct Bundles {
    pub built_in: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub created_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Tabled, Deserialize)]
pub struct Customization {
    pub auto: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub create_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    pub recovered: bool,
}

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Device {
    #[serde(default)]
    // pub activity: Vec<String>,
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

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct ExtraHop {
    #[serde(deserialize_with = "null_to_default")]
    pub display_host: String,
    #[serde(deserialize_with = "null_to_default")]
    pub external_hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub hostname: String,
    #[serde(deserialize_with = "null_to_default")]
    pub mgmt_ipaddr: String,
    #[serde(deserialize_with = "null_to_default")]
    pub platform: String,
    #[serde(deserialize_with = "null_to_default")]
    pub version: String,
}

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct License {
    #[serde(deserialize_with = "null_to_default")]
    pub dossier: String,
    #[serde(deserialize_with = "null_to_default")]
    pub edition: String,
    #[serde(deserialize_with = "null_to_default")]
    pub expires_at: i64,
    // pub modules: LicenseModules,
    // pub options: LicenseOptions,
    pub expires_in: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub platform: String,
    #[serde(deserialize_with = "null_to_default")]
    pub product_key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub serial: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LicenseModules {
    #[serde(deserialize_with = "null_to_default")]
    pub modules: HashMap<String, bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LicenseOptions {
    #[serde(deserialize_with = "null_to_default")]
    pub options: HashMap<String, bool>,
}

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Network {
    #[serde(deserialize_with = "null_to_default")]
    pub appliance_uuid: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub idle: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub node_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RunningConfig {
    // config: serde_json::Value,
    pub json: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Tag {
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
}

#[allow(dead_code)]
#[derive(Tabled, Debug, Deserialize)]
pub struct Vlan {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub network_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub node_id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlanid: i64,
}

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "
analysis: {}
analysis_level: {}
auto_role: {}
cdp_name: {}
cloud_account: {}
cloud_instance_id: {}
cloud_instance_name: {}
cloud_instance_type: {}
critical: {}
custom_criticality: {}
custom_make: {}
custom_model: {}
custom_name: {}
custom_type: {}
default_name: {}
description: {}
device_class: {}
dhcp_name: {}
discover_time: {}
discovery_id: {}
display_name: {}
dns_name: {}
extrahop_id: {}
id: {}
ipaddr4: {}
ipaddr6: {}
is_l3: {}
last_seen_time: {}
macaddr: {}
mod_time: {}
model: {}
model_override: {}
netbios_name: {}
node_id: {}
on_watchlist: {}
parent_id: {}
role: {}
subnet_id: {}
user_mod_time: {}
vendor: {}
vlanid: {}
vpc_id: {}",
            // self.activity.join(" | "),
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
            self.vpc_id
        );
        write!(f, "{}", output)
    }
}
