use crate::http::client::ExtraHopClient;
use crate::model::packet_search::PacketSearch;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;
use std::process::exit;

pub async fn packet_search(client: &ExtraHopClient, options: &PacketSearch) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let filename = format!("{}.{}", name, options.output);
    let url = format!("{}/packets/search", client.base_url);

    let params = (
        ("always_return_body", options.always_return_body),
        ("bpf", options.bpf.clone()),
        ("from", options.from.clone()),
        ("ip1", options.ip1.clone()),
        ("ip2", options.ip2.clone()),
        ("limit_bytes", options.limit_bytes.clone()),
        (
            "limit_search_duration",
            options.limit_search_duration.clone(),
        ),
        ("output", options.output.clone()),
        ("port1", options.port1.clone()),
        ("port2", options.port2.clone()),
        ("until", options.until.clone()),
    );

    let response = client
        .reqwest_client
        .post(&url)
        .form(&params)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            println!("=> downloading packets to `{}`", &filename);
            let mut file = File::create(&filename)
                .with_context(|| format!("=> failed to create `{}`", &filename))?;

            let mut stream = response.bytes_stream();

            while let Some(item) = stream.next().await {
                let chunk =
                    item.with_context(|| format!("=> error while downloading `{}`", &filename))?;
                file.write_all(&chunk)
                    .with_context(|| format!("=> error while writing to `{}`", &filename))?;
            }
        }
        StatusCode::NO_CONTENT => {
            println!("=> no packets returned from query")
        }
        _ => {
            eprintln!("=> unable to save packets to `{}`", &filename);
            eprintln!("=> {:#?}", response.error_for_status());
            exit(1)
        }
    }
    Ok(())
}
