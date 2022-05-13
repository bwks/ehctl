use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::alert::Alerts;

use anyhow::Result;

pub async fn get_alerts(client: &ExtraHopClient) -> Result<Alerts> {
    let response = reqwest_get(client, "alerts").await?;
    let alerts = Alerts {
        alerts: serde_json::from_str(&response.text().await?)?,
    };
    Ok(alerts)
}
