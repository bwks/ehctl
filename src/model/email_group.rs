use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct EmailGroups {
    pub email_groups: Vec<EmailGroup>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct EmailGroup {
    #[serde(deserialize_with = "null_to_default")]
    pub email_addresses: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub group_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub system_notifications: bool,
}

impl Tabled for EmailGroup {
    const LENGTH: usize = 42;

    fn fields(&self) -> Vec<String> {
        vec![
            self.email_addresses.join("\n"),
            self.group_name.to_string(),
            self.id.to_string(),
            self.system_notifications.to_string(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "email_addresses".to_string(),
            "group_name".to_string(),
            "id".to_string(),
            "system_notifications".to_string(),
        ]
    }
}
