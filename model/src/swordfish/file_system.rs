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
pub type FileSystem = crate::swordfish::file_system::v1_4_2::FileSystem;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportedShare {}
pub mod v1_4_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_system::v1_4_2::OemActions>,
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
            Option<Vec<crate::swordfish::file_system::v1_4_2::FileSystemAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_system::v1_4_2::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CasePreserved")]
        pub case_preserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CaseSensitive")]
        pub case_sensitive: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CharacterCodeSet")]
        pub character_code_set:
            Option<Vec<crate::swordfish::file_system::v1_4_2::FileSystemCharacterCodeSet>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClusterSizeBytes")]
        pub cluster_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::file_system::v1_4_2::FileSystemDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExportedShares")]
        pub exported_shares: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImportedShares")]
        pub imported_shares: Option<Vec<crate::swordfish::file_system::ImportedShare>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_system::v1_4_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::swordfish::file_system::v1_4_2::FileSystemMetrics>,
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
        pub remaining_capacity: Option<crate::swordfish::capacity::Capacity>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FileSystemAccessCapabilities {
        V000001(crate::swordfish::file_system::v1_4_2::FileSystemAccessCapabilitiesN1),
        DataStorageLoSCapabilitiesStorageAccessCapability(
            crate::swordfish::data_storage_los_capabilities::StorageAccessCapability,
        ),
    }
    impl Default for FileSystemAccessCapabilities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FileSystemAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FileSystemCharacterCodeSet {
        V010402(crate::swordfish::file_system::v1_4_2::CharacterCodeSet),
        V000001(crate::swordfish::file_system::v1_4_2::FileSystemCharacterCodeSetN1),
    }
    impl Default for FileSystemCharacterCodeSet {
        fn default() -> Self {
            Self::V010402(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FileSystemCharacterCodeSetN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FileSystemDescription {
        V000001(crate::swordfish::file_system::v1_4_2::FileSystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for FileSystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FileSystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FileSystemMetrics {
        V000001(crate::swordfish::file_system::v1_4_2::FileSystemMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for FileSystemMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FileSystemMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
