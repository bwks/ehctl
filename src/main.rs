mod cmd;
mod core;
mod http;
mod model;
mod run;
mod util;

use crate::cmd::cli::{CliOptions, OutputOption};
use crate::cmd::command::CliCommand;
use crate::core::config::ExtraHopConfig;
use crate::http::action::*;
use crate::http::client::{build_clients, ExtraHopAppliance, ExtraHopClient};
use crate::http::firmware::{get_firmware_next, get_firmware_previous, upload_firmware};
use crate::http::getter::{appliance_getters, GetterType};

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
use model::firmware::{FirmwarePrevious, FirmwaresNext};
use model::license::License;
use model::network::Networks;
use model::network_locality::NetworkLocalities;
use model::node::Nodes;
use model::packet_capture::PacketCaptures;
use model::software::Softwares;
use model::tag::Tags;
use model::threat_collection::ThreatCollections;
use model::trigger::Triggers;
use model::vlan::Vlans;

use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;
use std::process::exit;
use tabled::{Alignment, Disable, Full, MaxWidth, MinWidth, Modify, Rotate, Rows, Table};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliOptions::new();

    let time_now = Local::now();
    let timestamp = time_now.format("%Y-%m-%d--%H-%M-%S").to_string();

    let configs = ExtraHopConfig::new();

    let mut extrahop_appliances: Vec<ExtraHopClient> = Vec::new();

    let ccp_clients = build_clients(&configs.ccp, ExtraHopAppliance::CCP, &timestamp).await?;
    let eca_clients = build_clients(&configs.eca, ExtraHopAppliance::ECA, &timestamp).await?;
    let eda_clients = build_clients(&configs.eda, ExtraHopAppliance::EDA, &timestamp).await?;
    let exa_clients = build_clients(&configs.exa, ExtraHopAppliance::EXA, &timestamp).await?;
    let eta_clients = build_clients(&configs.eta, ExtraHopAppliance::ETA, &timestamp).await?;

    extrahop_appliances.extend(ccp_clients);
    extrahop_appliances.extend(eca_clients);
    extrahop_appliances.extend(eda_clients);
    extrahop_appliances.extend(exa_clients);
    extrahop_appliances.extend(eta_clients);

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
    let mut firmwares_next: HashMap<String, FirmwaresNext> = HashMap::new();
    let mut firmwares_previous: HashMap<String, FirmwarePrevious> = HashMap::new();
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

    for c in extrahop_appliances.iter() {
        match cli.command {
            CliCommand::Backup => match c.appliance_type {
                ExtraHopAppliance::ECA | ExtraHopAppliance::EDA => create_customization(c).await?,
                ExtraHopAppliance::EXA | ExtraHopAppliance::ETA => get_running_config(c).await?,
                _ => {}
            },
            CliCommand::Firmware => {
                if c.hostname == cli.firmware_options.hostname {
                    println!("c.hostname {}", c.hostname);
                    println!(
                        "cli.firmware_options.hostname {}",
                        cli.firmware_options.hostname
                    );
                    upload_firmware(c, cli.firmware_options.filename.as_str()).await?
                }
            }
            CliCommand::Get => {
                match cli.getter_type {
                    GetterType::ActivityMaps => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_activity_maps(c).await?;
                            activity_maps.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::AuditLogs => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_audit_logs(c).await?;
                            audit_logs.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Alerts => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_alerts(c).await?;
                            alerts.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::ApiKeys => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_api_keys(c).await?;
                            api_keys.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Appliances => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_appliances(c).await?;
                            appliances.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Bundles => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_bundles(c).await?;
                            bundles.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Customizations => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_customizations(c).await?;
                            customizations.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::CustomDevices => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_custom_devices(c).await?;
                            custom_devices.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Dashboards => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_dashboards(c).await?;
                            dashboards.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Detections => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_detections(c).await?;
                            detections.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Devices => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_devices(c).await?;
                            devices.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::DeviceGroups => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_device_groups(c).await?;
                            device_groups.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::EmailGroups => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_email_groups(c).await?;
                            email_groups.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::ExclusionIntervals => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_exclusion_intervals(c).await?;
                            exclusion_intervals.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::ExtraHop => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_extrahop(c).await?;
                            extrahops.push(result);
                        }
                    }
                    GetterType::FirmwareNext => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_firmware_next(c).await?;
                            firmwares_next.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::FirmwarePrevious => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_firmware_previous(c).await?;
                            firmwares_previous.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::IdentityProviders => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_identitiy_providers(c).await?;
                            identity_providers.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::License => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_license(c).await?;
                            licenses.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Networks => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_networks(c).await?;
                            networks.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::NetworkLocalities => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_network_localities(c).await?;
                            network_localities.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Nodes => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_nodes(c).await?;
                            nodes.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::PacketCaptures => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_packet_captures(c).await?;
                            packet_captures.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::RunningConfig => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            _ = get_running_config(c).await?;
                        }
                    }
                    GetterType::SamlSp => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_saml_sp(c).await?;
                            saml_sps.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Software => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_software(c).await?;
                            software.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Tags => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_tags(c).await?;
                            tags.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::ThreatCollections => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_threat_collections(c).await?;
                            threat_collections.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Triggers => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_triggers(c).await?;
                            triggers.insert(c.hostname.to_string(), result);
                        }
                    }
                    GetterType::Vlans => {
                        if appliance_getters(&c.appliance_type).contains(&cli.getter_type) {
                            let result = get_vlans(c).await?;
                            vlans.insert(c.hostname.to_string(), result);
                        }
                    }
                    // Should never get here
                    _ => {
                        eprintln!("should not be here, but yet I am.");
                        exit(1);
                    }
                }
            }
            CliCommand::PacketSearch => match c.appliance_type {
                ExtraHopAppliance::CCP | ExtraHopAppliance::ECA => {
                    packet_search(c, &cli.packet_search_options).await?
                }
                // No other appliance types can be searched for packets
                _ => {}
            },
            // Should never get here
            _ => {
                eprintln!("I should not be here, but I am help.");
                exit(1);
            }
        }
    }

    if cli.command == CliCommand::Get {
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
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::FirmwareNext => {
                for (key, value) in firmwares_next {
                    println!("{}:", key);
                    for d in value.firmware.iter() {
                        let table = Table::new(vec![d])
                            .with(
                                Modify::new(Full)
                                    .with(MinWidth::new(30))
                                    .with(MaxWidth::wrapping(30))
                                    .with(Alignment::left()),
                            )
                            .with(Rotate::Left);
                        println!("{}", table);
                    }
                }
            }
            GetterType::FirmwarePrevious => {
                for (key, value) in firmwares_previous {
                    println!("{}:", key);
                    let table = Table::new(vec![value])
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
            GetterType::ExtraHop => {
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
                eprintln!("should not be here, but here I am.");
                exit(1);
            }
        }
    }

    Ok(())
}
