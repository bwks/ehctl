use clap::{Arg, Command};

#[derive(Eq, PartialEq)]
pub enum Getter {
    ActivityMaps,
    AuditLogs,
    Alerts,
    Appliances,
    ApiKeys,
    Bundles,
    Customizations,
    CustomDevices,
    Dashboards,
    Detections,
    Devices,
    DeviceGroups,
    EmailGroups,
    ExclusionIntervals,
    Extrahop,
    IdentityProviders,
    License,
    Nodes,
    Networks,
    NetworkLocalities,
    PacketCaptures,
    RunningConfig,
    SamlSp,
    Software,
    Tags,
    ThreatCollections,
    Triggers,
    Vlans,
    None,
}

pub enum OutputOption {
    Brief,
    Detail,
}

pub struct Cli {
    pub backup: bool,
    pub backup_device: String,
    pub getter: Getter,
    pub output_option: OutputOption,
}

impl Cli {
    fn default() -> Self {
        Self {
            backup: false,
            backup_device: "".to_string(),
            getter: Getter::None,
            output_option: OutputOption::Brief,
        }
    }
    pub fn new() -> Self {
        let matches = Command::new("ehctl")
            .version("0.1.7")
            .about("Extrahop CLI")
            .subcommand(
                Command::new("backup")
                    .about("backup device customizations (currently `all` devices are backed up)")
                    .arg(
                        Arg::new("device")
                            .help("`all` or `device name` to backup")
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("get")
                    .about("get <endpoint>")
                    .arg(
                        Arg::new("endpoint")
                            .help("the uri endpoint to get")
                            .required(true),
                    )
                    .arg(
                        Arg::new("detail")
                            .long("detail")
                            .takes_value(false)
                            .help("Verbose output"),
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
                    println!("=> unknown device `{device}`");
                    cli.backup = false
                }
            };
        }
        // get
        else if let Some(get_matches) = matches.subcommand_matches("get") {
            if get_matches.is_present("detail") {
                cli.output_option = OutputOption::Detail
            }
            if let Some(getter) = get_matches.value_of("endpoint") {
                cli.getter = match getter {
                    "activitymaps" => Getter::ActivityMaps,
                    "auditlog" => Getter::AuditLogs,
                    "alerts" => Getter::Alerts,
                    "apikeys" => Getter::ApiKeys,
                    "appliances" => Getter::Appliances,
                    "bundles" => Getter::Bundles,
                    "customizations" => Getter::Customizations,
                    "customdevices" => Getter::CustomDevices,
                    "dashboards" => Getter::Dashboards,
                    "detections" => Getter::Detections,
                    "devicegroups" => Getter::DeviceGroups,
                    "devices" => Getter::Devices,
                    "emailgroups" => Getter::EmailGroups,
                    "exclusionintervals" => Getter::ExclusionIntervals,
                    "extrahop" => Getter::Extrahop,
                    "identityproviders" => Getter::IdentityProviders,
                    "license" => Getter::License,
                    "networks" => Getter::Networks,
                    "networklocalities" => Getter::NetworkLocalities,
                    "nodes" => Getter::Nodes,
                    "packetcaptures" => Getter::PacketCaptures,
                    "runningconfig" => Getter::RunningConfig,
                    "samlsp" => Getter::SamlSp,
                    "software" => Getter::Software,
                    "tags" => Getter::Tags,
                    "threatcollections" => Getter::ThreatCollections,
                    "triggers" => Getter::Triggers,
                    "vlans" => Getter::Vlans,
                    _ => {
                        println!("=> unknown endpoint `{}`", getter);
                        Getter::None
                    }
                };
            }
        }
        cli
    }
}
