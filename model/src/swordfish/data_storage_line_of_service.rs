use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DataStorageLineOfService {
    V010301(crate::swordfish::data_storage_line_of_service::v1_3_1::DataStorageLineOfService),
    V010201(crate::swordfish::data_storage_line_of_service::v1_2_1::DataStorageLineOfService),
    V010102(crate::swordfish::data_storage_line_of_service::v1_1_2::DataStorageLineOfService),
    V010002(crate::swordfish::data_storage_line_of_service::v1_0_2::DataStorageLineOfService),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
impl Default for DataStorageLineOfService {
    fn default() -> Self {
        Self::V010301(Default::default())
    }
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpaceEfficient")]
        pub is_space_efficient: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoveryTimeObjectives"
        )]
        pub recovery_time_objectives:
            Option<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>,
    }
}
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpaceEfficient")]
        pub is_space_efficient: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoveryTimeObjectives"
        )]
        pub recovery_time_objectives:
            Option<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>,
    }
}
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpaceEfficient")]
        pub is_space_efficient: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoveryTimeObjectives"
        )]
        pub recovery_time_objectives:
            Option<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>,
    }
}
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_storage_line_of_service::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_storage_line_of_service::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpaceEfficient")]
        pub is_space_efficient: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoveryTimeObjectives"
        )]
        pub recovery_time_objectives:
            Option<crate::swordfish::data_protection_los_capabilities::RecoveryAccessScope>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
