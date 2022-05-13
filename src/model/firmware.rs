use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;
use crate::util::time::extrahop_to_human_time;

#[allow(dead_code)]
pub enum FirmwareAction {
    LocalUpload,
    LocalUpgrade,
    UrlUpload,
    UrlUpgrade,
    CloudUpgrade,
    None,
}

pub struct FirmwareOptions {
    pub hostname: String,
    pub filename: String,
    pub action: FirmwareAction,
}

impl Default for FirmwareOptions {
    fn default() -> Self {
        Self {
            hostname: "".to_string(),
            filename: "".to_string(),
            action: FirmwareAction::None,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct FirmwaresNext {
    pub firmware: Vec<FirmwareNext>,
}
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct FirmwareNext {
    pub current_release: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub release: String,
    #[serde(deserialize_with = "null_to_default")]
    pub versions: Vec<String>,
}

impl Tabled for FirmwareNext {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        vec![
            self.current_release.to_string(),
            self.release.to_string(),
            self.versions.join(", "),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "current_release".to_string(),
            "release".to_string(),
            "versions".to_string(),
        ]
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct FirmwarePrevious {
    #[serde(deserialize_with = "null_to_default")]
    pub backup_time: u64,
    pub version: String,
}

impl Tabled for FirmwarePrevious {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        vec![
            extrahop_to_human_time(&self.backup_time),
            self.version.to_owned(),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["backup_time".to_string(), "version".to_string()]
    }
}
