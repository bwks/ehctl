use crate::http::client::ExtraHopAppliance;
use std::fmt::{Display, Formatter};
pub struct Getters {}

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
    FirmwareNext,
    FirmwarePrevious,
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

impl Display for GetterType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GetterType::ActivityMaps => write!(f, "activitymaps"),
            GetterType::AuditLogs => write!(f, "auditlog"),
            GetterType::Alerts => write!(f, "alerts"),
            GetterType::Appliances => write!(f, "apikeys"),
            GetterType::ApiKeys => write!(f, "appliances"),
            GetterType::Bundles => write!(f, "bundles"),
            GetterType::Customizations => write!(f, "customizations"),
            GetterType::CustomDevices => write!(f, "customdevices"),
            GetterType::Dashboards => write!(f, "dashboards"),
            GetterType::Detections => write!(f, "detections"),
            GetterType::Devices => write!(f, "devicegroups"),
            GetterType::DeviceGroups => write!(f, "devices"),
            GetterType::EmailGroups => write!(f, "emailgroups"),
            GetterType::ExclusionIntervals => write!(f, "exclusionintervals"),
            GetterType::ExtraHop => write!(f, "extrahop"),
            GetterType::FirmwareNext => write!(f, "firmware-next"),
            GetterType::FirmwarePrevious => write!(f, "firmware-previous"),
            GetterType::IdentityProviders => write!(f, "identityproviders"),
            GetterType::License => write!(f, "license"),
            GetterType::Nodes => write!(f, "networks"),
            GetterType::Networks => write!(f, "networklocalities"),
            GetterType::NetworkLocalities => write!(f, "nodes"),
            GetterType::PacketCaptures => write!(f, "packetcaptures"),
            GetterType::RunningConfig => write!(f, "runningconfig"),
            GetterType::SamlSp => write!(f, "samlsp"),
            GetterType::Software => write!(f, "software"),
            GetterType::Tags => write!(f, "tags"),
            GetterType::ThreatCollections => write!(f, "threatcollections"),
            GetterType::Triggers => write!(f, "triggers"),
            GetterType::Vlans => write!(f, "vlans"),
            GetterType::Unknown => write!(f, "unknown"),
        }
    }
}

impl Getters {
    pub fn all() -> Vec<String> {
        vec![
            GetterType::ActivityMaps.to_string(),
            GetterType::AuditLogs.to_string(),
            GetterType::Alerts.to_string(),
            GetterType::Appliances.to_string(),
            GetterType::ApiKeys.to_string(),
            GetterType::Bundles.to_string(),
            GetterType::Customizations.to_string(),
            GetterType::CustomDevices.to_string(),
            GetterType::Dashboards.to_string(),
            GetterType::Detections.to_string(),
            GetterType::Devices.to_string(),
            GetterType::DeviceGroups.to_string(),
            GetterType::EmailGroups.to_string(),
            GetterType::ExclusionIntervals.to_string(),
            GetterType::ExtraHop.to_string(),
            GetterType::FirmwareNext.to_string(),
            GetterType::FirmwarePrevious.to_string(),
            GetterType::IdentityProviders.to_string(),
            GetterType::License.to_string(),
            GetterType::Nodes.to_string(),
            GetterType::Networks.to_string(),
            GetterType::NetworkLocalities.to_string(),
            GetterType::PacketCaptures.to_string(),
            GetterType::RunningConfig.to_string(),
            GetterType::SamlSp.to_string(),
            GetterType::Software.to_string(),
            GetterType::Tags.to_string(),
            GetterType::ThreatCollections.to_string(),
            GetterType::Triggers.to_string(),
            GetterType::Vlans.to_string(),
            // GetterType::Unknown.to_string(),
        ]
    }
    pub fn ccp() -> Vec<String> {
        appliance_getters(&ExtraHopAppliance::CCP)
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    }
    pub fn eca() -> Vec<String> {
        appliance_getters(&ExtraHopAppliance::ECA)
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    }
    pub fn eda() -> Vec<String> {
        appliance_getters(&ExtraHopAppliance::EDA)
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    }
    pub fn exa() -> Vec<String> {
        appliance_getters(&ExtraHopAppliance::EXA)
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    }
    pub fn eta() -> Vec<String> {
        appliance_getters(&ExtraHopAppliance::ETA)
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    }
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
                GetterType::FirmwareNext,
                GetterType::FirmwarePrevious,
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
                GetterType::FirmwareNext,
                GetterType::FirmwarePrevious,
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
