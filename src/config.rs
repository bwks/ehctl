use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;
use toml;

#[derive(Debug, Deserialize)]
pub struct ExtraHopConfig {
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
    pub allow_insecure_tls: bool,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub api_key: String,
}

impl ExtraHopConfig {
    pub fn new() -> Self {
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
        let mut config: Self = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("unable to load data from {}, got error {}", &filename, e);
                exit(1);
            }
        };
        get_credentials(&mut config.eda);
        get_credentials(&mut config.eca);
        get_credentials(&mut config.exa);
        return config;
    }
}

/// Loads the credentials from environment variables 
/// if the credentials are not defined.
fn get_credentials(ehc: &mut Vec<ExtraHopCredential>) {
    for mut i in ehc.iter_mut() {
        if i.user_id == s!("") {
            i.user_id = get_env_var(format!("{}_USER_ID", i.hostname));
        }
        if i.api_key == s!("") {
            i.api_key = get_env_var(format!("{}_API_KEY", i.hostname));
        }
    }
}

/// Returns an environment variable or an empty string if 
/// the environment variable was not found.
fn get_env_var(s: String) -> String {
    let ts = to_env_var(s);
    let var = match env::var(&ts) {
        Ok(c) => c,
        Err(_) => s!(""),
    };
    var
}

/// Convert a string to an environment variable compatible string.
/// Replaces dashes (-) and dots (.) with underscores (_)
/// and transforms to UPPERCASE.
fn to_env_var(s: String) -> String {
    let var = s.replace("-", "_").replace(".", "_").to_uppercase();
    var
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_env_var() {
        assert_eq!(
            to_env_var(s!("THIS_is-A.TEST")),
            s!("THIS_IS_A_TEST"),
        );
    }
}
