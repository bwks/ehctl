use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct IdentitiyProviders {
    pub identity_providers: Vec<IdentitiyProvider>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct IdentitiyProvider {
    #[serde(deserialize_with = "null_to_default")]
    pub auto_provision_users: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub entity_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
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

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct SamlSps {
    pub saml_sps: Vec<SamlSp>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct SamlSp {
    #[serde(deserialize_with = "null_to_default")]
    pub acs_url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub entity_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub xml: String,
}
