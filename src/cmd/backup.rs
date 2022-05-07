use clap::Args;

#[derive(Args)]
pub struct Backup {
    #[clap(help = "Hostname of the device to backup")]
    pub device: String,
}
