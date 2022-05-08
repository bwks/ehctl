use crate::cmd::command::{CliCommand, Commands};
use crate::cmd::firmware::Firmware;
use crate::cmd::show::Show;
use crate::getter::{GetterType, Getters};
use crate::model::firmware::FirmwareOptions;
use crate::model::packet_search::PacketSearch;
use crate::util::print::print_list;
use clap::Parser;
use std::process::exit;

pub enum OutputOption {
    Brief,
    Detail,
}

#[derive(Parser)]
#[clap(author, version = "0.1.13", about = "ExtraHop CLI")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

pub struct CliOptions {
    pub backup_device: String,
    pub command: CliCommand,
    pub firmware_options: FirmwareOptions,
    pub getter_type: GetterType,
    pub output_option: OutputOption,
    pub packet_search_options: PacketSearch,
}

impl CliOptions {
    fn default() -> Self {
        Self {
            backup_device: "".to_string(),
            command: CliCommand::None,
            firmware_options: FirmwareOptions::default(),
            packet_search_options: PacketSearch::default(),
            getter_type: GetterType::Unknown,
            output_option: OutputOption::Brief,
        }
    }

    pub fn new() -> Self {
        let cli = Cli::parse();

        let mut cli_opts = CliOptions::default();

        match &cli.commands {
            Commands::Backup(backup) => match backup.device.as_str() {
                "all" => {
                    let dev = backup.device.clone();
                    cli_opts.command = CliCommand::Backup;
                    cli_opts.backup_device = dev
                }
                _ => {
                    eprintln!("=> unknown device `{}`", backup.device);
                    exit(1)
                }
            },
            Commands::Firmware(firmware_command) => match firmware_command {
                Firmware::Upload(x) => {
                    cli_opts.command = CliCommand::Firmware;
                    cli_opts.firmware_options.hostname = x.hostname.to_string();
                    cli_opts.firmware_options.filename = x.filename.to_string();
                }
                Firmware::Upgrade(x) => {
                    cli_opts.command = CliCommand::Firmware;
                    cli_opts.firmware_options.hostname = x.hostname.to_string();
                }
            },
            Commands::Get(get) => {
                cli_opts.command = CliCommand::Get;
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
                    "firmware-next" => GetterType::FirmwareNext,
                    "firmware-previous" => GetterType::FirmwarePrevious,
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
            }
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
                cli_opts.command = CliCommand::PacketSearch;
                cli_opts.packet_search_options = options;
            }
            Commands::Show(show_command) => match show_command {
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
            },
        }

        cli_opts
    }
}
