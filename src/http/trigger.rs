use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::trigger::Triggers;

use anyhow::Result;

pub async fn get_triggers(client: &ExtraHopClient) -> Result<Triggers> {
    let response = reqwest_get(client, "triggers").await?;
    let triggers = Triggers {
        triggers: serde_json::from_str(&response.text().await?)?,
    };
    Ok(triggers)
}