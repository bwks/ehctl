use clap::{Arg, Command};

#[derive(Eq, PartialEq)]
pub enum Getter {
    Appliances,
    Config,
    Customizations,
    Devices,
    Extrahop,
    Networks,
    Tags,
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
            .version("0.1.4")
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

        match getter {
            "appliances" => cli.getter = Getter::Appliances,
            "config" => cli.getter = Getter::Config,
            "customizations" => cli.getter = Getter::Customizations,
            "devices" => cli.getter = Getter::Devices,
            "extrahop" => cli.getter = Getter::Extrahop,
            "networks" => cli.getter = Getter::Networks,
            "tags" => cli.getter = Getter::Tags,
            "vlans" => cli.getter = Getter::Vlans,
            _ => cli.getter = Getter::None,
        }
        Self { ..cli }
    }
}
