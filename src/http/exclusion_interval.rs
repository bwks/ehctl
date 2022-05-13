use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::exclusion_interval::ExclusionIntervals;

use anyhow::Result;

pub async fn get_exclusion_intervals(client: &ExtraHopClient) -> Result<ExclusionIntervals> {
    let response = reqwest_get(client, "exclusionintervals").await?;
    let exclusion_intervals = ExclusionIntervals {
        exclusion_intervals: serde_json::from_str(&response.text().await?)?,
    };
    Ok(exclusion_intervals)
}