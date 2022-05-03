use crate::getter::{GetterType, Getters};
use crate::model::packet_search::PacketSearch;
use crate::util::print_list;
use clap::{Arg, Command};
use std::process::exit;

pub enum OutputOption {
    Brief,
    Detail,
}

pub struct Cli {
    pub backup: bool,
    pub backup_device: String,
    pub packet_search: bool,
    pub packet_search_options: PacketSearch,
    pub getter: bool,
    pub getter_type: GetterType,
    pub output_option: OutputOption,
}

impl Cli {
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
        let matches = Command::new("ehctl")
            .version("0.1.12")
            .about("Extrahop CLI")
            .subcommand(
                Command::new("backup")
                    .about("Backup device customizations (currently `all` devices are backed up)")
                    .arg(
                        Arg::new("device")
                            .help("`all` or `device name` to backup")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("get")
                    .about("Get data from a HTTP GET <endpoint>")
                    .arg(
                        Arg::new("endpoint")
                            .help("the uri endpoint to get")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::new("detail")
                            .help("Verbose output")
                            .long("detail")
                            .takes_value(false)
                            .required(false),
                    )
                    .arg(
                        Arg::new("list")
                            .help("List available getters")
                            .long("list")
                            .takes_value(false)
                            .required(false),
                    ),
            )
            .subcommand(
                Command::new("list")
                    .about("List available HTTP <endpoint> options per device type")
                    .subcommand(Command::new("get")
                    .about("HTTP GET endpoints")
                    .arg(
                        Arg::new("type")
                            .help("Valid options: all, ccp, eca, eda, exa, eta")
                            .takes_value(true)
                            .required(false),
                        )
                    )
            )
            .subcommand(
                Command::new("packet-search")
                    .about("Download a packet capture")
                    .arg(
                        Arg::new("output")
                            .long("output")
                            .help("Output format\nChoices: ( pcap | keylog_txt | zip )\nDefault: pcap")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::new("limit-bytes")
                            .long("limit-bytes")
                            .help("Maximum bytes to return\nDefault: 100MB")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::new("limit-duration")
                            .long("limit-duration")
                            .help("Limit search duration\nDefault: 5m")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::new("from")
                            .long("from")
                            .help("Beginning timestamp IE: -30m *Required*")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("until")
                            .long("until")
                            .help("Ending timestamp\nDefault: -1ms")
                            .required(false)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("bpf")
                            .long("bpf")
                            .help("Berkeley Packet Filter to apply")
                            .required(false)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("ip1")
                            .long("ip1")
                            .help("Returns packets sent to or received by the specified IP address")
                            .required(false)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("port1")
                            .long("port1")
                            .help("Returns packets sent to or received by the specified port")
                            .required(false)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("ip2")
                            .long("ip2")
                            .help("Returns packets sent to or received by the specified IP address")
                            .required(false)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("port2")
                            .long("port2")
                            .help("Returns packets sent to or received by the specified port")
                            .required(false)
                            .takes_value(true),
                    ),
            )
            .get_matches();

        let mut cli = Cli::default();

        // backup
        if let Some(backup_matches) = matches.subcommand_matches("backup") {
            if let Some(device) = backup_matches.value_of("device") {
                if device == "all" {
                    cli.backup = true;
                    cli.backup_device = device.to_string()
                } else {
                    eprintln!("=> unknown device `{device}`");
                    exit(1)
                }
            };
        }
        // list
        if let Some(list_matches) = matches.subcommand_matches("list") {
            if let Some(get_matches) = list_matches.subcommand_matches("get") {
                if let Some(device_type) = get_matches.value_of("type") {
                    match device_type {
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
                            eprintln!("=> unknown device type `{}`", device_type);
                            exit(1)
                        }
                    }
                    exit(0)
                }
            }
        }
        // get
        else if let Some(get_matches) = matches.subcommand_matches("get") {
            cli.getter = true;
            if get_matches.is_present("detail") {
                cli.output_option = OutputOption::Detail
            }
            if let Some(getter) = get_matches.value_of("endpoint") {
                cli.getter_type = match getter {
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
                        eprintln!("=> unknown endpoint `{}`", getter);
                        exit(1)
                    }
                };
            }
        }
        // packet-search
        else if let Some(backup_matches) = matches.subcommand_matches("packet-search") {
            let mut options = PacketSearch::new();
            if let Some(output) = backup_matches.value_of("output") {
                options.output = match output {
                    "pcap" | "keylog_txt" | "zip" => output.to_string(),
                    _ => {
                        eprintln!("=> unknown output type `{output}`");
                        exit(1)
                    }
                }
            }
            if let Some(from) = backup_matches.value_of("from") {
                options.from = from.to_string()
            };
            if let Some(limit_bytes) = backup_matches.value_of("limit-bytes") {
                options.limit_bytes = limit_bytes.to_string()
            };
            if let Some(limit_duration) = backup_matches.value_of("limit-duration") {
                options.limit_search_duration = limit_duration.to_string()
            };
            if let Some(until) = backup_matches.value_of("until") {
                options.until = until.to_string()
            };
            if let Some(bpf) = backup_matches.value_of("bpf") {
                options.bpf = bpf.to_string()
            };
            if let Some(ip1) = backup_matches.value_of("ip1") {
                options.ip1 = ip1.to_string()
            };
            if let Some(port1) = backup_matches.value_of("port1") {
                options.port1 = port1.to_string()
            };
            if let Some(ip2) = backup_matches.value_of("ip2") {
                options.ip2 = ip2.to_string()
            };
            if let Some(port2) = backup_matches.value_of("port2") {
                options.port2 = port2.to_string()
            };
            cli.packet_search = true;
            cli.packet_search_options = options;
        }
        cli
    }
}
