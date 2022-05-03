use crate::client::ExtraHopAppliance;

#[derive(Eq, PartialEq)]
pub enum GetterType {
    ActivityMaps,
    AuditLogs,
    Alerts,
    Appliances,
    ApiKeys,
    Bundles,
    Customizations,
    CustomDevices,
    Dashboards,
    Detections,
    Devices,
    DeviceGroups,
    EmailGroups,
    ExclusionIntervals,
    ExtraHop,
    IdentityProviders,
    License,
    Nodes,
    Networks,
    NetworkLocalities,
    PacketCaptures,
    RunningConfig,
    SamlSp,
    Software,
    Tags,
    ThreatCollections,
    Triggers,
    Vlans,
    Unknown,
}

pub fn getter_list() -> Vec<String> {
    vec![
        "activitymaps".to_string(),
        "auditlog".to_string(),
        "alerts".to_string(),
        "apikeys".to_string(),
        "appliances".to_string(),
        "bundles".to_string(),
        "customizations".to_string(),
        "customdevices".to_string(),
        "dashboards".to_string(),
        "detections".to_string(),
        "devicegroups".to_string(),
        "devices".to_string(),
        "emailgroups".to_string(),
        "exclusionintervals".to_string(),
        "extrahop".to_string(),
        "identityproviders".to_string(),
        "license".to_string(),
        "networks".to_string(),
        "networklocalities".to_string(),
        "nodes".to_string(),
        "packetcaptures".to_string(),
        "runningconfig".to_string(),
        "samlsp".to_string(),
        "software".to_string(),
        "tags".to_string(),
        "threatcollections".to_string(),
        "triggers".to_string(),
        "vlans".to_string(),
    ]
}

pub fn appliance_getters(appliance_type: &ExtraHopAppliance) -> Vec<GetterType> {
    match appliance_type {
        ExtraHopAppliance::CCP => {
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
                GetterType::ExtraHop,
                GetterType::Networks,
                GetterType::NetworkLocalities,
                GetterType::Tags,
                GetterType::Triggers,
                GetterType::Software,
                GetterType::ThreatCollections,
                GetterType::Vlans,
            ]
        }
        ExtraHopAppliance::ECA => {
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
                GetterType::ExtraHop,
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
            ]
        }
        ExtraHopAppliance::EDA => {
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
                GetterType::ExtraHop,
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
            ]
        }
        ExtraHopAppliance::EXA => {
            vec![
                GetterType::ApiKeys,
                GetterType::Appliances,
                GetterType::ExtraHop,
                GetterType::License,
                GetterType::RunningConfig,
            ]
        }
        ExtraHopAppliance::ETA => {
            vec![
                GetterType::ApiKeys,
                GetterType::Appliances,
                GetterType::ExtraHop,
                GetterType::License,
                GetterType::RunningConfig,
            ]
        }
    }
}
