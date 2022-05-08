use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Firmware {
    #[clap(about = "Upload ExtraHop firmware")]
    Upload(FirmwareUpload),

    #[clap(about = "Upgrade ExtraHop firmware")]
    Upgrade(FirmwareUpgrade),
    // #[clap(about = "Downgrade ExtraHop firmware")]
    // Downgrade {},
}

#[derive(Args)]
pub struct FirmwareUpload {
    #[clap(long, help = "Hostname of appliance")]
    pub hostname: String,

    #[clap(long, help = "Firmware filename")]
    pub filename: String,
}

#[derive(Args)]
pub struct FirmwareUpgrade {
    #[clap(long, help = "Hostname of appliance")]
    pub hostname: String,
}
