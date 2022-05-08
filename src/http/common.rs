use crate::client::ExtraHopClient;

use anyhow::Result;
use reqwest::StatusCode;
use std::process::exit;

pub async fn reqwest_get(client: &ExtraHopClient, endpoint: &str) -> Result<reqwest::Response> {
    let url = format!("{}/{}", client.base_url, endpoint);
    let response = client.reqwest_client.get(url).send().await?;

    match response.status() {
        StatusCode::OK => Ok(response),
        _ => {
            eprintln!("unable to get {endpoint}");
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }
}
