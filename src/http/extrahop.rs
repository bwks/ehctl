use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::extrahop::ExtraHop;

use anyhow::Result;


pub async fn get_extrahop(client: &ExtraHopClient) -> Result<ExtraHop> {
    let response = reqwest_get(client, "extrahop").await?;
    let extrahop: ExtraHop = serde_json::from_str(&response.text().await?)?;
    Ok(extrahop)
}
