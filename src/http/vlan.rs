use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::vlan::Vlans;

use anyhow::Result;

pub async fn get_vlans(client: &ExtraHopClient) -> Result<Vlans> {
    let response = reqwest_get(client, "vlans").await?;
    let vlans = Vlans {
        vlans: serde_json::from_str(&response.text().await?)?,
    };
    Ok(vlans)
}