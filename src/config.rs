use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ExtraHopConfig {
    pub ccp: Vec<ExtraHopCredential>,
    pub eca: Vec<ExtraHopCredential>,
    pub eda: Vec<ExtraHopCredential>,
    pub exa: Vec<ExtraHopCredential>,
    pub eta: Vec<ExtraHopCredential>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ExtraHopCredential {
    pub hostname: String,
    pub allow_insecure_tls: bool,
    pub user_id: String,
    pub api_key: String,
}

impl ExtraHopConfig {
    pub fn new() -> Self {
        // Determine if the `EHCTL_CONFIG` environment variable is set
        let config_path = match env::var("EHCTL_CONFIG") {
            Ok(cp) => cp,
            Err(_) => {
                println!("=> could not access `EHCTL_CONFIG` environment variable");
                "".to_string()
            }
        };

        // Determine the type of OS so we can figure out the default
        // environment variable for the users home directory
        let os = env::consts::OS;
        let home_env = match os {
            "windows" => "HOMEPATH",
            _ => "HOME",
        };
        let path_seperator = match os {
            "windows" => "\\",
            _ => "/",
        };
        let mut home_dir = "".to_string();

        if config_path.is_empty() {
            match env::var(home_env) {
                Ok(hd) => home_dir = hd,
                Err(_) => {
                    println!("=> could not access `{home_env}` environment variable");
                    "".to_string();
                }
            };
        }

        if config_path.is_empty() && home_dir.is_empty() {
            eprintln!("=> unable to determine config file path");
            exit(1)
        };

        let config_file = match config_path.is_empty() {
            true => format!("{home_dir}{path_seperator}.ehctl{path_seperator}config.toml"),
            false => config_path,
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

        match config.ccp == empty_credential {
            true => config.ccp = vec![],
            false => get_credentials(&mut config.ccp),
        }
        match config.eca == empty_credential {
            true => config.eca = vec![],
            false => get_credentials(&mut config.eca),
        }
        match config.eda == empty_credential {
            true => config.eda = vec![],
            false => get_credentials(&mut config.eda),
        }
        match config.exa == empty_credential {
            true => config.exa = vec![],
            false => get_credentials(&mut config.exa),
        }
        match config.eta == empty_credential {
            true => config.eta = vec![],
            false => get_credentials(&mut config.eta),
        }

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
