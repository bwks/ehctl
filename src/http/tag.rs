use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::tag::Tags;

use anyhow::Result;

pub async fn get_tags(client: &ExtraHopClient) -> Result<Tags> {
    let response = reqwest_get(client, "tags").await?;
    let tags = Tags {
        tags: serde_json::from_str(&response.text().await?)?,
    };
    Ok(tags)
}
