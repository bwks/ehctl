use crate::cmd::backup::Backup;
use crate::cmd::firmware::Firmware;
use crate::cmd::get::Get;
use crate::cmd::packet_search::PacketSearchCli;
use crate::cmd::show::Show;
use clap::Subcommand;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum CliCommand {
    Backup,
    Firmware,
    Get,
    PacketSearch,
    Show,
    None,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Backup device customizations and/or running-config")]
    Backup(Backup),

    #[clap(subcommand)]
    #[clap(about = "Manage appliance firmware")]
    Firmware(Firmware),

    #[clap(about = "Get data from a HTTP GET <endpoint>")]
    Get(Get),

    #[clap(about = "Download a packet capture")]
    PacketSearch(PacketSearchCli),

    #[clap(subcommand)]
    #[clap(about = "Show ehctl information")]
    Show(Show),
}
