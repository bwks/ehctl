use serde::{Deserialize, Deserializer};

pub mod appliance;
pub mod bundle;
pub mod customization;
pub mod device;
pub mod extrahop;
pub mod license;
pub mod network;
pub mod running_config;
pub mod tag;
pub mod vlan;

pub fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
