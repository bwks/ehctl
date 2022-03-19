mod config;
use config::ExtraHopConfig;

mod client;
use client::{get_oauth_token, ExtraHopAppliance, ExtraHopClient};

mod cli;
use cli::{Getter, CLI};

mod model;
use model::appliance::Appliance;
use model::bundle::Bundles;
use model::customization::Customization;
use model::device::Device;
use model::extrahop::ExtraHop;
use model::license::License;
use model::network::Network;
use model::network_locality::NetworkLocality;
use model::node::Node;
use model::running_config::RunningConfig;
use model::tag::Tag;
use model::threat_collection::ThreatCollection;
use model::vlan::Vlan;

mod util;

use chrono::Local;
use reqwest::StatusCode;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use tabled::{Disable, MaxWidth, Modify, Rotate, Row, Table};

async fn reqwest_get(
    client: &ExtraHopClient,
    endpoint: &str,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", client.base_url, endpoint);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == StatusCode::OK {
        Ok(response)
    } else {
        println!("unable to get {endpoint}");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_appliances(
    client: &ExtraHopClient,
) -> Result<Vec<Appliance>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "appliances").await?;
    let appliances: Vec<Appliance> = serde_json::from_str(&response.text().await?)?;
    Ok(appliances)
}

async fn get_bundles(client: &ExtraHopClient) -> Result<Vec<Bundles>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "bundles").await?;
    let bundles: Vec<Bundles> = serde_json::from_str(&response.text().await?)?;
    Ok(bundles)
}

async fn get_devices(client: &ExtraHopClient) -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "devices").await?;
    let devices: Vec<Device> = serde_json::from_str(&response.text().await?)?;
    Ok(devices)
}

async fn get_customizations(
    client: &ExtraHopClient,
) -> Result<Vec<Customization>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "customizations").await?;
    let customizations: Vec<Customization> = serde_json::from_str(&response.text().await?)?;
    Ok(customizations)
}

async fn create_customization(client: &ExtraHopClient) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let body = serde_json::json!({ "name": format!("{}", name) });

    println!("=> adding customization: {}", name);
    let url = format!("{}/customizations", client.base_url);
    let response = client.reqwest_client.post(url).json(&body).send().await?;
    if response.status() == StatusCode::CREATED {
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

async fn save_customization(
    client: &ExtraHopClient,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let url = format!("{}/customizations/{}/download", client.base_url, id);
    let response = client.reqwest_client.post(url).send().await?;
    if response.status() == StatusCode::OK {
        println!("=> downloading customization: {}", name);
        let bytes = response.bytes().await?;
        let filename = format!("{}-{}.zip", client.hostname, client.timestamp);
        let mut wf = File::create(&filename)?;
        wf.write(&bytes)
            .expect("=> error writing customization to file");
        Ok(())
    } else {
        println!("=> unable to save customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_extrahop(client: &ExtraHopClient) -> Result<ExtraHop, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "extrahop").await?;
    let extrahop: ExtraHop = serde_json::from_str(&response.text().await?)?;
    Ok(extrahop)
}

async fn get_running_config(client: &ExtraHopClient) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "runningconfig").await?;
    if response.status() == StatusCode::OK {
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
    let response = reqwest_get(&client, "license").await?;
    let licenses: License = serde_json::from_str(&response.text().await?)?;
    Ok(vec![licenses])
}

async fn get_networks(client: &ExtraHopClient) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "networks").await?;
    let networks: Vec<Network> = serde_json::from_str(&response.text().await?)?;
    Ok(networks)
}

async fn get_network_localities(client: &ExtraHopClient) -> Result<Vec<NetworkLocality>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "networklocalities").await?;
    let network_localities: Vec<NetworkLocality> = serde_json::from_str(&response.text().await?)?;
    Ok(network_localities)
}

async fn get_nodes(client: &ExtraHopClient) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "nodes").await?;
    let nodes: Vec<Node> = serde_json::from_str(&response.text().await?)?;
    Ok(nodes)
}

async fn get_tags(client: &ExtraHopClient) -> Result<Vec<Tag>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "tags").await?;
    let tags: Vec<Tag> = serde_json::from_str(&response.text().await?)?;
    Ok(tags)
}

