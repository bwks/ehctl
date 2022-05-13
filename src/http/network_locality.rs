use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::network_locality::NetworkLocalities;

use anyhow::Result;

pub async fn get_network_localities(client: &ExtraHopClient) -> Result<NetworkLocalities> {
    let response = reqwest_get(client, "networklocalities").await?;
    let network_localities = NetworkLocalities {
        network_localities: serde_json::from_str(&response.text().await?)?,
    };
    Ok(network_localities)
}
