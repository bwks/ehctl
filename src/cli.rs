use crate::getter::{GetterType, Getters};
use crate::model::packet_search::PacketSearch;
use crate::util::print_list;
use clap::{Args, Parser, Subcommand};
use std::process::exit;

pub enum OutputOption {
    Brief,
    Detail,
}


#[derive(Parser)]
#[clap(author, version = "0.1.12", about = "ExtraHop CLI")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Backup device customizations and/or running-config")]
    Backup(Backup),

    #[clap(about = "Get data from a HTTP GET <endpoint>")]
    Get(Get),

    #[clap(about = "Download a packet capture")]
    PacketSearch(PacketSearchCli),

    #[clap(subcommand)]
    #[clap(about = "Show ehctl information")]
    Show(Show),
}

#[derive(Args)]
struct Backup {
    #[clap(help = "Hostname of the device to backup")]
    device: String,
}

#[derive(Args)]
struct Get {
    #[clap(help = "the uri endpoint to get")]
    endpoint: String,

    #[clap(short, long, help = "Verbose output")]
    detail: bool,
}

#[derive(Args, Debug)]
struct PacketSearchCli {
    #[clap(
        long, 
        default_value_t = String::from("pcap"), 
        help="Choices: ( pcap | keylog_txt | zip )"
    )]
    output: String,

    #[clap(
        long, 
        default_value_t = String::from("100MB"), 
        help="Maximum bytes to return"
    )]
    limit_bytes: String,

    #[clap(
        long, 
        default_value_t = String::from("5m"), 
        help="Limit search duration"
    )]
    limit_search_duration: String,

    #[clap(
        long, 
        default_value_t = String::from("-30m"),
        help = "Beginning timestamp")]
    from: String,

    #[clap(
        long,
        default_value_t = 0_u64,
        help = "Beginning timestamp in milliseconds since epoch * 10000"
    )]
    from_ms: u64,

    #[clap(
        long, 
        default_value_t = String::from("-1ms"), 
        help="Ending timestamp"
    )]
    until: String,

    #[clap(
        long,
        default_value_t = 0_u64,
        help = "Ending timestamp in milliseconds since epoch * 10000"
    )]
    until_ms: u64,

    #[clap(
        long, 
        default_value_t = String::from(""),
        help = "Berkeley Packet Filter search"
    )]
    bpf: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified IP address"
    )]
    ip1: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified port"
    )]
    port1: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified IP address"
    )]
    ip2: String,

    #[clap(
        long,
        default_value_t = String::from(""),
        help = "Returns packets sent to or received by the specified port"
    )]
    port2: String,
}

#[derive(Subcommand)]
enum Show {
    #[clap(about = "ExtraHop CLI configuration details")]
    Config(ShowConfig),

    #[clap(about = "List available HTTP GET endpoints")]
    Get(ShowGet),
}

#[derive(Args)]
struct ShowConfig {
    #[clap(
        default_value_t = String::from("all"),
        help = "(Choices: all | ccp | eca | eda | exa | eta)",
    )]
    devices: String,

    #[clap(
        default_value_t = String::from(""),
        help = "Hostname of a device",
    )]
    hostname: String,
}

#[derive(Args)]
struct ShowGet {
    #[clap(help = "(Choices: all | ccp | eca | eda | exa | eta)")]
    _type: String,
}



pub struct CliOpts {
    pub backup: bool,
    pub backup_device: String,
    pub packet_search: bool,
    pub packet_search_options: PacketSearch,
    pub getter: bool,
    pub getter_type: GetterType,
    pub output_option: OutputOption,
}

impl CliOpts {
    fn default() -> Self {
        Self {
            backup: false,
            backup_device: "".to_string(),
            packet_search: false,
            packet_search_options: PacketSearch::default(),
            getter: false,
            getter_type: GetterType::Unknown,
            output_option: OutputOption::Brief,
        }
    }

