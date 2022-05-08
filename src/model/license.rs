use serde::Deserialize;
use std::collections::HashMap;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct License {
    #[serde(deserialize_with = "null_to_default")]
    pub dossier: String,
    #[serde(deserialize_with = "null_to_default")]
    pub edition: String,
    #[serde(deserialize_with = "null_to_default")]
    pub expires_at: u64,
    pub modules: HashMap<String, serde_json::Value>,
    pub options: HashMap<String, serde_json::Value>,
    pub expires_in: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub platform: String,
    #[serde(deserialize_with = "null_to_default")]
    pub product_key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub serial: String,
}

impl Tabled for License {
    const LENGTH: usize = 42;

    fn fields(&self) -> Vec<String> {
        let mut modules = vec![];
        for (k, v) in self.modules.iter() {
            let tmp = format!("{k:11}: {v}");
            modules.push(tmp);
        }
        let mut options = vec![];
        for (k, v) in self.options.iter() {
            let tmp = format!("{k:24}: {v}");
            options.push(tmp);
        }

        vec![
            self.dossier.to_string(),
            self.edition.to_string(),
            self.expires_at.to_string(),
            modules.join("\n"),
            options.join("\n"),
            self.expires_in.to_string(),
            self.platform.to_string(),
            self.product_key.to_string(),
            self.serial.to_string(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "dossier".to_string(),
            "edition".to_string(),
            "expires_at".to_string(),
            "modules".to_string(),
            "options".to_string(),
            "expires_in".to_string(),
            "platform".to_string(),
            "product_key".to_string(),
            "serial".to_string(),
        ]
    }
}
