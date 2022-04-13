use serde::Deserialize;

use crate::deserialize::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct IdentitiyProviders {
    pub identity_providers: Vec<IdentitiyProvider>,
}

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

impl Default for IdentitiyProvider {
    fn default() -> Self {
        Self {
            auto_provision_users: false,
            enabled: false,
            entity_id: "".to_string(),
            id: 0,
            name: "".to_string(),
            signing_certificate: "".to_string(),
            sso_url: "".to_string(),
            _type: "".to_string(),
        }
    }
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

impl Default for SamlSp {
    fn default() -> Self {
        Self {
            acs_url: "".to_string(),
            entity_id: "".to_string(),
            xml: "".to_string(),
        }
    }
}
