use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ProvisioningPolicy {
    #[default]
    #[serde(rename = "Fixed")]
    Fixed,
    #[serde(rename = "Thin")]
    Thin,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum StorageAccessCapability {
    #[default]
    #[serde(rename = "Append")]
    Append,
    #[serde(rename = "Execute")]
    Execute,
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "Streaming")]
    Streaming,
    #[serde(rename = "Write")]
    Write,
    #[serde(rename = "WriteOnce")]
    WriteOnce,
}
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_storage_los_capabilities::v1_2_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_storage_los_capabilities::v1_2_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaximumRecoverableCapacitySourceCount"
        )]
        pub maximum_recoverable_capacity_source_count: Option<i64>,
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
            rename = "SupportedAccessCapabilities"
        )]
        pub supported_access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
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
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedRecoveryTimeObjectives"
        )]
        pub supported_recovery_time_objectives:
            Option<Vec<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsSpaceEfficiency"
        )]
        pub supports_space_efficiency: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
