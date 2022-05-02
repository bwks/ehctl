use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ExtraHopConfig {
    pub ccp: Vec<ExtraHopCredential>,
    pub eca: Vec<ExtraHopCredential>,
    pub eda: Vec<ExtraHopCredential>,
    pub exa: Vec<ExtraHopCredential>,
    pub eta: Vec<ExtraHopCredential>,
}

#[derive(Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ExtraHopCredential {
    pub hostname: String,
    pub allow_insecure_tls: bool,
    pub user_id: String,
    pub api_key: String,
}

impl ExtraHopConfig {
    pub fn new() -> Self {
        // TODO: Check if windows or posix environment
        // Set $HOME/$HOMEPATH variable
        // EG1:
        // env::var("EHCTL_CONFIG").ok().unwrap_or("{}{}", homedir, path);

        // EG2:
        // if std::env::var("DOES_EXIST").is_ok() {
        //     println!("will be printed");
        // }
        // if std::env::var("DOES_NOT_EXIST").is_ok() {
        //     println!("will NOT be printed");
        // }
        let config_file = match env::var("EHCTL_CONFIG") {
            Ok(cf) => cf,
            Err(_) => {
                eprintln!("=> could not access `EHCTL_CONFIG` environment variable");
                exit(1);
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

        let mut config: ExtraHopConfig = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "=> unable to load data from config file `{config_file}`, got error `{e}`"
                );
                exit(1);
            }
        };

        let empty_credential = vec![ExtraHopCredential::default()];

        if config.ccp == empty_credential {
            config.ccp = vec![]
        } else {
            get_credentials(&mut config.ccp);
        };
        if config.eca == empty_credential {
            config.eca = vec![]
        } else {
            get_credentials(&mut config.eda);
        };
        if config.eda == empty_credential {
            config.eda = vec![]
        } else {
            get_credentials(&mut config.eca);
        };
        if config.exa == empty_credential {
            config.exa = vec![]
        } else {
            get_credentials(&mut config.exa);
        };
        if config.eta == empty_credential {
            config.eta = vec![]
        } else {
            get_credentials(&mut config.eta);
        };

        config
    }
}

/// Loads the credentials from environment variables
/// if the credentials are not defined.
fn get_credentials(ehc: &mut [ExtraHopCredential]) {
    for mut i in ehc.iter_mut() {
        if i.user_id.is_empty() {
            i.user_id = get_env_var(&format!("{}_USER_ID", i.hostname));
        }
        if i.api_key.is_empty() {
            i.api_key = get_env_var(&format!("{}_API_KEY", i.hostname));
        }
        if i.user_id.is_empty() && i.api_key.is_empty() {
            eprintln!("=> no credentials found for {}", i.hostname);
            exit(1);
        }
    }
}

/// Returns an environment variable or an empty string if
/// the environment variable was not found.
fn get_env_var(s: &str) -> String {
    let ts = to_env_var(s);
    match env::var(&ts) {
        Ok(c) => c,
        Err(_) => "".to_string(),
    }
}

/// Convert a string to an environment variable compatible string.
/// Replaces dashes (-) and dots (.) with underscores (_)
/// and transforms to UPPERCASE.
fn to_env_var(s: &str) -> String {
    s.replace('-', "_").replace('.', "_").to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_env_var() {
        assert_eq!(to_env_var("THIS_is-A.TEST"), "THIS_IS_A_TEST",);
    }
}
