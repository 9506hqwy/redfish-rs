use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum FileProtocol {
    #[default]
    #[serde(rename = "NFSv3")]
    NFSv3,
    #[serde(rename = "NFSv4_0")]
    NFSv4N0,
    #[serde(rename = "NFSv4_1")]
    NFSv4N1,
    #[serde(rename = "SMBv2_0")]
    SMBv2N0,
    #[serde(rename = "SMBv2_1")]
    SMBv2N1,
    #[serde(rename = "SMBv3_0")]
    SMBv3N0,
    #[serde(rename = "SMBv3_0_2")]
    SMBv3N0N2,
    #[serde(rename = "SMBv3_1_1")]
    SMBv3N1N1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportedShare {}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_system::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CharacterCodeSet {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "ExtendedUNIXCode")]
        ExtendedUNIXCode,
        #[serde(rename = "ISO2022")]
        ISO2022,
        #[serde(rename = "ISO8859_1")]
        ISO8859N1,
        #[serde(rename = "UCS_2")]
        UCSN2,
        #[serde(rename = "UTF_16")]
        UTFN16,
        #[serde(rename = "UTF_8")]
        UTFN8,
        #[serde(rename = "Unicode")]
        Unicode,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_system::v1_3_0::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CasePreserved")]
        pub case_preserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CaseSensitive")]
        pub case_sensitive: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CharacterCodeSet")]
        pub character_code_set:
            Option<Vec<crate::swordfish::file_system::v1_3_0::CharacterCodeSet>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClusterSizeBytes")]
        pub cluster_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExportedShares")]
        pub exported_shares: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImportedShares")]
        pub imported_shares: Option<Vec<crate::swordfish::file_system::ImportedShare>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_system::v1_3_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxFileNameLengthBytes"
        )]
        pub max_file_name_length_bytes: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemainingCapacity")]
        pub remaining_capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicationEnabled")]
        pub replication_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaCollection")]
        pub replica_collection: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaCollection@odata.count"
        )]
        pub replica_collection_odata_count: Option<i64>,
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
}
