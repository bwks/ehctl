use clap::Args;

#[derive(Args)]
pub struct Get {
    #[clap(help = "the uri endpoint to get")]
    pub endpoint: String,

    #[clap(short, long, help = "Verbose output")]
    pub detail: bool,
}