async fn get_threat_collections(
    client: &ExtraHopClient,
) -> Result<Vec<ThreatCollection>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "threatcollections").await?;
    let threat_collections: Vec<ThreatCollection> = serde_json::from_str(&response.text().await?)?;
    Ok(threat_collections)
}

async fn get_vlans(client: &ExtraHopClient) -> Result<Vec<Vlan>, Box<dyn std::error::Error>> {
    let response = reqwest_get(&client, "vlans").await?;
    let vlans: Vec<Vlan> = serde_json::from_str(&response.text().await?)?;
    Ok(vlans)
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
            Getter::Bundles,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Networks,
            Getter::NetworkLocalities,
            Getter::Tags,
            Getter::ThreatCollections,
            Getter::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::ECA,
        vec![
            Getter::Appliances,
            Getter::Bundles,
            Getter::Config,
            Getter::Customizations,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Licenses,
            Getter::Networks,
            Getter::NetworkLocalities,
            Getter::Nodes,
            Getter::Tags,
            Getter::ThreatCollections,
            Getter::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::EDA,
        vec![
            Getter::Appliances,
            Getter::Bundles,
            Getter::Config,
            Getter::Customizations,
            Getter::Devices,
            Getter::Extrahop,
            Getter::Licenses,
            Getter::Networks,
            Getter::NetworkLocalities,
            Getter::Tags,
            Getter::ThreatCollections,
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
    let mut bundles: HashMap<String, Vec<Bundles>> = HashMap::new();
    let mut customizations: HashMap<String, Vec<Customization>> = HashMap::new();
    let mut devices: HashMap<String, Vec<Device>> = HashMap::new();
    let mut licenses: HashMap<String, Vec<License>> = HashMap::new();
    let mut networks: HashMap<String, Vec<Network>> = HashMap::new();
    let mut network_localities: HashMap<String, Vec<NetworkLocality>> = HashMap::new();
    let mut nodes: HashMap<String, Vec<Node>> = HashMap::new();
    let mut tags: HashMap<String, Vec<Tag>> = HashMap::new();
    let mut threat_collections: HashMap<String, Vec<ThreatCollection>> = HashMap::new();
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
                Getter::Bundles => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_bundles(&c).await?;
                        bundles.insert(String::from(&c.hostname), result);
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
                Getter::NetworkLocalities => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_network_localities(&c).await?;
                        network_localities.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Nodes => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_nodes(&c).await?;
                        nodes.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::Tags => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_tags(&c).await?;
                        tags.insert(String::from(&c.hostname), result);
                    }
                }
                Getter::ThreatCollections => {
                    if getter_map[&c.appliance_type].contains(&cli.getter) {
                        let result = get_threat_collections(&c).await?;
                        threat_collections.insert(String::from(&c.hostname), result);
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
            Getter::Bundles => {
                for (key, mut value) in bundles {
                    value.sort_by(|a, b| a.name.cmp(&b.name));

                    println!("=> {}:", key);
                    let table = Table::new(value).with(
                        Modify::new(Row(1..))
                            // .with(MinWidth::new(50))
                            .with(MaxWidth::wrapping(50)),
                    );
                    println!("{table}");
                }
            }
            Getter::Customizations => {
                for (key, mut value) in customizations {
                    value.sort_by(|a, b| a.id.cmp(&b.id));

                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Devices => {
                for (key, value) in devices {
                    println!("{}:", key);
                    for d in value.iter() {
                        //     println!("{}", d)
                        // }
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Row(1..))
                                    // Not released yet, will be in future version.
                                    // .with(MinWidth::new(50))
                                    .with(MaxWidth::wrapping(50)),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            Getter::Extrahop => {
                let table = Table::new(extrahops)
                    .with(Disable::Column(1..=1))
                    .with(Rotate::Left);
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
            Getter::NetworkLocalities => {
                for (key, value) in network_localities {
                    println!("{}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Nodes => {
                for (key, value) in nodes {
                    println!("{}:", key);
                    let table = Table::new(value).with(Rotate::Left);
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
            Getter::ThreatCollections => {
                for (key, mut value) in threat_collections {
                    value.sort_by(|a, b| b.name.cmp(&a.name));

                    println!("=> {}:", key);
                    let table = Table::new(value);
                    println!("{table}");
                }
            }
            Getter::Vlans => {
                for (key, mut value) in vlans {
                    value.sort_by(|a, b| a.vlanid.cmp(&b.vlanid));

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
