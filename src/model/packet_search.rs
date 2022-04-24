#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PacketSearch {
    pub output: String,
    pub limit_bytes: String,
    pub limit_search_duration: String,
    pub always_return_body: bool,
    pub from: String,
    pub from_ms: u64,
    pub until: String,
    pub until_ms: u64,
    pub bpf: String,
    pub ip1: String,
    pub port1: String,
    pub ip2: String,
    pub port2: String,
}

impl Default for PacketSearch {
    fn default() -> Self {
        Self {
            output: "pcap".to_string(),
            limit_bytes: "100MB".to_string(),
            limit_search_duration: "5m".to_string(),
            always_return_body: false,
            from: "-30m".to_string(),
            from_ms: 0,
            until: "-1ms".to_string(),
            until_ms: 0,
            bpf: "".to_string(),
            ip1: "".to_string(),
            port1: "".to_string(),
            ip2: "".to_string(),
            port2: "".to_string(),
        }
    }
}

impl PacketSearch {
    pub fn new() -> Self {
        Self {
            ..PacketSearch::default()
        }
    }
}
