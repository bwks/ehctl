#![allow(dead_code)]

mod config;
use config::load_config;

mod model;
use model::{Appliance, Customization, Device, ExtraHop, RunningConfig};

mod client;
use client::{build_client, ExtraHopClient};

use chrono::{DateTime, Local};
use clap::{App, Arg};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use tabled::{Disable, MaxWidth, Modify, Rotate, Row, Table};

async fn get_appliances(
    client: &ExtraHopClient,
) -> Result<Vec<Appliance>, Box<dyn std::error::Error>> {
    let url = format!("{}/appliances", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == 200 {
        let appliances: Vec<Appliance> = serde_json::from_str(&response.text().await?)?;
        Ok(appliances)
    } else {
        println!("unable to get appliances");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_devices(client: &ExtraHopClient) -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let url = format!("{}/devices", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;

    if response.status() == 200 {
        let devices: Vec<Device> = serde_json::from_str(&response.text().await?)?;
        Ok(devices)
    } else {
        println!("unable to get devices");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

// tokio let's us use "async" on our main function
async fn get_customizations(
    client: &ExtraHopClient,
) -> Result<Vec<Customization>, Box<dyn std::error::Error>> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let url = format!("{}/customizations", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;

    if response.status() == 200 {
        let customizations: Vec<Customization> = serde_json::from_str(&response.text().await?)?;
        Ok(customizations)
    } else {
        println!("unable to get customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

// tokio let's us use "async" on our main function
async fn save_customization(
    client: &ExtraHopClient,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let url = format!("{}/customizations/{}/download", client.base_url, id);
    let response = client.reqwest_client.post(url).send().await?;
    if response.status() == 200 {
        println!("=> downloading customization: {}", name);
        let bytes = response.bytes().await?;
        let filename = format!("{}-{}.zip", client.hostname, client.timestamp);
        let mut wf = File::create(&filename)?;
        wf.write(&bytes)
            .expect("=> error writing customization to file");
        Ok(())
    } else {
        println!("=> unable to get customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn create_customization(client: &ExtraHopClient) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let body = serde_json::json!({ "name": format!("{}", name) });

    println!("=> adding customization: {}", name);
    let url = format!("{}/customizations", client.base_url);
    let response = client.reqwest_client.post(url).json(&body).send().await?;
    if response.status() == 201 {
        println!("=> new customization added: {}", name);
        let customizations = get_customizations(&client).await?;
        for c in customizations.iter() {
            if c.name.starts_with(&name) {
                save_customization(client, c.id).await?;
            }
        }
    } else {
        println!("=> unable to add customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
    Ok(())
}

// tokio let's us use "async" on our main function
async fn get_extrahop(client: &ExtraHopClient) -> Result<ExtraHop, Box<dyn std::error::Error>> {
    let url = format!("{}/extrahop", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;

    if response.status() == 200 {
        let extrahop: ExtraHop = serde_json::from_str(&response.text().await?)?;
        // extrahop.show();
        Ok(extrahop)
    } else {
        println!("unable to get extrahop");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_running_config(client: &ExtraHopClient) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/runningconfig", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;

    if response.status() == 200 {
        let json_data: serde_json::Value = serde_json::from_str(&response.text().await?)?;
        // let json_string = serde_json::to_string_pretty(&json_data)?;

        let config = RunningConfig { json: json_data };

        let filename = format!("{}-{}.json", client.hostname, client.timestamp);
        let wf = serde_json::to_writer(&File::create(&filename)?, &config.json);
        match wf {
            Ok(_) => println!("wrote file: {}", &filename),
            Err(_) => exit(1),
        };
        Ok(())
        // config.show();
    } else {
        println!("unable to get running config");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Move the app section to `cli.rs`
    // basic app information
    let app = App::new("ehopctl")
        .version("1.0")
        .about("Extrahop CLI")
        .author("Brad Searle");

    // Define the name command line option
    let get_option = Arg::new("get-endpoint")
        .long("get") // allow --get
        .short('g')
        .takes_value(true)
        .help("ExtraHop API GET")
        .required(false);

    let backup_option = Arg::new("backup")
        .long("backup") // allow --get
        .takes_value(false)
        .help("Backup ExtraHop customizations")
        .required(false);

    // now add in the argument we want to parse
    let app = app.arg(get_option).arg(backup_option);

    // extract the matches
    let matches = app.get_matches();

    // Extract the actual get
    let get = matches.value_of("get-endpoint").unwrap_or("none");

    let backup = matches.is_present("backup");

    let time_now: DateTime<Local> = Local::now();
    let timestamp = time_now.format("%Y-%m-%d--%H-%M-%S");

    let credentials = load_config();

    let mut results = vec![];
    let mut appliances: HashMap<String, Vec<Appliance>> = HashMap::new();
    let mut customizations: HashMap<String, Vec<Customization>> = HashMap::new();
    let mut devices: HashMap<String, Vec<Device>> = HashMap::new();

    for c in credentials.eda.iter() {
        let api_key = String::from(&c.secret);
        let reqwest_client = build_client(&api_key);

        let client = ExtraHopClient {
            hostname: String::from(&c.hostname),
            user_id: String::from(&c.user_id),
            secret: String::from(&api_key),
            base_url: format!("https://{}/api/v1", &c.hostname),
            reqwest_client: reqwest_client,
            timestamp: timestamp.to_string(),
        };

        if backup {
            create_customization(&client).await?
        }

        if get == "appliances" {
            let result = get_appliances(&client).await?;
            appliances.insert(String::from(&client.hostname), result);
        }

        if get == "customizations" {
            let result = get_customizations(&client).await?;
            customizations.insert(String::from(&client.hostname), result);
        }

        if get == "devices" {
            let result = get_devices(&client).await?;
            devices.insert(String::from(&client.hostname), result);
        }

        if get == "extrahop" {
            // println!("{}: ", client.hostname);
            let result = get_extrahop(&client).await?;
            results.push(result);
        }

        if get == "config" {
            // println!("{}: ", client.hostname);
            _ = get_running_config(&client).await?;
        }
    }

    if get == "customizations" {
        for (key, mut value) in customizations {
            value.sort_by(|a, b| b.id.cmp(&a.id));

            println!("{}:", key);
            let table = Table::new(value);
            println!("{}", table);
        }
    }
    if get == "extrahop" {
        let table = Table::new(results).with(Disable::Column(1..=1));
        println!("{}", table);
    }
    if get == "appliances" {
        for (key, value) in appliances {
            println!("{}:", key);
            for a in value.iter() {
                let table = Table::new(vec![a])
                    .with(
                        Modify::new(Row(1..))
                            // .with(MinWidth::new(50))
                            .with(MaxWidth::wrapping(50)),
                    )
                    .with(Rotate::Left);
                println!("{}", table);
            }
        }
    }
    if get == "devices" {
        for (key, value) in devices {
            println!("{}:", key);
            for d in value.iter() {
                // let table = Table::new(vec![d])
                //     .with(
                //         Modify::new(Row(1..))
                //             // .with(MinWidth::new(50))
                //             .with(MaxWidth::wrapping(50)),
                //     )
                //     .with(Rotate::Left);
                // println!("{}", table);
                println!("{:#?}", d)
            }
        }
    }
    Ok(())
}
