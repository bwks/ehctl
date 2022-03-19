use clap::{Arg, Command};

#[derive(Eq, PartialEq)]
pub enum Getter {
    Appliances,
    Bundles,
    Config,
    Customizations,
    Devices,
    Extrahop,
    Licenses,
    Nodes,
    Networks,
    NetworkLocalities,
    Tags,
    ThreatCollections,
    Vlans,
    None,
}

pub struct CLI {
    pub backup: bool,
    pub getter: Getter,
}

impl CLI {
    fn default() -> Self {
        Self {
            backup: false,
            getter: Getter::None,
        }
    }
    pub fn new() -> Self {
        let app = Command::new("ehopctl")
            .version("0.1.5")
            .about("Extrahop CLI");

        // Define the name command line option
        let get_option = Arg::new("get-endpoint")
            .long("get") // allow --get
            .short('g')
            .takes_value(true)
            .help("Get [options...]")
            .required(false);

        let backup_option = Arg::new("backup")
            .long("backup") // allow --get
            .takes_value(false)
            .help("Backup customizations")
            .required(false);

        let app = app.arg(get_option).arg(backup_option);

        let options = app.get_matches();

        let getter = options.value_of("get-endpoint").unwrap_or("none");

        let backup = options.is_present("backup");

        let mut cli = CLI::default();

        cli.backup = backup;

        cli.getter = match getter {
            "appliances" => Getter::Appliances,
            "bundles" => Getter::Bundles,
            "config" => Getter::Config,
            "customizations" => Getter::Customizations,
            "devices" => Getter::Devices,
            "extrahop" => Getter::Extrahop,
            "licenses" => Getter::Licenses,
            "networks" => Getter::Networks,
            "networklocalities" => Getter::NetworkLocalities,
            "nodes" => Getter::Nodes,
            "tags" => Getter::Tags,
            "threatcollections" => Getter::ThreatCollections,
            "vlans" => Getter::Vlans,
            _ => Getter::None,
        };
        cli
    }
}
