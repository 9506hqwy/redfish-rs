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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StoragePool {
    StoragePoolV1N0N2StoragePool(crate::swordfish::storage_pool::v1_0_2::StoragePool),
    StoragePoolV1N1N3StoragePool(crate::swordfish::storage_pool::v1_1_3::StoragePool),
    StoragePoolV1N2N1StoragePool(crate::swordfish::storage_pool::v1_2_1::StoragePool),
    StoragePoolV1N3N1StoragePool(crate::swordfish::storage_pool::v1_3_1::StoragePool),
    StoragePoolV1N4N1StoragePool(crate::swordfish::storage_pool::v1_4_1::StoragePool),
    StoragePoolV1N5N0StoragePool(crate::swordfish::storage_pool::v1_5_0::StoragePool),
    StoragePoolV1N6N1StoragePool(crate::swordfish::storage_pool::v1_6_1::StoragePool),
    StoragePoolV1N7N2StoragePool(crate::swordfish::storage_pool::v1_7_2::StoragePool),
    StoragePoolV1N8N0StoragePool(crate::swordfish::storage_pool::v1_8_0::StoragePool),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_0_2::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_0_2::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_0_2::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_0_2::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_0_2::SetEncryptionState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetCompressionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDeduplicationState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StoragePool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassesOfService")]
        pub classes_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_0_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_1_3::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_1_3::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_1_3::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_1_3::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_1_3::SetEncryptionState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetCompressionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDeduplicationState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StoragePool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassesOfService")]
        pub classes_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_1_3::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
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
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_2_1::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_2_1::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_2_1::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_2_1::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_2_1::SetEncryptionState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareResourceSets")]
        pub spare_resource_sets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareResourceSets@odata.count"
        )]
        pub spare_resource_sets_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetCompressionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDeduplicationState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StoragePool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassesOfService")]
        pub classes_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_2_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
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
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_3_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_3_1::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_3_1::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_3_1::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_3_1::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_3_1::SetEncryptionState>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareResourceSets")]
        pub spare_resource_sets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareResourceSets@odata.count"
        )]
        pub spare_resource_sets_odata_count: Option<i64>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_3_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
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
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_4_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_4_1::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_4_1::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_4_1::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_4_1::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_4_1::SetEncryptionState>,
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
        pub end_grp_lifetime: Option<crate::swordfish::storage_pool::v1_4_1::EndGrpLifetime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_4_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_4_1::NVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties: Option<crate::swordfish::storage_pool::v1_4_1::NVMeSetProperties>,
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
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_5_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_5_0::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_5_0::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_5_0::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_5_0::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_5_0::SetEncryptionState>,
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
        pub end_grp_lifetime: Option<crate::swordfish::storage_pool::v1_5_0::EndGrpLifetime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_5_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultClassOfService"
        )]
        pub default_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_5_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_5_0::NVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties: Option<crate::swordfish::storage_pool::v1_5_0::NVMeSetProperties>,
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
            rename = "RecoverableCapacitySourceCount"
        )]
        pub recoverable_capacity_source_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_6_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_6_1::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_6_1::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_6_1::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_6_1::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_6_1::SetEncryptionState>,
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
        pub end_grp_lifetime: Option<crate::swordfish::storage_pool::v1_6_1::EndGrpLifetime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMePoolType")]
        pub nvme_pool_type: Option<crate::swordfish::storage_pool::NVMePoolType>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_6_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionEnabled")]
        pub encryption_enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_6_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_6_1::NVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeProperties")]
        pub nvme_properties: Option<crate::swordfish::storage_pool::v1_6_1::NVMeProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties: Option<crate::swordfish::storage_pool::v1_6_1::NVMeSetProperties>,
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
        pub pool_type: Option<Vec<crate::swordfish::storage_pool::PoolType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
pub mod v1_7_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_7_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_7_2::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_7_2::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_7_2::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_7_2::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_7_2::SetEncryptionState>,
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
        pub end_grp_lifetime: Option<crate::swordfish::storage_pool::v1_7_2::EndGrpLifetime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMePoolType")]
        pub nvme_pool_type: Option<crate::swordfish::storage_pool::NVMePoolType>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_7_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionEnabled")]
        pub encryption_enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_7_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_7_2::NVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeProperties")]
        pub nvme_properties: Option<crate::swordfish::storage_pool::v1_7_2::NVMeProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties: Option<crate::swordfish::storage_pool::v1_7_2::NVMeSetProperties>,
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
        pub pool_type: Option<Vec<crate::swordfish::storage_pool::PoolType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedPoolTypes")]
        pub supported_pool_types: Option<Vec<crate::swordfish::storage_pool::PoolType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
pub mod v1_8_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_pool::v1_8_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.AddDrives"
        )]
        pub storage_pool_add_drives: Option<crate::swordfish::storage_pool::v1_8_0::AddDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.RemoveDrives"
        )]
        pub storage_pool_remove_drives:
            Option<crate::swordfish::storage_pool::v1_8_0::RemoveDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetCompressionState"
        )]
        pub storage_pool_set_compression_state:
            Option<crate::swordfish::storage_pool::v1_8_0::SetCompressionState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetDeduplicationState"
        )]
        pub storage_pool_set_deduplication_state:
            Option<crate::swordfish::storage_pool::v1_8_0::SetDeduplicationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StoragePool.SetEncryptionState"
        )]
        pub storage_pool_set_encryption_state:
            Option<crate::swordfish::storage_pool::v1_8_0::SetEncryptionState>,
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
        pub end_grp_lifetime: Option<crate::swordfish::storage_pool::v1_8_0::EndGrpLifetime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMePoolType")]
        pub nvme_pool_type: Option<crate::swordfish::storage_pool::NVMePoolType>,
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
        pub actions: Option<crate::swordfish::storage_pool::v1_8_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedVolumes")]
        pub allocated_volumes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionEnabled")]
        pub encryption_enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_pool::v1_8_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeEnduranceGroupProperties"
        )]
        pub nvme_endurance_group_properties:
            Option<crate::swordfish::storage_pool::v1_8_0::NVMeEnduranceGroupProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeProperties")]
        pub nvme_properties: Option<crate::swordfish::storage_pool::v1_8_0::NVMeProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSetProperties")]
        pub nvme_set_properties: Option<crate::swordfish::storage_pool::v1_8_0::NVMeSetProperties>,
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
        pub pool_type: Option<Vec<crate::swordfish::storage_pool::PoolType>>,
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
        pub supported_pool_types: Option<Vec<crate::swordfish::storage_pool::PoolType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedProvisioningPolicies"
        )]
        pub supported_provisioning_policies:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types: Option<Vec<crate::swordfish::volume::RAIDType>>,
    }
}
