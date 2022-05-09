use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::detection::Detections;

use anyhow::Result;

pub async fn get_detections(client: &ExtraHopClient) -> Result<Detections> {
    let response = reqwest_get(client, "detections").await?;
    let detections = Detections {
        detections: serde_json::from_str(&response.text().await?)?,
    };
    Ok(detections)
}
