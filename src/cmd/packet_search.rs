use clap::Args;

#[derive(Args, Debug)]
pub struct PacketSearchCli {
    #[clap(
        long, 
        default_value_t = String::from("pcap"), 
        help="Choices: ( pcap | keylog_txt | zip )"
    )]
    pub output: String,

    #[clap(
        long, 
        default_value_t = String::from("100MB"), 
        help="Maximum bytes to return"
    )]
    pub limit_bytes: String,

    #[clap(
        long, 
        default_value_t = String::from("5m"), 
        help="Limit search duration"
    )]
    pub limit_search_duration: String,

    #[clap(
        long, 
        default_value_t = String::from("-30m"),
        help = "Beginning timestamp"
    )]
    pub from: String, 

    #[clap(
        long,
        default_value_t = 0_u64,
        help = "Beginning timestamp in milliseconds since epoch * 10000"
    )]
    pub from_ms: u64,

    #[clap(
        long, 
        default_value_t = String::from("-1ms"), 
        help="Ending timestamp"
    )]
    pub until: String,

    #[clap(
        long,
        default_value_t = 0_u64,
        help = "Ending timestamp in milliseconds since epoch * 10000"
    )]
    pub until_ms: u64,

    #[clap(
        long, 
        default_value_t = String::from(""),
        help = "Berkeley Packet Filter search"
    )]
    pub bpf: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified IP address"
    )]
    pub ip1: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified port"
    )]
    pub port1: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified IP address"
    )]
    pub ip2: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified port"
    )]
    pub port2: String,
}