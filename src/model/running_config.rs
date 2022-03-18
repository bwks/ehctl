use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RunningConfig {
    // config: serde_json::Value,
    pub json: serde_json::Value,
}
