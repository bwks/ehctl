use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::activity_map::ActivityMaps;

use anyhow::Result;

pub async fn get_activity_maps(client: &ExtraHopClient) -> Result<ActivityMaps> {
    let response = reqwest_get(client, "activitymaps").await?;
    let activity_maps = ActivityMaps {
        activity_maps: serde_json::from_str(&response.text().await?)?,
    };
    Ok(activity_maps)
}
