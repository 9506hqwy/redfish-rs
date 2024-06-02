pub type DataStorageLineOfService =
    crate::swordfish::data_storage_line_of_service::v1_3_1::DataStorageLineOfService;
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_storage_line_of_service::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataStorageLineOfService { # [serde (skip_serializing_if = "Option::is_none" , rename = "AccessCapabilities")] pub access_capabilities : Option < Vec < crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceAccessCapabilities > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceDescription > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "IsSpaceEfficient")] pub is_space_efficient : Option < bool > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ProvisioningPolicy")] pub provisioning_policy : Option < crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceProvisioningPolicy > , # [serde (skip_serializing_if = "Option::is_none" , rename = "RecoverableCapacitySourceCount")] pub recoverable_capacity_source_count : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none" , rename = "RecoveryTimeObjectives")] pub recovery_time_objectives : Option < crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceRecoveryTimeObjectives > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataStorageLineOfServiceAccessCapabilities {
        V000001 (crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceAccessCapabilitiesN1) , DataStorageLoSCapabilitiesStorageAccessCapability (crate :: swordfish :: data_storage_los_capabilities :: StorageAccessCapability) }
    impl Default for DataStorageLineOfServiceAccessCapabilities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataStorageLineOfServiceAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataStorageLineOfServiceDescription {
        V000001 (crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceDescriptionN1) , ResourceDescription (String) }
    impl Default for DataStorageLineOfServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataStorageLineOfServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataStorageLineOfServiceProvisioningPolicy {
        V000001 (crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceProvisioningPolicyN1) , DataStorageLoSCapabilitiesProvisioningPolicy (crate :: swordfish :: data_storage_los_capabilities :: ProvisioningPolicy) }
    impl Default for DataStorageLineOfServiceProvisioningPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataStorageLineOfServiceProvisioningPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataStorageLineOfServiceRecoveryTimeObjectives {
        V000001 (crate :: swordfish :: data_storage_line_of_service :: v1_3_1 :: DataStorageLineOfServiceRecoveryTimeObjectivesN1) , DataProtectionLoSCapabilitiesRecoveryAccessScope (crate :: swordfish :: data_protection_los_capabilities :: RecoveryAccessScope) }
    impl Default for DataStorageLineOfServiceRecoveryTimeObjectives {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataStorageLineOfServiceRecoveryTimeObjectivesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
