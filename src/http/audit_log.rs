use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::audit_log::AuditLogs;

use anyhow::Result;

pub async fn get_audit_logs(client: &ExtraHopClient) -> Result<AuditLogs> {
    let response = reqwest_get(client, "auditlog").await?;
    let audit_logs = AuditLogs {
        audit_logs: serde_json::from_str(&response.text().await?)?,
    };
    Ok(audit_logs)
}
