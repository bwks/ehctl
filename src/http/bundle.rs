use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::bundle::Bundles;

use anyhow::Result;

pub async fn get_bundles(client: &ExtraHopClient) -> Result<Bundles> {
    let response = reqwest_get(client, "bundles").await?;
    let bundles = Bundles {
        bundles: serde_json::from_str(&response.text().await?)?,
    };
    Ok(bundles)
}