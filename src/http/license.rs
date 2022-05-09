use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::license::License;

use anyhow::Result;

pub async fn get_license(client: &ExtraHopClient) -> Result<License> {
    let response = reqwest_get(client, "license").await?;
    let license: License = serde_json::from_str(&response.text().await?)?;
    Ok(license)
}
