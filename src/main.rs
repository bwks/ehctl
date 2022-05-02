mod cli;
mod client;
mod config;
mod deserialize;
mod model;
mod util;

use config::ExtraHopConfig;

use client::{get_oauth_token, ExtraHopAppliance, ExtraHopClient};

use cli::{Cli, GetterType, OutputOption};

use model::activity_map::ActivityMaps;
use model::alert::Alerts;
use model::api_key::ApiKeys;
use model::appliance::Appliances;
use model::audit_log::AuditLogs;
use model::auth_provider::{IdentitiyProviders, SamlSps};
use model::bundle::Bundles;
use model::custom_device::CustomDevices;
use model::customization::Customizations;
use model::dashboard::Dashboards;
use model::detection::Detections;
use model::device::Devices;
use model::device_group::DeviceGroups;
use model::email_group::EmailGroups;
use model::exclusion_interval::ExclusionIntervals;
use model::extrahop::ExtraHop;
use model::license::License;
use model::network::Networks;
use model::network_locality::NetworkLocalities;
use model::node::Nodes;
use model::packet_capture::PacketCaptures;
use model::packet_search::PacketSearch;
use model::running_config::RunningConfig;
use model::software::Softwares;
use model::tag::Tags;
use model::threat_collection::ThreatCollections;
use model::trigger::Triggers;
use model::vlan::Vlans;

use anyhow::{Context, Result};
use chrono::Local;
use futures_util::StreamExt;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use tabled::{Alignment, Disable, Full, MaxWidth, MinWidth, Modify, Rotate, Rows, Table};

