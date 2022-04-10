use serde::Deserialize;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct IdentitiyProvider {
    #[serde(deserialize_with = "null_to_default")]
    pub auto_provision_users: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub entity_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub signing_certificate: String,
    #[serde(deserialize_with = "null_to_default")]
    pub sso_url: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SamlSp {
    #[serde(deserialize_with = "null_to_default")]
    pub acs_url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub entity_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub xml: String,
}
