use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::api_key::ApiKeys;

use anyhow::Result;

pub async fn get_api_keys(client: &ExtraHopClient) -> Result<ApiKeys> {
    let response = reqwest_get(client, "apikeys").await?;
    let api_keys = ApiKeys {
        api_keys: serde_json::from_str(&response.text().await?)?,
    };
    Ok(api_keys)
}
