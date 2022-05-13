use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::job::Jobs;

use anyhow::Result;

pub async fn get_jobs(client: &ExtraHopClient) -> Result<Jobs> {
    let response = reqwest_get(client, "jobs").await?;
    let jobs = Jobs {
        jobs: serde_json::from_str(&response.text().await?)?,
    };
    Ok(jobs)
}
