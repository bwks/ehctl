use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::customization::Customizations;

use anyhow::Result;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;
use std::process::exit;

pub async fn get_customizations(client: &ExtraHopClient) -> Result<Customizations> {
    let response = reqwest_get(client, "customizations").await?;
    let customizations = Customizations {
        customizations: serde_json::from_str(&response.text().await?)?,
    };
    Ok(customizations)
}

pub async fn create_customization(client: &ExtraHopClient) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let body = serde_json::json!({ "name": name.to_string() });

    println!("=> adding customization: {}", name);
    let url = format!("{}/customizations", client.base_url);
    let response = client.reqwest_client.post(url).json(&body).send().await?;

    match response.status() {
        StatusCode::CREATED => {
            println!("=> new customization added: {}", name);
            let customizations = get_customizations(client).await?;
            for c in customizations.customizations.iter() {
                if c.name.starts_with(&name) {
                    save_customization(client, &c.id).await?;
                }
            }
            Ok(())
        }
        _ => {
            eprintln!("=> unable to add customization: {}", name);
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }
}

pub async fn save_customization(client: &ExtraHopClient, id: &u64) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let url = format!("{}/customizations/{}/download", client.base_url, id);
    let response = client.reqwest_client.post(url).send().await?;

    match response.status() {
        StatusCode::OK => {
            println!("=> downloading customization: {}", name);
            let bytes = response.bytes().await?;
            let filename = format!("{}-{}.zip", client.hostname, client.timestamp);
            let mut wf = File::create(&filename)?;
            wf.write_all(&bytes)
                .expect("=> error writing customization to file");
            Ok(())
        }
        _ => {
            eprintln!("=> unable to save customization: {}", name);
            eprintln!("=> {:#?}", response.error_for_status());
            exit(1)
        }
    }
}
