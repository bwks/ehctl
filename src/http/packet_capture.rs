use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::packet_capture::PacketCaptures;

use anyhow::Result;

pub async fn get_packet_captures(client: &ExtraHopClient) -> Result<PacketCaptures> {
    let response = reqwest_get(client, "packetcaptures").await?;
    let packet_captures = PacketCaptures {
        packet_captures: serde_json::from_str(&response.text().await?)?,
    };
    Ok(packet_captures)
}
