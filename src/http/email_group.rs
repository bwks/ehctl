use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::email_group::EmailGroups;

use anyhow::Result;

pub async fn get_email_groups(client: &ExtraHopClient) -> Result<EmailGroups> {
    let response = reqwest_get(client, "emailgroups").await?;
    let email_groups = EmailGroups {
        email_groups: serde_json::from_str(&response.text().await?)?,
    };
    Ok(email_groups)
}
