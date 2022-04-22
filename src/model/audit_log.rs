use serde::Deserialize;
use tabled::Tabled;

use crate::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct AuditLogs {
    pub audit_logs: Vec<AuditLog>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct AuditLog {
    pub body: Body,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub occur_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub time: String,
}

impl Tabled for AuditLog {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let mut details = self.body.details.to_owned();
        details.truncate(50);
        let body = format!(
            "facility:  {}
operation: {}
priority:  {}
user:      {}
details:  {}",
            self.body.facility, self.body.operation, self.body.priority, self.body.user, details,
        );
        vec![
            body,
            self.id.to_string(),
            self.occur_time.to_string(),
            self.time.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "body".to_string(),
            "id".to_string(),
            "occur_time".to_string(),
            "time".to_string(),
        ]
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Body {
    #[serde(deserialize_with = "null_to_default")]
    pub details: String,
    #[serde(deserialize_with = "null_to_default")]
    pub facility: String,
    #[serde(deserialize_with = "null_to_default")]
    pub operation: String,
    #[serde(deserialize_with = "null_to_default")]
    pub priority: String,
    #[serde(deserialize_with = "null_to_default")]
    pub user: String,
}
