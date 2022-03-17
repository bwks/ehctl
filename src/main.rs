mod config;
use config::ExtraHopConfig;

mod model;
use model::{
    Appliance, Customization, Device, ExtraHop, License, Network, RunningConfig, Tag, Vlan,
};

mod client;
use client::{get_oauth_token, ExtraHopAppliance, ExtraHopClient};

mod cli;
use cli::{Getter, CLI};

use chrono::Local;
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

        let config = RunningConfig { json: json_data };

        let filename = format!("{}-{}.json", client.hostname, client.timestamp);
        let wf = serde_json::to_writer(&File::create(&filename)?, &config.json);
        match wf {
            Ok(_) => println!("wrote file: {}", &filename),
            Err(_) => exit(1),
        };
        Ok(())
    } else {
        println!("unable to get running config");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_license(client: &ExtraHopClient) -> Result<Vec<License>, Box<dyn std::error::Error>> {
    let url = format!("{}/license", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == 200 {
        let licenses: License = serde_json::from_str(&response.text().await?)?;
        Ok(vec![licenses])
    } else {
        println!("unable to get networks");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_networks(client: &ExtraHopClient) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
    let url = format!("{}/networks", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == 200 {
        let networks: Vec<Network> = serde_json::from_str(&response.text().await?)?;
        Ok(networks)
    } else {
        println!("unable to get networks");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_tags(client: &ExtraHopClient) -> Result<Vec<Tag>, Box<dyn std::error::Error>> {
    let url = format!("{}/tags", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == 200 {
        let tags: Vec<Tag> = serde_json::from_str(&response.text().await?)?;
        Ok(tags)
    } else {
        println!("unable to get tags");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_vlans(client: &ExtraHopClient) -> Result<Vec<Vlan>, Box<dyn std::error::Error>> {
    let url = format!("{}/vlans", client.base_url);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == 200 {
        let vlans: Vec<Vlan> = serde_json::from_str(&response.text().await?)?;
        Ok(vlans)
    } else {
        println!("unable to get vlans");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::new();

    let time_now = Local::now();
    let timestamp = time_now.format("%Y-%m-%d--%H-%M-%S");

    let configs = ExtraHopConfig::new();

    let mut getter_map: HashMap<ExtraHopAppliance, Vec<Getter>> = HashMap::new();
    getter_map.insert(
        ExtraHopAppliance::CCP,
        vec![
            Getter::Appliances,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Networks,
            Getter::Tags,
            Getter::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::ECA,
        vec![
            Getter::Appliances,
            Getter::Config,
            Getter::Customizations,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Licenses,
            Getter::Networks,
            Getter::Tags,
            Getter::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::EDA,
        vec![
            Getter::Appliances,
            Getter::Config,
            Getter::Customizations,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Licenses,
            Getter::Networks,
            Getter::Tags,
            Getter::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::ETA,
        vec![
            Getter::Appliances,
            Getter::Config,
            Getter::Extrahop,
            Getter::Licenses,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::EXA,
        vec![
            Getter::Appliances,
            Getter::Config,
            Getter::Extrahop,
            Getter::Licenses,
        ],
    );

    let mut extrahop_appliaces: Vec<ExtraHopClient> = Vec::new();

    for c in configs.ccp {
        let token = get_oauth_token(&c.hostname, &c.user_id, &c.api_key).await?;

        let client = ExtraHopClient::new(
            String::from(&c.hostname),
            String::from(&c.user_id),
            String::from(&c.api_key),
            format!("https://{}/api/v1", &c.hostname),
            timestamp.to_string(),
            token.access_token,
            c.allow_insecure_tls,
            ExtraHopAppliance::CCP,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.eca {
        let client = ExtraHopClient::new(
            String::from(&c.hostname),
            String::from(&c.user_id),
            String::from(&c.api_key),
            format!("https://{}/api/v1", &c.hostname),
            timestamp.to_string(),
            String::from(""),
            c.allow_insecure_tls,
            ExtraHopAppliance::ECA,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.eda {
        let client = ExtraHopClient::new(
            String::from(&c.hostname),
            String::from(&c.user_id),
            String::from(&c.api_key),
            format!("https://{}/api/v1", &c.hostname),
            timestamp.to_string(),
            String::from(""),
            c.allow_insecure_tls,
            ExtraHopAppliance::EDA,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.exa {
        let client = ExtraHopClient::new(
            String::from(&c.hostname),
            String::from(&c.user_id),
            String::from(&c.api_key),
            format!("https://{}/api/v1", &c.hostname),
            timestamp.to_string(),
            String::from(""),
            c.allow_insecure_tls,
            ExtraHopAppliance::EXA,
        );
        extrahop_appliaces.push(client);
    }

    // for c in configs.eta {
    //     let client = ExtraHopClient::new(
    //         String::from(&c.hostname),
    //         String::from(&c.user_id),
    //         String::from(&c.api_key),
    //         format!("https://{}/api/v1", &c.hostname),
    //         timestamp.to_string(),
    //         String::from(""),
    //         c.allow_insecure_tls,
    //     );
    //     etas.push(client);
    // }

    let mut extrahops = vec![];
    let mut appliances: HashMap<String, Vec<Appliance>> = HashMap::new();
    let mut customizations: HashMap<String, Vec<Customization>> = HashMap::new();
    let mut devices: HashMap<String, Vec<Device>> = HashMap::new();
    let mut licenses: HashMap<String, Vec<License>> = HashMap::new();
    let mut networks: HashMap<String, Vec<Network>> = HashMap::new();
    let mut tags: HashMap<String, Vec<Tag>> = HashMap::new();
    let mut vlans: HashMap<String, Vec<Vlan>> = HashMap::new();

    for c in extrahop_appliaces.iter() {
        if cli.backup {
            if getter_map[&c.appliance_type].contains(&cli.getter) {
                create_customization(&c).await?
            }
        } else {
            match cli.getter {
                Getter::Appliances => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_appliances(&c).await?;
                        appliances.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Config => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        _ = get_running_config(&c).await?;
                    }
                }
                Getter::Customizations => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_customizations(&c).await?;
                        customizations.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Devices => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_devices(&c).await?;
                        devices.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Extrahop => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_extrahop(&c).await?;
                        extrahops.push(result);
                    }
                }
                Getter::Licenses => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_license(&c).await?;
                        licenses.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Networks => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_networks(&c).await?;
                        networks.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Tags => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_tags(&c).await?;
                        tags.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Vlans => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_vlans(&c).await?;
                        vlans.insert(String::from(&c.hostname), result);
                    }
                }
                _ => {
                    println!("unknown endpoint");
                    // exit(1)
                }
            }
        }
    }

    if !cli.backup {
        match cli.getter {
            Getter::Appliances => {
                for (key, value) in appliances {
                    println!("{key}:");
                    for a in value.iter() {
                        let table = Table::new(vec![a])
                            .with(
                                Modify::new(Row(1..))
                                    // .with(MinWidth::new(50))
                                    .with(MaxWidth::wrapping(50)),
                            )
                            .with(Rotate::Left);
                        println!("{table}");
                    }
                }
            }
            Getter::Customizations => {
                for (key, mut value) in customizations {
                    value.sort_by(|a, b| b.id.cmp(&a.id));

                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Devices => {
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
                        println!("{}", d)
                    }
                }
            }
            Getter::Extrahop => {
                let table = Table::new(extrahops).with(Disable::Column(1..=1));
                println!("{table}");
            }
            Getter::Licenses => {
                for (key, value) in licenses {
                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Networks => {
                for (key, value) in networks {
                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Tags => {
                for (key, mut value) in tags {
                    value.sort_by(|a, b| b.name.cmp(&a.name));

                    println!("=> {}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Vlans => {
                for (key, mut value) in vlans {
                    value.sort_by(|a, b| b.vlanid.cmp(&a.vlanid));
                    value.reverse();

                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            _ => {}
        }
    }

    Ok(())
}
