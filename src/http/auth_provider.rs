use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::auth_provider::{IdentitiyProviders, SamlSps};

use anyhow::Result;

pub async fn get_identitiy_providers(client: &ExtraHopClient) -> Result<IdentitiyProviders> {
    let response = reqwest_get(client, "/auth/identityproviders").await?;
    let identity_providers = IdentitiyProviders {
        identity_providers: serde_json::from_str(&response.text().await?)?,
    };
    Ok(identity_providers)
}

pub async fn get_saml_sp(client: &ExtraHopClient) -> Result<SamlSps> {
    let response = reqwest_get(client, "/auth/samlsp").await?;
    let saml_sps = SamlSps {
        saml_sps: serde_json::from_str(&response.text().await?)?,
    };
    Ok(saml_sps)
}
