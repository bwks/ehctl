use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Softwares {
    pub softwares: Vec<Software>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct Software {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub software_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub version: String,
}
