use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::dashboard::Dashboards;

use anyhow::Result;

pub async fn get_dashboards(client: &ExtraHopClient) -> Result<Dashboards> {
    let response = reqwest_get(client, "dashboards").await?;
    let dashboards = Dashboards {
        dashboards: serde_json::from_str(&response.text().await?)?,
    };
    Ok(dashboards)
}