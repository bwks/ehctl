use serde::Deserialize;

use tabled::Tabled;

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

impl Tabled for Device {
    const LENGTH: usize = 42;

    fn fields(&self) -> Vec<String> {
        let mut activities = vec![];
        for a in self.activity.iter() {
            let tmp = String::from(a);
            activities.push(tmp);
        }
        vec![
            self.analysis.to_string(),
            format!("{}", self.analysis_level),
            self.auto_role.to_string(),
            self.cdp_name.to_string(),
            self.cloud_account.to_string(),
            self.cloud_instance_id.to_string(),
            self.cloud_instance_name.to_string(),
            self.cloud_instance_type.to_string(),
            self.critical.to_string(),
            self.custom_criticality.to_string(),
            self.custom_make.to_string(),
            self.custom_model.to_string(),
            self.custom_name.to_string(),
            self.custom_type.to_string(),
            self.default_name.to_string(),
            self.description.to_string(),
            self.device_class.to_string(),
            self.dhcp_name.to_string(),
            self.discover_time.to_string(),
            self.discovery_id.to_string(),
            self.display_name.to_string(),
            self.dns_name.to_string(),
            self.extrahop_id.to_string(),
            self.id.to_string(),
            self.ipaddr4.to_string(),
            self.ipaddr6.to_string(),
            format!("{}", self.is_l3),
            format!("{}", self.last_seen_time),
            self.macaddr.to_string(),
            format!("{}", self.mod_time),
            self.model.to_string(),
            self.model_override.to_string(),
            self.netbios_name.to_string(),
            format!("{}", self.node_id),
            format!("{}", self.on_watchlist),
            format!("{}", self.parent_id),
            self.role.to_string(),
            self.subnet_id.to_string(),
            format!("{}", self.user_mod_time),
            self.vendor.to_string(),
            format!("{}", self.vlanid),
            self.vpc_id.to_string(),
            activities.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            String::from("analysis"),
            String::from("analysis_level"),
            String::from("auto_role"),
            String::from("cdp_name"),
            String::from("cloud_account"),
            String::from("cloud_instance_id"),
            String::from("cloud_instance_name"),
            String::from("cloud_instance_type"),
            String::from("critical"),
            String::from("custom_criticality"),
            String::from("custom_make"),
            String::from("custom_model"),
            String::from("custom_name"),
            String::from("custom_type"),
            String::from("default_name"),
            String::from("description"),
            String::from("device_class"),
            String::from("dhcp_name"),
            String::from("discover_time"),
            String::from("discovery_id"),
            String::from("display_name"),
            String::from("dns_name"),
            String::from("extrahop_id"),
            String::from("id"),
            String::from("ipaddr4"),
            String::from("ipaddr6"),
            String::from("is_l3"),
            String::from("last_seen_time"),
            String::from("macaddr"),
            String::from("mod_time"),
            String::from("model"),
            String::from("model_override"),
            String::from("netbios_name"),
            String::from("node_id"),
            String::from("on_watchlist"),
            String::from("parent_id"),
            String::from("role"),
            String::from("subnet_id"),
            String::from("user_mod_time"),
            String::from("vendor"),
            String::from("vlanid"),
            String::from("vpc_id"),
            String::from("activities"),
        ]
    }
}
