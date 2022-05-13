use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::appliance::Appliances;

use anyhow::Result;

pub async fn get_appliances(client: &ExtraHopClient) -> Result<Appliances> {
    let response = reqwest_get(client, "appliances").await?;
    let appliances = Appliances {
        appliances: serde_json::from_str(&response.text().await?)?,
    };
    Ok(appliances)
}