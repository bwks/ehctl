use anyhow::Result;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::StatusCode;
use std::process::exit;
use tokio::fs::File;

use crate::http::client::ExtraHopClient;
use crate::http::common::reqwest_get;
use crate::model::firmware::{FirmwarePrevious, FirmwaresNext};
use crate::util::file::file_to_body;

pub async fn get_firmware_next(client: &ExtraHopClient) -> Result<FirmwaresNext> {
    let response = reqwest_get(client, "extrahop/firmware/next").await?;
    let firmwares_next = FirmwaresNext {
        firmware: serde_json::from_str(&response.text().await?)?,
    };
    Ok(firmwares_next)
}

pub async fn get_firmware_previous(client: &ExtraHopClient) -> Result<FirmwarePrevious> {
    let response = reqwest_get(client, "extrahop/firmware/previous").await?;
    let firmware_previous: FirmwarePrevious = serde_json::from_str(&response.text().await?)?;
    Ok(firmware_previous)
}

pub async fn firmware_upload(client: &ExtraHopClient, filename: &str) -> Result<()> {
    let file = File::open(filename).await?;
    let url = format!("{}/extrahop/firmware", client.base_url);

    println!("=> attempting to upload firmware to `{}`", client.hostname);
    let response = client
        .reqwest_client
        .post(url)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.extrahop.firmware"),
        )
        .body(file_to_body(file))
        .send()
        .await?;

    match response.status() {
        StatusCode::CREATED => {
            println!(
                "=> firmware `{}` successfully uploaded and validated",
                filename
            );
        }
        _ => {
            eprintln!("=> unable upload firmware `{}`", filename);
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }

    Ok(())
}

pub async fn firmware_upgrade(client: &ExtraHopClient) -> Result<()> {
    let url = format!("{}/extrahop/firmware/latest/upgrade", client.base_url);
    let body = serde_json::json!({ "restart_after": true, "silent": true });

    println!(
        "=> attempting to upgrade firmware for `{}`",
        client.hostname
    );

    let response = client.reqwest_client.post(url).json(&body).send().await?;

    match response.status() {
        StatusCode::ACCEPTED => {
            println!(
                "=> job was created for upgrading the firmware for `{}`",
                client.hostname
            );
        }
        _ => {
            eprintln!("=> unable upgrade firmware for `{}`", client.hostname);
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }

    Ok(())
}
