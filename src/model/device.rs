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
            self.analysis_level.to_string(),
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
            self.is_l3.to_string(),
            self.last_seen_time.to_string(),
            self.macaddr.to_string(),
            self.mod_time.to_string(),
            self.model.to_string(),
            self.model_override.to_string(),
            self.netbios_name.to_string(),
            self.node_id.to_string(),
            self.on_watchlist.to_string(),
            self.parent_id.to_string(),
            self.role.to_string(),
            self.subnet_id.to_string(),
            self.user_mod_time.to_string(),
            self.vendor.to_string(),
            self.vlanid.to_string(),
            self.vpc_id.to_string(),
            activities.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "analysis".to_string(),
            "analysis_level".to_string(),
            "auto_role".to_string(),
            "cdp_name".to_string(),
            "cloud_account".to_string(),
            "cloud_instance_id".to_string(),
            "cloud_instance_name".to_string(),
            "cloud_instance_type".to_string(),
            "critical".to_string(),
            "custom_criticality".to_string(),
            "custom_make".to_string(),
            "custom_model".to_string(),
            "custom_name".to_string(),
            "custom_type".to_string(),
            "default_name".to_string(),
            "description".to_string(),
            "device_class".to_string(),
            "dhcp_name".to_string(),
            "discover_time".to_string(),
            "discovery_id".to_string(),
            "display_name".to_string(),
            "dns_name".to_string(),
            "extrahop_id".to_string(),
            "id".to_string(),
            "ipaddr4".to_string(),
            "ipaddr6".to_string(),
            "is_l3".to_string(),
            "last_seen_time".to_string(),
            "macaddr".to_string(),
            "mod_time".to_string(),
            "model".to_string(),
            "model_override".to_string(),
            "netbios_name".to_string(),
            "node_id".to_string(),
            "on_watchlist".to_string(),
            "parent_id".to_string(),
            "role".to_string(),
            "subnet_id".to_string(),
            "user_mod_time".to_string(),
            "vendor".to_string(),
            "vlanid".to_string(),
            "vpc_id".to_string(),
            "activities".to_string(),
        ]
    }
}
