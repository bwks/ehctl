use clap::{Arg, Command};

pub enum Getters {
    Appliances,
    Config,
    Customizations,
    Devices,
    Extrahop,
    None,
}

pub struct CLI {
    pub backup: bool,
    pub getter: Getters,
}

impl CLI {
    fn default() -> Self {
        Self {
            backup: false,
            getter: Getters::None,
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
            "appliances" => cli.getter = Getters::Appliances,
            "config" => cli.getter = Getters::Config,
            "customizations" => cli.getter = Getters::Customizations,
            "devices" => cli.getter = Getters::Devices,
            "extrahop" => cli.getter = Getters::Extrahop,
            _ => cli.getter = Getters::None,
        }
        Self { ..cli }
    }
}
