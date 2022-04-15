use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PacketCaptures {
    pub packet_captures: Vec<PacketCapture>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct PacketCapture {
    #[serde(deserialize_with = "null_to_default")]
    pub bytes: i64,
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
    pub ipproto: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub l7proto: String,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub pkts: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub port1: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub port2: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub snaplen: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub tend_usec: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub tstart_usec: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub vlanid: i64,
}
