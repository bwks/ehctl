use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::network::Networks;

use anyhow::Result;

pub async fn get_networks(client: &ExtraHopClient) -> Result<Networks> {
    let response = reqwest_get(client, "networks").await?;
    let networks = Networks {
        networks: serde_json::from_str(&response.text().await?)?,
    };
    Ok(networks)
}