    pub fn new() -> Self {
        let cli = Cli::parse();

        let mut cli_opts = CliOpts::default();

        match &cli.commands {
            Commands::Backup(backup) => {
                if backup.device == "all" {
                    let dev = backup.device.clone();
                    cli_opts.backup = true;
                    cli_opts.backup_device = dev
                } else {
                    eprintln!("=> unknown device `{}`", backup.device);
                    exit(1)
                }
            },
            Commands::Get(get) => {
                cli_opts.getter = true;
                if get.detail {
                    cli_opts.output_option = OutputOption::Detail
                }
                cli_opts.getter_type = match get.endpoint.as_str() {
                    "activitymaps" => GetterType::ActivityMaps,
                    "auditlog" => GetterType::AuditLogs,
                    "alerts" => GetterType::Alerts,
                    "apikeys" => GetterType::ApiKeys,
                    "appliances" => GetterType::Appliances,
                    "bundles" => GetterType::Bundles,
                    "customizations" => GetterType::Customizations,
                    "customdevices" => GetterType::CustomDevices,
                    "dashboards" => GetterType::Dashboards,
                    "detections" => GetterType::Detections,
                    "devicegroups" => GetterType::DeviceGroups,
                    "devices" => GetterType::Devices,
                    "emailgroups" => GetterType::EmailGroups,
                    "exclusionintervals" => GetterType::ExclusionIntervals,
                    "extrahop" => GetterType::ExtraHop,
                    "identityproviders" => GetterType::IdentityProviders,
                    "license" => GetterType::License,
                    "networks" => GetterType::Networks,
                    "networklocalities" => GetterType::NetworkLocalities,
                    "nodes" => GetterType::Nodes,
                    "packetcaptures" => GetterType::PacketCaptures,
                    "runningconfig" => GetterType::RunningConfig,
                    "samlsp" => GetterType::SamlSp,
                    "software" => GetterType::Software,
                    "tags" => GetterType::Tags,
                    "threatcollections" => GetterType::ThreatCollections,
                    "triggers" => GetterType::Triggers,
                    "vlans" => GetterType::Vlans,
                    _ => {
                        eprintln!("=> unknown endpoint `{}`", get.endpoint);
                        exit(1)
                    }
                }
            },
            Commands::PacketSearch(packet_search) => {
                let options = PacketSearch {
                    always_return_body: false,
                    output: packet_search.output.clone(),
                    from: packet_search.from.clone(),
                    from_ms: packet_search.from_ms,
                    limit_bytes: packet_search.limit_bytes.clone(),
                    limit_search_duration: packet_search.limit_search_duration.clone(),
                    until: packet_search.until.clone(),
                    until_ms: packet_search.until_ms,
                    bpf: packet_search.bpf.clone(),
                    ip1: packet_search.ip1.clone(),
                    port1: packet_search.port1.clone(),
                    ip2: packet_search.ip2.clone(),
                    port2: packet_search.port2.clone(),
                };
                cli_opts.packet_search = true;
                cli_opts.packet_search_options = options;
            },
            Commands::Show(show_command) => {
                match show_command {
                    Show::Config(config) => {
                        match config.devices.as_str() {
                            "all" => {
                                println!("All configs")
                            }
                            "ccp" => {
                                println!("CCP configs")
                            }
                            "eca" => {
                                println!("ECA configs")
                            }
                            "eda" => {
                                println!("EDA configs")
                            }
                            "exa" => {
                                println!("EXA configs")
                            }
                            "eta" => {
                                println!("ETA configs")
                            }
                            _ => {
                                eprintln!("=> unknown device type `{}`", config.devices);
                                exit(1)
                            }
                        }
                        exit(0)
                    }
                    Show::Get(get) => {
                        match get._type.as_str() {
                            "all" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::all())
                            }
                            "ccp" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::ccp())
                            }
                            "eca" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::eca())
                            }
                            "eda" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::eda())
                            }
                            "exa" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::exa())
                            }
                            "eta" => {
                                println!("Available GET endpoints");
                                print_list(&Getters::eta())
                            }
                            _ => {
                                eprintln!("=> unknown device type `{}`", get._type);
                                exit(1)
                            }
                        }
                        exit(0)
                    }
                }
            }
        }

        cli_opts
    }
}
