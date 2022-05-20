use clap::{Args, Subcommand};

pub enum ShowConfigOption {
    All,
    Ccp,
    Eca,
    Eda,
    Exa,
    Eta,
    None,
}

#[derive(Subcommand)]
pub enum Show {
    #[clap(about = "ExtraHop CLI configuration details")]
    Config(ShowConfig),

    #[clap(about = "List available HTTP GET endpoints")]
    Get(ShowGet),
}

#[derive(Args)]
pub struct ShowConfig {
    #[clap(
        default_value_t = String::from("all"),
        help = "(Choices: all | ccp | eca | eda | exa | eta)",
    )]
    pub devices: String,

    #[clap(
        default_value_t = String::from(""),
        help = "Hostname of a device",
    )]
    pub hostname: String,
}

#[derive(Args)]
pub struct ShowGet {
    #[clap(help = "(Choices: all | ccp | eca | eda | exa | eta)")]
    pub _type: String,
}
