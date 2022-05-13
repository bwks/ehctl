use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::device_group::DeviceGroups;

use anyhow::Result;

pub async fn get_device_groups(client: &ExtraHopClient) -> Result<DeviceGroups> {
    let response = reqwest_get(client, "devicegroups").await?;
    let device_groups = DeviceGroups {
        device_groups: serde_json::from_str(&response.text().await?)?,
    };
    Ok(device_groups)
}
