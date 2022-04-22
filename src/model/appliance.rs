use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Tabled, Default, Debug, Deserialize)]
#[serde(default)]
pub struct ApplianceBrief {
    pub hostname: String,
    pub id: i64,
    pub firmware_version: String,
    pub platform: String,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Appliances {
    pub appliances: Vec<Appliance>,
}

#[derive(Tabled, Default, Debug, Deserialize)]
#[serde(default)]
pub struct Appliance {
    #[serde(deserialize_with = "null_to_default")]
    pub add_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub advanced_analysis_capacity: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub analysis_levels_managed: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub total_capacity: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub fingerprint: String,
    #[serde(deserialize_with = "null_to_default")]
    pub connection_type: String,
    pub data_access: bool,
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

impl Appliance {
    pub fn brief(&self) -> ApplianceBrief {
        ApplianceBrief {
            hostname: self.hostname.to_owned(),
            id: self.id,
            firmware_version: self.firmware_version.to_owned(),
            platform: self.platform.to_owned(),
        }
    }
}
