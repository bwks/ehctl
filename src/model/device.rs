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
            let tmp = format!("{a}");
            activities.push(tmp);
        }
        vec![
            format!("{}", self.analysis),
            format!("{}", self.analysis_level),
            format!("{}", self.auto_role),
            format!("{}", self.cdp_name),
            format!("{}", self.cloud_account),
            format!("{}", self.cloud_instance_id),
            format!("{}", self.cloud_instance_name),
            format!("{}", self.cloud_instance_type),
            format!("{}", self.critical),
            format!("{}", self.custom_criticality),
            format!("{}", self.custom_make),
            format!("{}", self.custom_model),
            format!("{}", self.custom_name),
            format!("{}", self.custom_type),
            format!("{}", self.default_name),
            format!("{}", self.description),
            format!("{}", self.device_class),
            format!("{}", self.dhcp_name),
            format!("{}", self.discover_time),
            format!("{}", self.discovery_id),
            format!("{}", self.display_name),
            format!("{}", self.dns_name),
            format!("{}", self.extrahop_id),
            format!("{}", self.id),
            format!("{}", self.ipaddr4),
            format!("{}", self.ipaddr6),
            format!("{}", self.is_l3),
            format!("{}", self.last_seen_time),
            format!("{}", self.macaddr),
            format!("{}", self.mod_time),
            format!("{}", self.model),
            format!("{}", self.model_override),
            format!("{}", self.netbios_name),
            format!("{}", self.node_id),
            format!("{}", self.on_watchlist),
            format!("{}", self.parent_id),
            format!("{}", self.role),
            format!("{}", self.subnet_id),
            format!("{}", self.user_mod_time),
            format!("{}", self.vendor),
            format!("{}", self.vlanid),
            format!("{}", self.vpc_id),
            format!("{}", activities.join("\n")),
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
