#![allow(dead_code)]
pub enum PacketOutput {
    Pcap,
    Keylog,
    Zip,
}

pub struct PacketSearch {
    pub output: PacketOutput,
    pub limit_bytes: String,
    pub limit_search_duration: String,
    pub always_return_body: bool,
    pub from: String,
    pub until: String,
    pub bpf: String,
    pub ip1: String,
    pub port1: String,
    pub ip2: String,
    pub port2: String,
}

impl Default for PacketSearch {
    fn default() -> Self {
        Self {
            output: PacketOutput::Pcap,
            limit_bytes: "100MB".to_string(),
            limit_search_duration: "5m".to_string(),
            always_return_body: false,
            from: "-6h".to_string(),
            until: "0".to_string(),
            bpf: "".to_string(),
            ip1: "".to_string(),
            port1: "".to_string(),
            ip2: "".to_string(),
            port2: "".to_string(),
        }
    }
}

impl PacketSearch {
    fn new() -> Self {
        Self {
            ..PacketSearch::default()
        }
    }
}
