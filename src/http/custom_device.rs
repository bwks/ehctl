use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::custom_device::CustomDevices;

use anyhow::Result;

pub async fn get_custom_devices(client: &ExtraHopClient) -> Result<CustomDevices> {
    let response = reqwest_get(client, "customdevices").await?;
    let custom_devices = CustomDevices {
        custom_devices: serde_json::from_str(&response.text().await?)?,
    };
    Ok(custom_devices)
}
