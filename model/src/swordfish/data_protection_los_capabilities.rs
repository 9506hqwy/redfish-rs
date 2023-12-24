use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DataProtectionLoSCapabilities {
    DataProtectionLoSCapabilitiesV1N0N3DataProtectionLoSCapabilities(
        crate::swordfish::data_protection_los_capabilities::v1_0_3::DataProtectionLoSCapabilities,
    ),
    DataProtectionLoSCapabilitiesV1N1N3DataProtectionLoSCapabilities(
        crate::swordfish::data_protection_los_capabilities::v1_1_3::DataProtectionLoSCapabilities,
    ),
    DataProtectionLoSCapabilitiesV1N2N0DataProtectionLoSCapabilities(
        crate::swordfish::data_protection_los_capabilities::v1_2_0::DataProtectionLoSCapabilities,
    ),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum FailureDomainScope {
    #[default]
    #[serde(rename = "Datacenter")]
    Datacenter,
    #[serde(rename = "Rack")]
    Rack,
    #[serde(rename = "RackGroup")]
    RackGroup,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Row")]
    Row,
    #[serde(rename = "Server")]
    Server,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum RecoveryAccessScope {
    #[default]
    #[serde(rename = "Nearline")]
    Nearline,
    #[serde(rename = "Offline")]
    Offline,
    #[serde(rename = "OnlineActive")]
    OnlineActive,
    #[serde(rename = "OnlinePassive")]
    OnlinePassive,
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataProtectionLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::data_protection_los_capabilities::v1_0_3::Links>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.etag")]
        pub odata_etag: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(rename = "@odata.type")]
        pub odata_type: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMinLifetimes"
        )]
        pub supported_min_lifetimes: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryGeographicObjectives"
        )]
        pub supported_recovery_geographic_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::FailureDomainScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryPointObjectiveTimes"
        )]
        pub supported_recovery_point_objective_times: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryTimeObjectives"
        )]
        pub supported_recovery_time_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaTypes"
        )]
        pub supported_replica_types:
            Option<Vec<crate::swordfish::storage_replica_info::ReplicaType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsIsolated")]
        pub supports_isolated: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions"
        )]
        pub supported_replica_options: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions@odata.count"
        )]
        pub supported_replica_options_odata_count: Option<i64>,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_protection_los_capabilities::v1_1_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataProtectionLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_protection_los_capabilities::v1_1_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::data_protection_los_capabilities::v1_1_3::Links>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.etag")]
        pub odata_etag: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(rename = "@odata.type")]
        pub odata_type: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMinLifetimes"
        )]
        pub supported_min_lifetimes: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryGeographicObjectives"
        )]
        pub supported_recovery_geographic_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::FailureDomainScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryPointObjectiveTimes"
        )]
        pub supported_recovery_point_objective_times: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryTimeObjectives"
        )]
        pub supported_recovery_time_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaTypes"
        )]
        pub supported_replica_types:
            Option<Vec<crate::swordfish::storage_replica_info::ReplicaType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsIsolated")]
        pub supports_isolated: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions"
        )]
        pub supported_replica_options: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions@odata.count"
        )]
        pub supported_replica_options_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_protection_los_capabilities::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataProtectionLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_protection_los_capabilities::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::data_protection_los_capabilities::v1_2_0::Links>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.etag")]
        pub odata_etag: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(rename = "@odata.type")]
        pub odata_type: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMinLifetimes"
        )]
        pub supported_min_lifetimes: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryGeographicObjectives"
        )]
        pub supported_recovery_geographic_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::FailureDomainScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryPointObjectiveTimes"
        )]
        pub supported_recovery_point_objective_times: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryTimeObjectives"
        )]
        pub supported_recovery_time_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaTypes"
        )]
        pub supported_replica_types:
            Option<Vec<crate::swordfish::storage_replica_info::ReplicaType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsIsolated")]
        pub supports_isolated: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions"
        )]
        pub supported_replica_options: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedReplicaOptions@odata.count"
        )]
        pub supported_replica_options_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
