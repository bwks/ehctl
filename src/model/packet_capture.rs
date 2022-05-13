use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PacketCaptures {
    pub packet_captures: Vec<PacketCapture>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct PacketCapture {
    #[serde(deserialize_with = "null_to_default")]
    pub bytes: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub device_id1: String,
    #[serde(deserialize_with = "null_to_default")]
    pub device_id2: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr1: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipaddr2: String,
    #[serde(deserialize_with = "null_to_default")]
    pub ipproto: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub l7proto: String,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub pkts: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub port1: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub port2: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub snaplen: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub tend_usec: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub tstart_usec: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlanid: u64,
}
