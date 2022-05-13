use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::running_config::RunningConfig;

use anyhow::Result;
use reqwest::StatusCode;
use std::fs::File;
use std::process::exit;

pub async fn get_running_config(client: &ExtraHopClient) -> Result<()> {
    let response = reqwest_get(client, "runningconfig").await?;

    match response.status() {
        StatusCode::OK => {
            let json_data: serde_json::Value = serde_json::from_str(&response.text().await?)?;

            let config = RunningConfig { json: json_data };

            let filename = format!("{}-{}.json", client.hostname, client.timestamp);
            let wf = serde_json::to_writer(&File::create(&filename)?, &config.json);
            match wf {
                Ok(_) => println!("=> wrote file: {}", &filename),
                Err(_) => exit(1),
            };
            Ok(())
        }
        _ => {
            eprintln!("=> unable to get running config");
            eprintln!("=> {:#?}", response.error_for_status());
            exit(1)
        }
    }
}
