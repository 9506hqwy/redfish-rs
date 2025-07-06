use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum NVMePoolType {
    #[default]
    #[serde(rename = "EnduranceGroup")]
    EnduranceGroup,
    #[serde(rename = "NVMSet")]
    NVMSet,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PoolType {
    #[default]
    #[serde(rename = "Block")]
    Block,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Object")]
    Object,
    #[serde(rename = "Pool")]
    Pool,
}
pub type StoragePool = crate::swordfish::storage_pool::v1_9_2::StoragePool;
pub mod v1_9_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_9_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_9_2::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_9_2::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_9_2::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_9_2::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_9_2::SetEncryptionState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddDrivesRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySource")]
        pub capacity_source: Option<crate::swordfish::capacity::CapacitySource>,
        #[serde(rename = "Drives")]
        pub drives: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EndGrpLifetime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataUnitsRead")]
        pub data_units_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataUnitsWritten")]
        pub data_units_written: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnduranceEstimate")]
        pub endurance_estimate: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ErrorInformationLogEntryCount"
        )]
        pub error_information_log_entry_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReadCommandCount"
        )]
        pub host_read_command_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostWriteCommandCount"
        )]
        pub host_write_command_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MediaAndDataIntegrityErrorCount"
        )]
        pub media_and_data_integrity_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaUnitsWritten")]
        pub media_units_written: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentUsed")]
        pub percent_used: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DedicatedSpareDrives"
        )]
        pub dedicated_spare_drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DedicatedSpareDrives@odata.count"
        )]
        pub dedicated_spare_drives_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OwningStorageResource"
        )]
        pub owning_storage_resource: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareResourceSets")]
        pub spare_resource_sets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareResourceSets@odata.count"
        )]
        pub spare_resource_sets_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeEnduranceGroupProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndGrpLifetime")]
        pub end_grp_lifetime: Option<
            crate::swordfish::storage_pool::v1_9_2::NVMeEnduranceGroupPropertiesEndGrpLifetime,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeEnduranceGroupPropertiesEndGrpLifetime {
        V010902(crate::swordfish::storage_pool::v1_9_2::EndGrpLifetime),
        V000001(
            crate::swordfish::storage_pool::v1_9_2::NVMeEnduranceGroupPropertiesEndGrpLifetimeN1,
        ),
    }
    impl Default for NVMeEnduranceGroupPropertiesEndGrpLifetime {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeEnduranceGroupPropertiesEndGrpLifetimeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMePoolType")]
        pub nvme_pool_type:
            Option<crate::swordfish::storage_pool::v1_9_2::NVMePropertiesNVMePoolType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMePropertiesNVMePoolType {
        V000001(crate::swordfish::storage_pool::v1_9_2::NVMePropertiesNVMePoolTypeN1),
        StoragePoolNVMePoolType(crate::swordfish::storage_pool::NVMePoolType),
    }
    impl Default for NVMePropertiesNVMePoolType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMePropertiesNVMePoolTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSetProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnduranceGroupIdentifier"
        )]
        pub endurance_group_identifier: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OptimalWriteSizeBytes"
        )]
        pub optimal_write_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Random4kReadTypicalNanoSeconds"
        )]
        pub random4k_read_typical_nano_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetIdentifier")]
        pub set_identifier: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnallocatedNVMNamespaceCapacityBytes"
        )]
        pub unallocated_nvm_namespace_capacity_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveDrivesRequestBody {
        #[serde(rename = "Drives")]
        pub drives: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetCompressionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetCompressionStateRequestBody {
        #[serde(rename = "Enable")]
        pub enable: bool,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDeduplicationState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDeduplicationStateRequestBody {
        #[serde(rename = "Enable")]
        pub enable: bool,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionStateRequestBody {
        #[serde(rename = "Enable")]
        pub enable: bool,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StoragePool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_pool::v1_9_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassesOfService")]
        pub classes_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Compressed")]
        pub compressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompressionEnabled")]
        pub compression_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeduplicationEnabled"
        )]
        pub deduplication_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultCompressionBehavior"
        )]
        pub default_compression_behavior: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultDeduplicationBehavior"
        )]
        pub default_deduplication_behavior: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultEncryptionBehavior"
        )]
        pub default_encryption_behavior: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::storage_pool::v1_9_2::StoragePoolDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionEnabled")]
        pub encryption_enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_9_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::swordfish::storage_pool::v1_9_2::StoragePoolMetrics>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeProperties")]
        pub nvme_properties:
            Option<crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMeProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties:
            Option<crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMeSetProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoolType")]
        pub pool_type: Option<Vec<crate::swordfish::storage_pool::v1_9_2::StoragePoolPoolType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicationEnabled")]
        pub replication_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedPoolTypes")]
        pub supported_pool_types:
            Option<Vec<crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedPoolTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies: Option<
            Vec<crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedProvisioningPolicies>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types:
            Option<Vec<crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedRAIDTypes>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolDescription {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StoragePoolDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolMetrics {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for StoragePoolMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolNVMeEnduranceGroupProperties {
        V010902(crate::swordfish::storage_pool::v1_9_2::NVMeEnduranceGroupProperties),
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMeEnduranceGroupPropertiesN1),
    }
    impl Default for StoragePoolNVMeEnduranceGroupProperties {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolNVMeEnduranceGroupPropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolNVMeProperties {
        V010902(crate::swordfish::storage_pool::v1_9_2::NVMeProperties),
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMePropertiesN1),
    }
    impl Default for StoragePoolNVMeProperties {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolNVMePropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolNVMeSetProperties {
        V010902(crate::swordfish::storage_pool::v1_9_2::NVMeSetProperties),
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolNVMeSetPropertiesN1),
    }
    impl Default for StoragePoolNVMeSetProperties {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolNVMeSetPropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolPoolType {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolPoolTypeN1),
        StoragePoolPoolType(crate::swordfish::storage_pool::PoolType),
    }
    impl Default for StoragePoolPoolType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolPoolTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolSupportedPoolTypes {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedPoolTypesN1),
        StoragePoolPoolType(crate::swordfish::storage_pool::PoolType),
    }
    impl Default for StoragePoolSupportedPoolTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolSupportedPoolTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolSupportedProvisioningPolicies {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedProvisioningPoliciesN1),
        DataStorageLoSCapabilitiesProvisioningPolicy(
            crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy,
        ),
    }
    impl Default for StoragePoolSupportedProvisioningPolicies {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolSupportedProvisioningPoliciesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StoragePoolSupportedRAIDTypes {
        V000001(crate::swordfish::storage_pool::v1_9_2::StoragePoolSupportedRAIDTypesN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for StoragePoolSupportedRAIDTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StoragePoolSupportedRAIDTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
