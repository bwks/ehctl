use serde::{Deserialize, Deserializer};
use tabled::Tabled;

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
    is_l3: bool,
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

#[derive(Debug, Deserialize)]
pub struct RunningConfig {
    // config: serde_json::Value,
    pub json: serde_json::Value,
}

impl Appliance {
    fn show(&self) {
        println!("{:#?}", &self)
    }
}

impl Customization {
    fn show(&self) {
        println!("{:#?}", &self)
    }
}

impl Device {
    fn show(&self) {
        println!("{:#?}", &self)
    }
}

impl ExtraHop {
    fn show(&self) {
        println!("{:#?}", &self);
    }
}

impl RunningConfig {
    fn show(&self) {
        println!("{}", &self.json);
    }
}

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
