use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::node::Nodes;

use anyhow::Result;

pub async fn get_nodes(client: &ExtraHopClient) -> Result<Nodes> {
    let response = reqwest_get(client, "nodes").await?;
    let nodes = Nodes {
        nodes: serde_json::from_str(&response.text().await?)?,
    };
    Ok(nodes)
}
