use serde::{Deserialize, Deserializer};

pub mod activity_map;
pub mod alert;
pub mod api_key;
pub mod appliance;
pub mod auth_provider;
pub mod bundle;
pub mod custom_device;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod device_group;
pub mod email_group;
pub mod extrahop;
pub mod license;
pub mod network;
pub mod network_locality;
pub mod node;
pub mod packet_capture;
pub mod running_config;
pub mod software;
pub mod tag;
pub mod threat_collection;
pub mod trigger;
pub mod vlan;

pub fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
