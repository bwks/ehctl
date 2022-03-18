use serde::Deserialize;
use std::collections::HashMap;
use tabled::Tabled;

use crate::model::null_to_default;

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
