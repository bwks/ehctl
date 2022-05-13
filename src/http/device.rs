use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::device::Devices;

use anyhow::Result;

pub async fn get_devices(client: &ExtraHopClient) -> Result<Devices> {
    let response = reqwest_get(client, "devices").await?;
    let devices = Devices {
        devices: serde_json::from_str(&response.text().await?)?,
    };
    Ok(devices)
}
