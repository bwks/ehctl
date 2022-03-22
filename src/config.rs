use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct ExtraHopConfig {
    #[serde(default)]
    pub ccp: Vec<ExtraHopCredential>,
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
        let config_file = match env::var("EHCTL_CONFIG") {
            Ok(cf) => cf,
            Err(_) => {
                let home_dir = match env::var("HOME") {
                    Ok(hd) => hd,
                    Err(_) => {
                        println!("=> could not access `EHCTL_CONFIG` environment variable");
                        println!("=> could not access `HOME` environment variable");
                        exit(1);
                    }
                };
                format!("{home_dir}/.ehctl/config.toml")
            }
        };

        let contents = match fs::read_to_string(&config_file) {
            Ok(c) => {
                println!("=> using config file `{config_file}`");
                c
            }
            Err(_) => {
                eprintln!("=> could not read config file `{config_file}`");
                exit(1);
            }
        };
        let mut config: Self = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "=> unable to load data from confg file `{config_file}`, got error `{e}`"
                );
                exit(1);
            }
        };
        get_credentials(&mut config.ccp);
        get_credentials(&mut config.eda);
        get_credentials(&mut config.eca);
        get_credentials(&mut config.exa);
        config
    }
}

/// Loads the credentials from environment variables
/// if the credentials are not defined.
fn get_credentials(ehc: &mut Vec<ExtraHopCredential>) {
    for mut i in ehc.iter_mut() {
        if i.user_id.is_empty() {
            i.user_id = get_env_var(format!("{}_USER_ID", i.hostname));
        }
        if i.api_key.is_empty() {
            i.api_key = get_env_var(format!("{}_API_KEY", i.hostname));
        }
    }
}

/// Returns an environment variable or an empty string if
/// the environment variable was not found.
fn get_env_var(s: String) -> String {
    let ts = to_env_var(s);
    match env::var(&ts) {
        Ok(c) => c,
        Err(_) => String::from(""),
    }
}

/// Convert a string to an environment variable compatible string.
/// Replaces dashes (-) and dots (.) with underscores (_)
/// and transforms to UPPERCASE.
fn to_env_var(s: String) -> String {
    s.replace('-', "_").replace('.', "_").to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_env_var() {
        assert_eq!(
            to_env_var(String::from("THIS_is-A.TEST")),
            String::from("THIS_IS_A_TEST"),
        );
    }
}
