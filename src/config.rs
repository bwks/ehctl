use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;
use toml;

#[derive(Debug, Deserialize)]
pub struct ExtraHopCredentials {
    #[serde(default)]
    pub eca: Vec<ExtraHopCredential>,
    #[serde(default)]
    pub eda: Vec<ExtraHopCredential>,
    #[serde(default)]
    pub exa: Vec<ExtraHopCredential>,
}

#[derive(Debug, Deserialize)]
pub struct ExtraHopCredential {
    pub hostname: String,
    pub user_id: String,
    pub secret: String,
    pub allow_insecure_tls: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    appliances: String,
}

pub fn load_config() -> ExtraHopCredentials {
    let home_dir = match env::var("HOME") {
        Ok(c) => c,
        Err(_) => {
            println!("could not access $HOME environment variable");
            exit(1);
        }
    };

    let filename = format!("{}/.config/ehctl/config.toml", &home_dir);

    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("could not read file {}", &filename);
            exit(1);
        }
    };
    let credentials: ExtraHopCredentials = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("unable to load data from {}, got error {}", &filename, e);
            exit(1);
        }
    };
    return credentials;
}
