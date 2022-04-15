use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ExclusionIntervals {
    pub exclusion_intervals: Vec<ExclusionInterval>,
}

#[derive(Default, Deserialize, Tabled)]
#[serde(default)]
pub struct ExclusionInterval {
    #[serde(deserialize_with = "null_to_default")]
    pub alert_apply_all: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub end: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub interval_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub start: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub trend_apply_all: bool,
}
