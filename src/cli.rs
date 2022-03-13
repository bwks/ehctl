use clap::{Arg, Command};

pub enum Getters {
    Appliances,
    Config,
    Customizations,
    Devices,
    Extrahop,
    None,
}

pub struct CliArgs {
    pub backup: bool,
    pub getter: Getters,
}

impl CliArgs {
    fn default() -> Self {
        Self {
            backup: false,
            getter: Getters::None,
        }
    }
}

pub fn cli() -> CliArgs {
    let app = Command::new("ehopctl")
        .version("1.0")
        .about("Extrahop CLI")
        .author("Brad Searle");

    // Define the name command line option
    let get_option = Arg::new("get-endpoint")
        .long("get") // allow --get
        .short('g')
        .takes_value(true)
        .help("ExtraHop API GET")
        .required(false);

    let backup_option = Arg::new("backup")
        .long("backup") // allow --get
        .takes_value(false)
        .help("Backup ExtraHop customizations")
        .required(false);

    let app = app.arg(get_option).arg(backup_option);

    let options = app.get_matches();

    let getter = options.value_of("get-endpoint").unwrap_or("none");

    let backup = options.is_present("backup");

    let mut cli_args = CliArgs::default();

    cli_args.backup = backup;

    match getter {
        "appliances" => cli_args.getter = Getters::Appliances,
        "config" => cli_args.getter = Getters::Config,
        "customizations" => cli_args.getter = Getters::Customizations,
        "devices" => cli_args.getter = Getters::Devices,
        "extrahop" => cli_args.getter = Getters::Extrahop,
        _ => cli_args.getter = Getters::None,
    }
    cli_args
}