async fn reqwest_get(client: &ExtraHopClient, endpoint: &str) -> Result<reqwest::Response> {
    let url = format!("{}/{}", client.base_url, endpoint);
    let response = client.reqwest_client.get(url).send().await?;
    if response.status() == StatusCode::OK {
        Ok(response)
    } else {
        eprintln!("unable to get {endpoint}");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_api_keys(client: &ExtraHopClient) -> Result<ApiKeys> {
    let response = reqwest_get(client, "apikeys").await?;
    let api_keys = ApiKeys {
        api_keys: serde_json::from_str(&response.text().await?)?,
    };
    Ok(api_keys)
}

async fn get_activity_maps(client: &ExtraHopClient) -> Result<ActivityMaps> {
    let response = reqwest_get(client, "activitymaps").await?;
    let activity_maps = ActivityMaps {
        activity_maps: serde_json::from_str(&response.text().await?)?,
    };
    Ok(activity_maps)
}

async fn get_audit_logs(client: &ExtraHopClient) -> Result<AuditLogs> {
    let response = reqwest_get(client, "auditlog").await?;
    let audit_logs = AuditLogs {
        audit_logs: serde_json::from_str(&response.text().await?)?,
    };
    Ok(audit_logs)
}

async fn get_alerts(client: &ExtraHopClient) -> Result<Alerts> {
    let response = reqwest_get(client, "alerts").await?;
    let alerts = Alerts {
        alerts: serde_json::from_str(&response.text().await?)?,
    };
    Ok(alerts)
}

async fn get_appliances(client: &ExtraHopClient) -> Result<Appliances> {
    let response = reqwest_get(client, "appliances").await?;
    let appliances = Appliances {
        appliances: serde_json::from_str(&response.text().await?)?,
    };
    Ok(appliances)
}

async fn get_bundles(client: &ExtraHopClient) -> Result<Bundles> {
    let response = reqwest_get(client, "bundles").await?;
    let bundles = Bundles {
        bundles: serde_json::from_str(&response.text().await?)?,
    };
    Ok(bundles)
}

async fn get_dashboards(client: &ExtraHopClient) -> Result<Dashboards> {
    let response = reqwest_get(client, "dashboards").await?;
    let dashboards = Dashboards {
        dashboards: serde_json::from_str(&response.text().await?)?,
    };
    Ok(dashboards)
}

async fn get_detections(client: &ExtraHopClient) -> Result<Detections> {
    let response = reqwest_get(client, "detections").await?;
    let detections = Detections {
        detections: serde_json::from_str(&response.text().await?)?,
    };
    Ok(detections)
}

async fn get_devices(client: &ExtraHopClient) -> Result<Devices> {
    let response = reqwest_get(client, "devices").await?;
    let devices = Devices {
        devices: serde_json::from_str(&response.text().await?)?,
    };
    Ok(devices)
}

async fn get_device_groups(client: &ExtraHopClient) -> Result<DeviceGroups> {
    let response = reqwest_get(client, "devicegroups").await?;
    let device_groups = DeviceGroups {
        device_groups: serde_json::from_str(&response.text().await?)?,
    };
    Ok(device_groups)
}

async fn get_customizations(client: &ExtraHopClient) -> Result<Customizations> {
    let response = reqwest_get(client, "customizations").await?;
    let customizations = Customizations {
        customizations: serde_json::from_str(&response.text().await?)?,
    };
    Ok(customizations)
}

async fn create_customization(client: &ExtraHopClient) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let body = serde_json::json!({ "name": name.to_string() });

    println!("=> adding customization: {}", name);
    let url = format!("{}/customizations", client.base_url);
    let response = client.reqwest_client.post(url).json(&body).send().await?;
    if response.status() == StatusCode::CREATED {
        println!("=> new customization added: {}", name);
        let customizations = get_customizations(client).await?;
        for c in customizations.customizations.iter() {
            if c.name.starts_with(&name) {
                save_customization(client, &c.id).await?;
            }
        }
    } else {
        eprintln!("=> unable to add customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
    Ok(())
}

async fn save_customization(client: &ExtraHopClient, id: &i64) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let url = format!("{}/customizations/{}/download", client.base_url, id);
    let response = client.reqwest_client.post(url).send().await?;
    if response.status() == StatusCode::OK {
        println!("=> downloading customization: {}", name);
        let bytes = response.bytes().await?;
        let filename = format!("{}-{}.zip", client.hostname, client.timestamp);
        let mut wf = File::create(&filename)?;
        wf.write_all(&bytes)
            .expect("=> error writing customization to file");
        Ok(())
    } else {
        eprintln!("=> unable to save customization: {}", name);
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_custom_devices(client: &ExtraHopClient) -> Result<CustomDevices> {
    let response = reqwest_get(client, "customdevices").await?;
    let custom_devices = CustomDevices {
        custom_devices: serde_json::from_str(&response.text().await?)?,
    };
    Ok(custom_devices)
}

async fn get_email_groups(client: &ExtraHopClient) -> Result<EmailGroups> {
    let response = reqwest_get(client, "emailgroups").await?;
    let email_groups = EmailGroups {
        email_groups: serde_json::from_str(&response.text().await?)?,
    };
    Ok(email_groups)
}

async fn get_exclusion_intervals(client: &ExtraHopClient) -> Result<ExclusionIntervals> {
    let response = reqwest_get(client, "exclusionintervals").await?;
    let exclusion_intervals = ExclusionIntervals {
        exclusion_intervals: serde_json::from_str(&response.text().await?)?,
    };
    Ok(exclusion_intervals)
}

async fn get_extrahop(client: &ExtraHopClient) -> Result<ExtraHop> {
    let response = reqwest_get(client, "extrahop").await?;
    let extrahop: ExtraHop = serde_json::from_str(&response.text().await?)?;
    Ok(extrahop)
}

async fn get_running_config(client: &ExtraHopClient) -> Result<()> {
    let response = reqwest_get(client, "runningconfig").await?;
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
        eprintln!("unable to get running config");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}

async fn get_identitiy_providers(client: &ExtraHopClient) -> Result<IdentitiyProviders> {
    let response = reqwest_get(client, "/auth/identityproviders").await?;
    let identity_providers = IdentitiyProviders {
        identity_providers: serde_json::from_str(&response.text().await?)?,
    };
    Ok(identity_providers)
}

async fn get_license(client: &ExtraHopClient) -> Result<License> {
    let response = reqwest_get(client, "license").await?;
    let license: License = serde_json::from_str(&response.text().await?)?;
    Ok(license)
}

async fn get_networks(client: &ExtraHopClient) -> Result<Networks> {
    let response = reqwest_get(client, "networks").await?;
    let networks = Networks {
        networks: serde_json::from_str(&response.text().await?)?,
    };
    Ok(networks)
}

async fn get_network_localities(client: &ExtraHopClient) -> Result<NetworkLocalities> {
    let response = reqwest_get(client, "networklocalities").await?;
    let network_localities = NetworkLocalities {
        network_localities: serde_json::from_str(&response.text().await?)?,
    };
    Ok(network_localities)
}

async fn get_nodes(client: &ExtraHopClient) -> Result<Nodes> {
    let response = reqwest_get(client, "nodes").await?;
    let nodes = Nodes {
        nodes: serde_json::from_str(&response.text().await?)?,
    };
    Ok(nodes)
}

async fn get_packet_captures(client: &ExtraHopClient) -> Result<PacketCaptures> {
    let response = reqwest_get(client, "packetcaptures").await?;
    let packet_captures = PacketCaptures {
        packet_captures: serde_json::from_str(&response.text().await?)?,
    };
    Ok(packet_captures)
}

async fn packet_search(client: &ExtraHopClient, options: &PacketSearch) -> Result<()> {
    let name = format!("{}-{}", client.hostname, client.timestamp);
    let filename = format!("{}.{}", name, options.output);
    let url = format!("{}/packets/search", client.base_url);

    let params = (
        ("always_return_body", options.always_return_body),
        ("bpf", options.bpf.to_owned()),
        ("from", options.from.to_owned()),
        ("ip1", options.ip1.to_owned()),
        ("ip2", options.ip2.to_owned()),
        ("limit_bytes", options.limit_bytes.to_owned()),
        (
            "limit_search_duration",
            options.limit_search_duration.to_owned(),
        ),
        ("output", options.output.to_owned()),
        ("port1", options.port1.to_owned()),
        ("port2", options.port2.to_owned()),
        ("until", options.until.to_owned()),
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
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }
    Ok(())
}

async fn get_saml_sp(client: &ExtraHopClient) -> Result<SamlSps> {
    let response = reqwest_get(client, "/auth/samlsp").await?;
    let saml_sps = SamlSps {
        saml_sps: serde_json::from_str(&response.text().await?)?,
    };
    Ok(saml_sps)
}

async fn get_software(client: &ExtraHopClient) -> Result<Softwares> {
    let response = reqwest_get(client, "software").await?;
    let software = Softwares {
        softwares: serde_json::from_str(&response.text().await?)?,
    };
    Ok(software)
}

async fn get_tags(client: &ExtraHopClient) -> Result<Tags> {
    let response = reqwest_get(client, "tags").await?;
    let tags = Tags {
        tags: serde_json::from_str(&response.text().await?)?,
    };
    Ok(tags)
}

async fn get_threat_collections(client: &ExtraHopClient) -> Result<ThreatCollections> {
    let response = reqwest_get(client, "threatcollections").await?;
    let threat_collections = ThreatCollections {
        threat_collections: serde_json::from_str(&response.text().await?)?,
    };
    Ok(threat_collections)
}

async fn get_triggers(client: &ExtraHopClient) -> Result<Triggers> {
    let response = reqwest_get(client, "triggers").await?;
    let triggers = Triggers {
        triggers: serde_json::from_str(&response.text().await?)?,
    };
    Ok(triggers)
}

async fn get_vlans(client: &ExtraHopClient) -> Result<Vlans> {
    let response = reqwest_get(client, "vlans").await?;
    let vlans = Vlans {
        vlans: serde_json::from_str(&response.text().await?)?,
    };
    Ok(vlans)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();

    let time_now = Local::now();
    let timestamp = time_now.format("%Y-%m-%d--%H-%M-%S");

    let configs = ExtraHopConfig::new();

    let mut getter_map: HashMap<ExtraHopAppliance, Vec<GetterType>> = HashMap::new();
    getter_map.insert(
        ExtraHopAppliance::CCP,
        vec![
            GetterType::ActivityMaps,
            GetterType::AuditLogs,
            GetterType::Alerts,
            GetterType::Appliances,
            GetterType::Bundles,
            GetterType::Dashboards,
            GetterType::Detections,
            GetterType::Devices,
            GetterType::DeviceGroups,
            GetterType::ExclusionIntervals,
            GetterType::Extrahop,
            GetterType::Networks,
            GetterType::NetworkLocalities,
            GetterType::Tags,
            GetterType::Triggers,
            GetterType::Software,
            GetterType::ThreatCollections,
            GetterType::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::ECA,
        vec![
            GetterType::ActivityMaps,
            GetterType::AuditLogs,
            GetterType::Alerts,
            GetterType::ApiKeys,
            GetterType::Appliances,
            GetterType::Bundles,
            GetterType::Customizations,
            // TODO: This endpoint does not work on ECA
            // API doc says its supported
            // I suspect its required to select the sensor
            // GetterType::CustomDevices,
            GetterType::Dashboards,
            GetterType::Detections,
            GetterType::Devices,
            GetterType::DeviceGroups,
            GetterType::EmailGroups,
            GetterType::ExclusionIntervals,
            GetterType::Extrahop,
            GetterType::IdentityProviders,
            GetterType::License,
            GetterType::Networks,
            GetterType::NetworkLocalities,
            GetterType::Nodes,
            GetterType::RunningConfig,
            GetterType::Software,
            GetterType::Tags,
            GetterType::ThreatCollections,
            GetterType::Triggers,
            GetterType::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::EDA,
        vec![
            GetterType::ActivityMaps,
            GetterType::AuditLogs,
            GetterType::Alerts,
            GetterType::ApiKeys,
            GetterType::Appliances,
            GetterType::Bundles,
            GetterType::Customizations,
            GetterType::CustomDevices,
            GetterType::Dashboards,
            GetterType::Detections,
            GetterType::Devices,
            GetterType::DeviceGroups,
            GetterType::EmailGroups,
            GetterType::ExclusionIntervals,
            GetterType::Extrahop,
            GetterType::IdentityProviders,
            GetterType::License,
            GetterType::Networks,
            GetterType::NetworkLocalities,
            GetterType::PacketCaptures,
            GetterType::RunningConfig,
            GetterType::Software,
            GetterType::Tags,
            GetterType::ThreatCollections,
            GetterType::Triggers,
            GetterType::Vlans,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::ETA,
        vec![
            GetterType::ApiKeys,
            GetterType::Appliances,
            GetterType::Extrahop,
            GetterType::License,
            GetterType::RunningConfig,
        ],
    );
    getter_map.insert(
        ExtraHopAppliance::EXA,
        vec![
            GetterType::ApiKeys,
            GetterType::Appliances,
            GetterType::Extrahop,
            GetterType::License,
            GetterType::RunningConfig,
        ],
    );

    let mut extrahop_appliaces: Vec<ExtraHopClient> = Vec::new();

    for c in configs.ccp {
        if !c.hostname.is_empty() {
            let token = get_oauth_token(&c.hostname, &c.user_id, &c.api_key).await?;
            let base_url = format!("https://{}/api/v1", &c.hostname);
            let client = ExtraHopClient::new(
                &c.hostname,
                &c.user_id,
                &c.api_key,
                &base_url,
                &timestamp.to_string(),
                &token.access_token,
                &c.allow_insecure_tls,
                ExtraHopAppliance::CCP,
            );
            extrahop_appliaces.push(client);
        };
    }

    for c in configs.eca {
        let base_url = format!("https://{}/api/v1", &c.hostname);
        let client = ExtraHopClient::new(
            &c.hostname,
            &c.user_id,
            &c.api_key,
            &base_url,
            &timestamp.to_string(),
            "",
            &c.allow_insecure_tls,
            ExtraHopAppliance::ECA,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.eda {
        let base_url = format!("https://{}/api/v1", &c.hostname);
        let client = ExtraHopClient::new(
            &c.hostname,
            &c.user_id,
            &c.api_key,
            &base_url,
            &timestamp.to_string(),
            "",
            &c.allow_insecure_tls,
            ExtraHopAppliance::EDA,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.exa {
        let base_url = format!("https://{}/api/v1", &c.hostname);
        let client = ExtraHopClient::new(
            &c.hostname,
            &c.user_id,
            &c.api_key,
            &base_url,
            &timestamp.to_string(),
            "",
            &c.allow_insecure_tls,
            ExtraHopAppliance::EXA,
        );
        extrahop_appliaces.push(client);
    }

    for c in configs.eta {
        let base_url = format!("https://{}/api/v1", &c.hostname);
        let client = ExtraHopClient::new(
            &c.hostname,
            &c.user_id,
            &c.api_key,
            &base_url,
            &timestamp.to_string(),
            "",
            &c.allow_insecure_tls,
            ExtraHopAppliance::ETA,
        );
        extrahop_appliaces.push(client);
    }

    let mut activity_maps: HashMap<String, ActivityMaps> = HashMap::new();
    let mut audit_logs: HashMap<String, AuditLogs> = HashMap::new();
    let mut alerts: HashMap<String, Alerts> = HashMap::new();
    let mut api_keys: HashMap<String, ApiKeys> = HashMap::new();
    let mut appliances: HashMap<String, Appliances> = HashMap::new();
    let mut bundles: HashMap<String, Bundles> = HashMap::new();
    let mut customizations: HashMap<String, Customizations> = HashMap::new();
    let mut custom_devices: HashMap<String, CustomDevices> = HashMap::new();
    let mut dashboards: HashMap<String, Dashboards> = HashMap::new();
    let mut detections: HashMap<String, Detections> = HashMap::new();
    let mut devices: HashMap<String, Devices> = HashMap::new();
    let mut device_groups: HashMap<String, DeviceGroups> = HashMap::new();
    let mut email_groups: HashMap<String, EmailGroups> = HashMap::new();
    let mut exclusion_intervals: HashMap<String, ExclusionIntervals> = HashMap::new();
    let mut extrahops: Vec<ExtraHop> = Vec::new();
    let mut identity_providers: HashMap<String, IdentitiyProviders> = HashMap::new();
    let mut licenses: HashMap<String, License> = HashMap::new();
    let mut networks: HashMap<String, Networks> = HashMap::new();
    let mut network_localities: HashMap<String, NetworkLocalities> = HashMap::new();
    let mut nodes: HashMap<String, Nodes> = HashMap::new();
    let mut packet_captures: HashMap<String, PacketCaptures> = HashMap::new();
    let mut saml_sps: HashMap<String, SamlSps> = HashMap::new();
    let mut software: HashMap<String, Softwares> = HashMap::new();
    let mut tags: HashMap<String, Tags> = HashMap::new();
    let mut threat_collections: HashMap<String, ThreatCollections> = HashMap::new();
    let mut triggers: HashMap<String, Triggers> = HashMap::new();
    let mut vlans: HashMap<String, Vlans> = HashMap::new();

    for c in extrahop_appliaces.iter() {
        if cli.backup {
            match c.appliance_type {
                ExtraHopAppliance::ECA | ExtraHopAppliance::EDA => create_customization(c).await?,
                ExtraHopAppliance::EXA | ExtraHopAppliance::ETA => get_running_config(c).await?,
                _ => {}
            }
        } else if cli.packet_search {
            match c.appliance_type {
                ExtraHopAppliance::CCP | ExtraHopAppliance::ECA => {
                    packet_search(c, &cli.packet_search_options).await?
                }
                _ => {}
            }
        } else if cli.getter {
            match cli.getter_type {
                GetterType::ActivityMaps => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_activity_maps(c).await?;
                        activity_maps.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::AuditLogs => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_audit_logs(c).await?;
                        audit_logs.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Alerts => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_alerts(c).await?;
                        alerts.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::ApiKeys => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_api_keys(c).await?;
                        api_keys.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Appliances => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_appliances(c).await?;
                        appliances.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Bundles => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_bundles(c).await?;
                        bundles.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Customizations => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_customizations(c).await?;
                        customizations.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::CustomDevices => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_custom_devices(c).await?;
                        custom_devices.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Dashboards => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_dashboards(c).await?;
                        dashboards.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Detections => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_detections(c).await?;
                        detections.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Devices => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_devices(c).await?;
                        devices.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::DeviceGroups => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_device_groups(c).await?;
                        device_groups.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::EmailGroups => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_email_groups(c).await?;
                        email_groups.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::ExclusionIntervals => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_exclusion_intervals(c).await?;
                        exclusion_intervals.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Extrahop => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_extrahop(c).await?;
                        extrahops.push(result);
                    }
                }
                GetterType::IdentityProviders => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_identitiy_providers(c).await?;
                        identity_providers.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::License => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_license(c).await?;
                        licenses.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Networks => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_networks(c).await?;
                        networks.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::NetworkLocalities => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_network_localities(c).await?;
                        network_localities.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Nodes => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_nodes(c).await?;
                        nodes.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::PacketCaptures => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_packet_captures(c).await?;
                        packet_captures.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::RunningConfig => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        _ = get_running_config(c).await?;
                    }
                }
                GetterType::SamlSp => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_saml_sp(c).await?;
                        saml_sps.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Software => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_software(c).await?;
                        software.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Tags => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_tags(c).await?;
                        tags.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::ThreatCollections => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_threat_collections(c).await?;
                        threat_collections.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Triggers => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_triggers(c).await?;
                        triggers.insert(c.hostname.to_string(), result);
                    }
                }
                GetterType::Vlans => {
                    if getter_map[&c.appliance_type].contains(&cli.getter_type) {
                        let result = get_vlans(c).await?;
                        vlans.insert(c.hostname.to_string(), result);
                    }
                }
                _ => {
                    // Should never get here
                }
            }
        }
    }

    if cli.getter {
        match cli.getter_type {
            GetterType::ActivityMaps => {
                for (key, value) in activity_maps {
                    println!("{key}:");
                    for a in value.activity_maps.iter() {
                        let table = Table::new(vec![a])
                            .with(
                                Modify::new(Rows::new(1..))
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30)),
                            )
                            .with(Rotate::Left);
                        println!("{table}");
                    }
                }
            }
            GetterType::AuditLogs => {
                for (key, value) in audit_logs {
                    println!("{key}:");
                    for a in value.audit_logs.iter() {
                        let table = Table::new(vec![a])
                            .with(
                                Modify::new(Rows::new(1..))
                                    // .with(MinWidth::new(30))
                                    // .with(MaxWidth::wrapping(99)),
                            )
                            .with(Rotate::Left);
                        println!("{table}");
                    }
                }
            }
            GetterType::Alerts => {
                for (key, value) in alerts {
                    println!("{key}:");

                    match cli.output_option {
                        OutputOption::Detail => {
                            for a in value.alerts {
                                let table = Table::new(vec![a])
                                    .with(
                                        Modify::new(Rows::new(1..))
                                            .with(MinWidth::new(30))
                                            .with(MaxWidth::wrapping(50)),
                                    )
                                    .with(Rotate::Left);
                                println!("{table}");
                            }
                        }
                        OutputOption::Brief => {
                            let mut data = Vec::new();
                            for a in value.alerts {
                                data.push(a.brief());
                            }
                            let table = Table::new(data);
                            println!("{table}");
                        }
                    }
                }
            }
            GetterType::ApiKeys => {
                for (key, value) in api_keys {
                    println!("{key}:");
                    for a in value.api_keys.iter() {
                        let table = Table::new(vec![a])
                            .with(
                                Modify::new(Rows::new(1..))
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30)),
                            )
                            .with(Rotate::Left);
                        println!("{table}");
                    }
                }
            }
            GetterType::Appliances => {
                for (key, value) in appliances {
                    println!("{key}:");

                    match cli.output_option {
                        OutputOption::Detail => {
                            for a in value.appliances {
                                let table = Table::new(vec![a])
                                    .with(
                                        Modify::new(Rows::new(1..))
                                            .with(MinWidth::new(30))
                                            .with(MaxWidth::wrapping(30)),
                                    )
                                    .with(Rotate::Left);
                                println!("{table}");
                            }
                        }
                        OutputOption::Brief => {
                            let mut data = Vec::new();
                            for a in value.appliances {
                                data.push(a.brief());
                            }
                            let table = Table::new(data);
                            println!("{table}");
                        }
                    }
                }
            }
            GetterType::Bundles => {
                for (key, mut value) in bundles {
                    value.bundles.sort_by(|a, b| a.name.cmp(&b.name));

                    println!("=> {}:", key);
                    let table = Table::new(value.bundles)
                        .with(
                            Modify::new(Rows::new(1..))
                                .with(MinWidth::new(30))
                                .with(MaxWidth::wrapping(30)),
                        )
                        .with(Rotate::Left);
                    println!("{table}");
                }
            }
            GetterType::Customizations => {
                for (key, mut value) in customizations {
                    value.customizations.sort_by(|a, b| a.id.cmp(&b.id));

                    println!("{}:", key);
                    let table = Table::new(value.customizations);
                    println!("{table}");
                }
            }
            GetterType::CustomDevices => {
                for (key, mut value) in custom_devices {
                    if value.custom_devices.is_empty() {
                        println!("{}:", key);
                        println!(" no custom devices");
                    } else {
                        value.custom_devices.sort_by(|a, b| a.name.cmp(&b.name));

                        println!("{}:", key);
                        let table = Table::new(value.custom_devices)
                            .with(Modify::new(Full).with(Alignment::left()));
                        println!("{table}");
                    }
                }
            }
            GetterType::Dashboards => {
                for (key, value) in dashboards {
                    println!("{}:", key);
                    for d in value.dashboards.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::Detections => {
                for (key, value) in detections {
                    println!("{}:", key);
                    for d in value.detections.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    // .with(MinWidth::new(20))
                                    // .with(MaxWidth::wrapping(100))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::Devices => {
                for (key, value) in devices {
                    println!("{}:", key);
                    match cli.output_option {
                        OutputOption::Detail => {
                            for a in value.devices {
                                let table = Table::new(vec![a])
                                    .with(
                                        Modify::new(Rows::new(1..))
                                            .with(MinWidth::new(30))
                                            .with(MaxWidth::wrapping(30)),
                                    )
                                    .with(Rotate::Left);
                                println!("{table}");
                            }
                        }
                        OutputOption::Brief => {
                            let mut data = Vec::new();
                            for a in value.devices {
                                data.push(a.brief());
                            }
                            let table = Table::new(data);
                            println!("{table}");
                        }
                    }
                }
            }
            GetterType::DeviceGroups => {
                for (key, value) in device_groups {
                    println!("{}:", key);
                    for d in value.device_groups.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::EmailGroups => {
                for (key, value) in email_groups {
                    println!("{}:", key);
                    for d in value.email_groups.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::ExclusionIntervals => {
                for (key, value) in exclusion_intervals {
                    println!("{}:", key);
                    for d in value.exclusion_intervals.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::Extrahop => {
                let table = Table::new(extrahops).with(Disable::Column(1..=1));
                println!("{table}");
            }
            GetterType::IdentityProviders => {
                for (key, value) in identity_providers {
                    println!("{}:", key);
                    let table = Table::new(value.identity_providers);
                    println!("{table}");
                }
            }
            GetterType::License => {
                for (key, value) in licenses {
                    println!("{}:", key);
                    let table = Table::new(vec![value]);
                    println!("{table}");
                }
            }
            GetterType::Networks => {
                for (key, value) in networks {
                    println!("{}:", key);
                    let table = Table::new(value.networks);
                    println!("{table}");
                }
            }
            GetterType::NetworkLocalities => {
                for (key, value) in network_localities {
                    println!("{}:", key);
                    let table = Table::new(value.network_localities);
                    println!("{table}");
                }
            }
            GetterType::Nodes => {
                for (key, value) in nodes {
                    println!("{}:", key);
                    let table = Table::new(value.nodes).with(Rotate::Left);
                    println!("{table}");
                }
            }
            GetterType::SamlSp => {
                for (key, value) in saml_sps {
                    println!("{}:", key);
                    let table = Table::new(value.saml_sps);
                    println!("{table}");
                }
            }
            GetterType::Software => {
                for (key, mut value) in software {
                    value.softwares.sort_by(|a, b| a.name.cmp(&b.name));

                    println!("=> {}:", key);
                    let table = Table::new(value.softwares);
                    println!("{table}");
                }
            }
            GetterType::Tags => {
                for (key, mut value) in tags {
                    value.tags.sort_by(|a, b| b.name.cmp(&a.name));

                    println!("=> {}:", key);
                    let table = Table::new(value.tags);
                    println!("{table}");
                }
            }
            GetterType::ThreatCollections => {
                for (key, mut value) in threat_collections {
                    value.threat_collections.sort_by(|a, b| b.name.cmp(&a.name));

                    println!("=> {}:", key);
                    let table = Table::new(value.threat_collections);
                    println!("{table}");
                }
            }
            GetterType::Triggers => {
                for (key, value) in triggers {
                    println!("{}:", key);
                    for d in value.triggers.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    // Not released yet, will be in future version.
                                    // .with(MinWidth::new(30))
                                    // .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::Vlans => {
                for (key, mut value) in vlans {
                    value.vlans.sort_by(|a, b| a.vlanid.cmp(&b.vlanid));

                    println!("{}:", key);
                    let table = Table::new(value.vlans);
                    println!("{table}");
                }
            }
            _ => {
                // Should never get here
            }
        }
    }

    Ok(())
}
