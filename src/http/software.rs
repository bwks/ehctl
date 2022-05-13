use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::software::Softwares;

use anyhow::Result;

pub async fn get_software(client: &ExtraHopClient) -> Result<Softwares> {
    let response = reqwest_get(client, "software").await?;
    let software = Softwares {
        softwares: serde_json::from_str(&response.text().await?)?,
    };
    Ok(software)
}