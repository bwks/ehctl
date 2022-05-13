use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Customizations {
    pub customizations: Vec<Customization>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct Customization {
    pub auto: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub create_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    pub recovered: bool,
}
