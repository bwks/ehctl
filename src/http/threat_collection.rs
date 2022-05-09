use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::threat_collection::ThreatCollections;

use anyhow::Result;

pub async fn get_threat_collections(client: &ExtraHopClient) -> Result<ThreatCollections> {
    let response = reqwest_get(client, "threatcollections").await?;
    let threat_collections = ThreatCollections {
        threat_collections: serde_json::from_str(&response.text().await?)?,
    };
    Ok(threat_collections)
}