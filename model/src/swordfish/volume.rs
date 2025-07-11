use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EncryptionTypes {
    #[default]
    #[serde(rename = "ControllerAssisted")]
    ControllerAssisted,
    #[serde(rename = "NativeDriveEncryption")]
    NativeDriveEncryption,
    #[serde(rename = "SoftwareAssisted")]
    SoftwareAssisted,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum InitializeMethod {
    #[default]
    #[serde(rename = "Background")]
    Background,
    #[serde(rename = "Foreground")]
    Foreground,
    #[serde(rename = "Skip")]
    Skip,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum InitializeType {
    #[default]
    #[serde(rename = "Fast")]
    Fast,
    #[serde(rename = "Slow")]
    Slow,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum LBAFormatType {
    #[default]
    #[serde(rename = "LBAFormat0")]
    LBAFormat0,
    #[serde(rename = "LBAFormat1")]
    LBAFormat1,
    #[serde(rename = "LBAFormat10")]
    LBAFormat10,
    #[serde(rename = "LBAFormat11")]
    LBAFormat11,
    #[serde(rename = "LBAFormat12")]
    LBAFormat12,
    #[serde(rename = "LBAFormat13")]
    LBAFormat13,
    #[serde(rename = "LBAFormat14")]
    LBAFormat14,
    #[serde(rename = "LBAFormat15")]
    LBAFormat15,
    #[serde(rename = "LBAFormat2")]
    LBAFormat2,
    #[serde(rename = "LBAFormat3")]
    LBAFormat3,
    #[serde(rename = "LBAFormat4")]
    LBAFormat4,
    #[serde(rename = "LBAFormat5")]
    LBAFormat5,
    #[serde(rename = "LBAFormat6")]
    LBAFormat6,
    #[serde(rename = "LBAFormat7")]
    LBAFormat7,
    #[serde(rename = "LBAFormat8")]
    LBAFormat8,
    #[serde(rename = "LBAFormat9")]
    LBAFormat9,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum LBARelativePerformanceType {
    #[default]
    #[serde(rename = "Best")]
    Best,
    #[serde(rename = "Better")]
    Better,
    #[serde(rename = "Degraded")]
    Degraded,
    #[serde(rename = "Good")]
    Good,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum NamespaceType {
    #[default]
    #[serde(rename = "Block")]
    Block,
    #[serde(rename = "Computational")]
    Computational,
    #[serde(rename = "KeyValue")]
    KeyValue,
    #[serde(rename = "ZNS")]
    ZNS,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum OperationType {
    #[default]
    #[serde(rename = "ChangeRAIDType")]
    ChangeRAIDType,
    #[serde(rename = "ChangeStripSize")]
    ChangeStripSize,
    #[serde(rename = "CheckConsistency")]
    CheckConsistency,
    #[serde(rename = "Compress")]
    Compress,
    #[serde(rename = "Decrypt")]
    Decrypt,
    #[serde(rename = "Deduplicate")]
    Deduplicate,
    #[serde(rename = "Delete")]
    Delete,
    #[serde(rename = "Encrypt")]
    Encrypt,
    #[serde(rename = "Format")]
    Format,
    #[serde(rename = "Initialize")]
    Initialize,
    #[serde(rename = "Rebuild")]
    Rebuild,
    #[serde(rename = "Replicate")]
    Replicate,
    #[serde(rename = "Resize")]
    Resize,
    #[serde(rename = "Sanitize")]
    Sanitize,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum RAIDType {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "RAID0")]
    RAID0,
    #[serde(rename = "RAID00")]
    RAID00,
    #[serde(rename = "RAID01")]
    RAID01,
    #[serde(rename = "RAID1")]
    RAID1,
    #[serde(rename = "RAID10")]
    RAID10,
    #[serde(rename = "RAID10E")]
    RAID10E,
    #[serde(rename = "RAID10Triple")]
    RAID10Triple,
    #[serde(rename = "RAID1E")]
    RAID1E,
    #[serde(rename = "RAID1Triple")]
    RAID1Triple,
    #[serde(rename = "RAID3")]
    RAID3,
    #[serde(rename = "RAID4")]
    RAID4,
    #[serde(rename = "RAID5")]
    RAID5,
    #[serde(rename = "RAID50")]
    RAID50,
    #[serde(rename = "RAID6")]
    RAID6,
    #[serde(rename = "RAID60")]
    RAID60,
    #[serde(rename = "RAID6TP")]
    RAID6TP,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReadCachePolicyType {
    #[default]
    #[serde(rename = "AdaptiveReadAhead")]
    AdaptiveReadAhead,
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "ReadAhead")]
    ReadAhead,
}
pub type Volume = crate::swordfish::volume::v1_10_2::Volume;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum VolumeType {
    #[default]
    #[serde(rename = "Mirrored")]
    Mirrored,
    #[serde(rename = "NonRedundant")]
    NonRedundant,
    #[serde(rename = "RawDevice")]
    RawDevice,
    #[serde(rename = "SpannedMirrors")]
    SpannedMirrors,
    #[serde(rename = "SpannedStripesWithParity")]
    SpannedStripesWithParity,
    #[serde(rename = "StripedWithParity")]
    StripedWithParity,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum VolumeUsageType {
    #[default]
    #[serde(rename = "CacheOnly")]
    CacheOnly,
    #[serde(rename = "Data")]
    Data,
    #[serde(rename = "ReplicationReserve")]
    ReplicationReserve,
    #[serde(rename = "SystemData")]
    SystemData,
    #[serde(rename = "SystemReserve")]
    SystemReserve,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum WriteCachePolicyType {
    #[default]
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "ProtectedWriteBack")]
    ProtectedWriteBack,
    #[serde(rename = "UnprotectedWriteBack")]
    UnprotectedWriteBack,
    #[serde(rename = "WriteThrough")]
    WriteThrough,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum WriteCacheStateType {
    #[default]
    #[serde(rename = "Degraded")]
    Degraded,
    #[serde(rename = "Protected")]
    Protected,
    #[serde(rename = "Unprotected")]
    Unprotected,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum WriteHoleProtectionPolicyType {
    #[default]
    #[serde(rename = "DistributedLog")]
    DistributedLog,
    #[serde(rename = "Journaling")]
    Journaling,
    #[serde(rename = "Oem")]
    Oem,
    #[serde(rename = "Off")]
    Off,
}
pub mod v1_10_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ALUA {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ANAGroupId")]
        pub ana_group_id: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::volume::v1_10_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.AssignReplicaTarget"
        )]
        pub volume_assign_replica_target:
            Option<crate::swordfish::volume::v1_10_1::AssignReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ChangeRAIDLayout"
        )]
        pub volume_change_raid_layout: Option<crate::swordfish::volume::v1_10_1::ChangeRAIDLayout>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.CheckConsistency"
        )]
        pub volume_check_consistency: Option<crate::swordfish::volume::v1_10_1::CheckConsistency>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.CreateReplicaTarget"
        )]
        pub volume_create_replica_target:
            Option<crate::swordfish::volume::v1_10_1::CreateReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ForceEnable"
        )]
        pub volume_force_enable: Option<crate::swordfish::volume::v1_10_1::ForceEnable>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Volume.Initialize")]
        pub volume_initialize: Option<crate::swordfish::volume::v1_10_1::Initialize>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.RemoveReplicaRelationship"
        )]
        pub volume_remove_replica_relationship:
            Option<crate::swordfish::volume::v1_10_1::RemoveReplicaRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ResumeReplication"
        )]
        pub volume_resume_replication: Option<crate::swordfish::volume::v1_10_1::ResumeReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ReverseReplicationRelationship"
        )]
        pub volume_reverse_replication_relationship:
            Option<crate::swordfish::volume::v1_10_1::ReverseReplicationRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.SplitReplication"
        )]
        pub volume_split_replication: Option<crate::swordfish::volume::v1_10_1::SplitReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.SuspendReplication"
        )]
        pub volume_suspend_replication:
            Option<crate::swordfish::volume::v1_10_1::SuspendReplication>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangeRAIDLayout {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangeRAIDLayoutRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaSpanCount")]
        pub media_span_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RAIDType")]
        pub raid_type: Option<crate::swordfish::volume::RAIDType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StripSizeBytes")]
        pub strip_size_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CheckConsistency {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CheckConsistencyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetStoragePool")]
        pub target_storage_pool: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeName")]
        pub volume_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceEnable {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceEnableRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Initialize {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InitializeRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeMethod")]
        pub initialize_method: Option<crate::swordfish::volume::InitializeMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeType")]
        pub initialize_type: Option<crate::swordfish::volume::InitializeType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LBAFormat {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBADataSizeBytes")]
        pub lba_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormatType")]
        pub lba_format_type: Option<crate::swordfish::volume::v1_10_1::LBAFormatLBAFormatType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LBAMetadataSizeBytes"
        )]
        pub lba_metadata_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelativePerformance"
        )]
        pub relative_performance:
            Option<crate::swordfish::volume::v1_10_1::LBAFormatRelativePerformance>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LBAFormatLBAFormatType {
        V000001(crate::swordfish::volume::v1_10_1::LBAFormatLBAFormatTypeN1),
        VolumeLBAFormatType(crate::swordfish::volume::LBAFormatType),
    }
    impl Default for LBAFormatLBAFormatType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LBAFormatLBAFormatTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LBAFormatRelativePerformance {
        V000001(crate::swordfish::volume::v1_10_1::LBAFormatRelativePerformanceN1),
        VolumeLBARelativePerformanceType(crate::swordfish::volume::LBARelativePerformanceType),
    }
    impl Default for LBAFormatRelativePerformance {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LBAFormatRelativePerformanceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheDataVolumes")]
        pub cache_data_volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CacheDataVolumes@odata.count"
        )]
        pub cache_data_volumes_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheVolumeSource")]
        pub cache_volume_source: Option<crate::swordfish::volume::v1_10_1::LinksCacheVolumeSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientEndpoints")]
        pub client_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpoints@odata.count"
        )]
        pub client_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyGroups")]
        pub consistency_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyGroups@odata.count"
        )]
        pub consistency_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Controllers@odata.count"
        )]
        pub controllers_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JournalingMedia")]
        pub journaling_media: Option<crate::swordfish::volume::v1_10_1::LinksJournalingMedia>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OwningStorageResource"
        )]
        pub owning_storage_resource: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OwningStorageService"
        )]
        pub owning_storage_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidingStoragePool"
        )]
        pub providing_storage_pool:
            Option<crate::swordfish::volume::v1_10_1::LinksProvidingStoragePool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerEndpoints")]
        pub server_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpoints@odata.count"
        )]
        pub server_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareResourceSets")]
        pub spare_resource_sets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareResourceSets@odata.count"
        )]
        pub spare_resource_sets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageGroups@odata.count"
        )]
        pub storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksCacheVolumeSource {
        V000001(crate::swordfish::volume::v1_10_1::LinksCacheVolumeSourceN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksCacheVolumeSource {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksCacheVolumeSourceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksJournalingMedia {
        V000001(crate::swordfish::volume::v1_10_1::LinksJournalingMediaN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksJournalingMedia {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksJournalingMediaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksProvidingStoragePool {
        V000001(crate::swordfish::volume::v1_10_1::LinksProvidingStoragePoolN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksProvidingStoragePool {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksProvidingStoragePoolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeNamespaceProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FormattedLBASize")]
        pub formatted_lba_size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsShareable")]
        pub is_shareable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormat")]
        pub lba_format: Option<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormat>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormats")]
        pub lba_formats:
            Option<Vec<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormats>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LBAFormatsSupported"
        )]
        pub lba_formats_supported: Option<
            Vec<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormatsSupported>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetadataTransferredAtEndOfDataLBA"
        )]
        pub metadata_transferred_at_end_of_data_lba: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceFeatures")]
        pub namespace_features:
            Option<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesNamespaceFeatures>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceId")]
        pub namespace_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceType")]
        pub namespace_type:
            Option<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesNamespaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NumberLBAFormats")]
        pub number_lba_formats: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeVersion")]
        pub nvme_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsIOPerformanceHints"
        )]
        pub supports_io_performance_hints: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsMultipleNamespaceAttachments"
        )]
        pub supports_multiple_namespace_attachments: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormat {
        V011001(crate::swordfish::volume::v1_10_1::LBAFormat),
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormatN1),
    }
    impl Default for NVMeNamespacePropertiesLBAFormat {
        fn default() -> Self {
            Self::V011001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormats {
        V011001(crate::swordfish::volume::v1_10_1::LBAFormat),
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormatsN1),
    }
    impl Default for NVMeNamespacePropertiesLBAFormats {
        fn default() -> Self {
            Self::V011001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormatsSupported {
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesLBAFormatsSupportedN1),
        VolumeLBAFormatType(crate::swordfish::volume::LBAFormatType),
    }
    impl Default for NVMeNamespacePropertiesLBAFormatsSupported {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatsSupportedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesNamespaceFeatures {
        V011001(crate::swordfish::volume::v1_10_1::NamespaceFeatures),
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesNamespaceFeaturesN1),
    }
    impl Default for NVMeNamespacePropertiesNamespaceFeatures {
        fn default() -> Self {
            Self::V011001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesNamespaceFeaturesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesNamespaceType {
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesNamespaceTypeN1),
        VolumeNamespaceType(crate::swordfish::volume::NamespaceType),
    }
    impl Default for NVMeNamespacePropertiesNamespaceType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesNamespaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesType {
        V000001(crate::swordfish::volume::v1_10_1::NVMeNamespacePropertiesTypeN1),
        VolumeNamespaceType(crate::swordfish::volume::NamespaceType),
    }
    impl Default for NVMeNamespacePropertiesType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NamespaceFeatures {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsAtomicTransactionSize"
        )]
        pub supports_atomic_transaction_size: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsDeallocatedOrUnwrittenLBError"
        )]
        pub supports_deallocated_or_unwritten_lb_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsIOPerformanceHints"
        )]
        pub supports_io_performance_hints: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsNGUIDReuse")]
        pub supports_nguid_reuse: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsThinProvisioning"
        )]
        pub supports_thin_provisioning: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Operation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedFeaturesRegistry"
        )]
        pub associated_features_registry: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operation")]
        pub operation: Option<crate::swordfish::volume::v1_10_1::OperationOperation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperationOperation {
        V000001(crate::swordfish::volume::v1_10_1::OperationOperationN1),
        VolumeOperationType(crate::swordfish::volume::OperationType),
    }
    impl Default for OperationOperation {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationOperationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationshipRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeleteTargetVolume")]
        pub delete_target_volume: Option<bool>,
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationshipRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Volume {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::volume::v1_10_1::VolumeAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::volume::v1_10_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ALUA")]
        pub alua: Option<crate::swordfish::volume::v1_10_1::VolumeALUA>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Compressed")]
        pub compressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::volume::v1_10_1::VolumeDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionTypes")]
        pub encryption_types: Option<Vec<crate::swordfish::volume::EncryptionTypes>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeMethod")]
        pub initialize_method: Option<crate::swordfish::volume::v1_10_1::VolumeInitializeMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOPerfModeEnabled")]
        pub io_perf_mode_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsBootCapable")]
        pub is_boot_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::volume::v1_10_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalUnitNumber")]
        pub logical_unit_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaSpanCount")]
        pub media_span_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeNamespaceProperties"
        )]
        pub nvme_namespace_properties:
            Option<crate::swordfish::volume::v1_10_1::VolumeNVMeNamespaceProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operations")]
        pub operations: Option<Vec<crate::swordfish::volume::v1_10_1::Operation>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OptimumIOSizeBytes")]
        pub optimum_io_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::volume::v1_10_1::VolumeProvisioningPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RAIDType")]
        pub raid_type: Option<crate::swordfish::volume::v1_10_1::VolumeRAIDType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadCachePolicy")]
        pub read_cache_policy: Option<crate::swordfish::volume::v1_10_1::VolumeReadCachePolicy>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteReplicaTargets"
        )]
        pub remote_replica_targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicationEnabled")]
        pub replication_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StripSizeBytes")]
        pub strip_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeType")]
        pub volume_type: Option<crate::swordfish::volume::v1_10_1::VolumeVolumeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeUsage")]
        pub volume_usage: Option<crate::swordfish::volume::v1_10_1::VolumeVolumeUsage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCachePolicy")]
        pub write_cache_policy: Option<crate::swordfish::volume::v1_10_1::VolumeWriteCachePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCacheState")]
        pub write_cache_state: Option<crate::swordfish::volume::v1_10_1::VolumeWriteCacheState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WriteHoleProtectionPolicy"
        )]
        pub write_hole_protection_policy:
            Option<crate::swordfish::volume::WriteHoleProtectionPolicyType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeALUA {
        V011001(crate::swordfish::volume::v1_10_1::ALUA),
        V000001(crate::swordfish::volume::v1_10_1::VolumeALUAN1),
    }
    impl Default for VolumeALUA {
        fn default() -> Self {
            Self::V011001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeALUAN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeAccessCapabilities {
        V000001(crate::swordfish::volume::v1_10_1::VolumeAccessCapabilitiesN1),
        DataStorageLoSCapabilitiesStorageAccessCapability(
            crate::swordfish::data_storage_los_capabilities::StorageAccessCapability,
        ),
    }
    impl Default for VolumeAccessCapabilities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeDescription {
        V000001(crate::swordfish::volume::v1_10_1::VolumeDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VolumeDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInitializeMethod {
        V000001(crate::swordfish::volume::v1_10_1::VolumeInitializeMethodN1),
        VolumeInitializeMethod(crate::swordfish::volume::InitializeMethod),
    }
    impl Default for VolumeInitializeMethod {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInitializeMethodN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeNVMeNamespaceProperties {
        V011001(crate::swordfish::volume::v1_10_1::NVMeNamespaceProperties),
        V000001(crate::swordfish::volume::v1_10_1::VolumeNVMeNamespacePropertiesN1),
    }
    impl Default for VolumeNVMeNamespaceProperties {
        fn default() -> Self {
            Self::V011001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeNVMeNamespacePropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeProvisioningPolicy {
        V000001(crate::swordfish::volume::v1_10_1::VolumeProvisioningPolicyN1),
        DataStorageLoSCapabilitiesProvisioningPolicy(
            crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy,
        ),
    }
    impl Default for VolumeProvisioningPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeProvisioningPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeRAIDType {
        V000001(crate::swordfish::volume::v1_10_1::VolumeRAIDTypeN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for VolumeRAIDType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeRAIDTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeReadCachePolicy {
        V000001(crate::swordfish::volume::v1_10_1::VolumeReadCachePolicyN1),
        VolumeReadCachePolicyType(crate::swordfish::volume::ReadCachePolicyType),
    }
    impl Default for VolumeReadCachePolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeReadCachePolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeVolumeType {
        V000001(crate::swordfish::volume::v1_10_1::VolumeVolumeTypeN1),
        VolumeVolumeType(crate::swordfish::volume::VolumeType),
    }
    impl Default for VolumeVolumeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeVolumeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeVolumeUsage {
        V000001(crate::swordfish::volume::v1_10_1::VolumeVolumeUsageN1),
        VolumeVolumeUsageType(crate::swordfish::volume::VolumeUsageType),
    }
    impl Default for VolumeVolumeUsage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeVolumeUsageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeWriteCachePolicy {
        V000001(crate::swordfish::volume::v1_10_1::VolumeWriteCachePolicyN1),
        VolumeWriteCachePolicyType(crate::swordfish::volume::WriteCachePolicyType),
    }
    impl Default for VolumeWriteCachePolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeWriteCachePolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeWriteCacheState {
        V000001(crate::swordfish::volume::v1_10_1::VolumeWriteCacheStateN1),
        VolumeWriteCacheStateType(crate::swordfish::volume::WriteCacheStateType),
    }
    impl Default for VolumeWriteCacheState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeWriteCacheStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_10_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ALUA {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ANAGroupId")]
        pub ana_group_id: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::volume::v1_10_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.AssignReplicaTarget"
        )]
        pub volume_assign_replica_target:
            Option<crate::swordfish::volume::v1_10_2::AssignReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ChangeRAIDLayout"
        )]
        pub volume_change_raid_layout: Option<crate::swordfish::volume::v1_10_2::ChangeRAIDLayout>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.CheckConsistency"
        )]
        pub volume_check_consistency: Option<crate::swordfish::volume::v1_10_2::CheckConsistency>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.CreateReplicaTarget"
        )]
        pub volume_create_replica_target:
            Option<crate::swordfish::volume::v1_10_2::CreateReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ForceEnable"
        )]
        pub volume_force_enable: Option<crate::swordfish::volume::v1_10_2::ForceEnable>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Volume.Initialize")]
        pub volume_initialize: Option<crate::swordfish::volume::v1_10_2::Initialize>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.RemoveReplicaRelationship"
        )]
        pub volume_remove_replica_relationship:
            Option<crate::swordfish::volume::v1_10_2::RemoveReplicaRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ResumeReplication"
        )]
        pub volume_resume_replication: Option<crate::swordfish::volume::v1_10_2::ResumeReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.ReverseReplicationRelationship"
        )]
        pub volume_reverse_replication_relationship:
            Option<crate::swordfish::volume::v1_10_2::ReverseReplicationRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.SplitReplication"
        )]
        pub volume_split_replication: Option<crate::swordfish::volume::v1_10_2::SplitReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Volume.SuspendReplication"
        )]
        pub volume_suspend_replication:
            Option<crate::swordfish::volume::v1_10_2::SuspendReplication>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangeRAIDLayout {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangeRAIDLayoutRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaSpanCount")]
        pub media_span_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RAIDType")]
        pub raid_type: Option<crate::swordfish::volume::RAIDType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StripSizeBytes")]
        pub strip_size_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CheckConsistency {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CheckConsistencyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetStoragePool")]
        pub target_storage_pool: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeName")]
        pub volume_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceEnable {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceEnableRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Initialize {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InitializeRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeMethod")]
        pub initialize_method: Option<crate::swordfish::volume::InitializeMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeType")]
        pub initialize_type: Option<crate::swordfish::volume::InitializeType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LBAFormat {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBADataSizeBytes")]
        pub lba_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormatType")]
        pub lba_format_type: Option<crate::swordfish::volume::v1_10_2::LBAFormatLBAFormatType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LBAMetadataSizeBytes"
        )]
        pub lba_metadata_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelativePerformance"
        )]
        pub relative_performance:
            Option<crate::swordfish::volume::v1_10_2::LBAFormatRelativePerformance>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LBAFormatLBAFormatType {
        V000001(crate::swordfish::volume::v1_10_2::LBAFormatLBAFormatTypeN1),
        VolumeLBAFormatType(crate::swordfish::volume::LBAFormatType),
    }
    impl Default for LBAFormatLBAFormatType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LBAFormatLBAFormatTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LBAFormatRelativePerformance {
        V000001(crate::swordfish::volume::v1_10_2::LBAFormatRelativePerformanceN1),
        VolumeLBARelativePerformanceType(crate::swordfish::volume::LBARelativePerformanceType),
    }
    impl Default for LBAFormatRelativePerformance {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LBAFormatRelativePerformanceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheDataVolumes")]
        pub cache_data_volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CacheDataVolumes@odata.count"
        )]
        pub cache_data_volumes_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheVolumeSource")]
        pub cache_volume_source: Option<crate::swordfish::volume::v1_10_2::LinksCacheVolumeSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientEndpoints")]
        pub client_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpoints@odata.count"
        )]
        pub client_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyGroups")]
        pub consistency_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyGroups@odata.count"
        )]
        pub consistency_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Controllers@odata.count"
        )]
        pub controllers_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JournalingMedia")]
        pub journaling_media: Option<crate::swordfish::volume::v1_10_2::LinksJournalingMedia>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OwningStorageResource"
        )]
        pub owning_storage_resource: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OwningStorageService"
        )]
        pub owning_storage_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidingStoragePool"
        )]
        pub providing_storage_pool:
            Option<crate::swordfish::volume::v1_10_2::LinksProvidingStoragePool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerEndpoints")]
        pub server_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpoints@odata.count"
        )]
        pub server_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareResourceSets")]
        pub spare_resource_sets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareResourceSets@odata.count"
        )]
        pub spare_resource_sets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageGroups@odata.count"
        )]
        pub storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksCacheVolumeSource {
        V000001(crate::swordfish::volume::v1_10_2::LinksCacheVolumeSourceN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksCacheVolumeSource {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksCacheVolumeSourceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksJournalingMedia {
        V000001(crate::swordfish::volume::v1_10_2::LinksJournalingMediaN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksJournalingMedia {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksJournalingMediaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksProvidingStoragePool {
        V000001(crate::swordfish::volume::v1_10_2::LinksProvidingStoragePoolN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksProvidingStoragePool {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksProvidingStoragePoolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeNamespaceProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FormattedLBASize")]
        pub formatted_lba_size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsShareable")]
        pub is_shareable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormat")]
        pub lba_format: Option<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormat>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LBAFormats")]
        pub lba_formats:
            Option<Vec<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormats>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LBAFormatsSupported"
        )]
        pub lba_formats_supported: Option<
            Vec<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormatsSupported>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetadataTransferredAtEndOfDataLBA"
        )]
        pub metadata_transferred_at_end_of_data_lba: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceFeatures")]
        pub namespace_features:
            Option<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesNamespaceFeatures>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceId")]
        pub namespace_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NamespaceType")]
        pub namespace_type:
            Option<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesNamespaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NumberLBAFormats")]
        pub number_lba_formats: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeVersion")]
        pub nvme_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsIOPerformanceHints"
        )]
        pub supports_io_performance_hints: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsMultipleNamespaceAttachments"
        )]
        pub supports_multiple_namespace_attachments: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormat {
        V011002(crate::swordfish::volume::v1_10_2::LBAFormat),
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormatN1),
    }
    impl Default for NVMeNamespacePropertiesLBAFormat {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormats {
        V011002(crate::swordfish::volume::v1_10_2::LBAFormat),
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormatsN1),
    }
    impl Default for NVMeNamespacePropertiesLBAFormats {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesLBAFormatsSupported {
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesLBAFormatsSupportedN1),
        VolumeLBAFormatType(crate::swordfish::volume::LBAFormatType),
    }
    impl Default for NVMeNamespacePropertiesLBAFormatsSupported {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesLBAFormatsSupportedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesNamespaceFeatures {
        V011002(crate::swordfish::volume::v1_10_2::NamespaceFeatures),
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesNamespaceFeaturesN1),
    }
    impl Default for NVMeNamespacePropertiesNamespaceFeatures {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesNamespaceFeaturesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesNamespaceType {
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesNamespaceTypeN1),
        VolumeNamespaceType(crate::swordfish::volume::NamespaceType),
    }
    impl Default for NVMeNamespacePropertiesNamespaceType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesNamespaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeNamespacePropertiesType {
        V000001(crate::swordfish::volume::v1_10_2::NVMeNamespacePropertiesTypeN1),
        VolumeNamespaceType(crate::swordfish::volume::NamespaceType),
    }
    impl Default for NVMeNamespacePropertiesType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeNamespacePropertiesTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NamespaceFeatures {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsAtomicTransactionSize"
        )]
        pub supports_atomic_transaction_size: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsDeallocatedOrUnwrittenLBError"
        )]
        pub supports_deallocated_or_unwritten_lb_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsIOPerformanceHints"
        )]
        pub supports_io_performance_hints: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsNGUIDReuse")]
        pub supports_nguid_reuse: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsThinProvisioning"
        )]
        pub supports_thin_provisioning: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Operation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedFeaturesRegistry"
        )]
        pub associated_features_registry: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operation")]
        pub operation: Option<crate::swordfish::volume::v1_10_2::OperationOperation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperationOperation {
        V000001(crate::swordfish::volume::v1_10_2::OperationOperationN1),
        VolumeOperationType(crate::swordfish::volume::OperationType),
    }
    impl Default for OperationOperation {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationOperationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationshipRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeleteTargetVolume")]
        pub delete_target_volume: Option<bool>,
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationshipRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplicationRequestBody {
        #[serde(rename = "TargetVolume")]
        pub target_volume: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Volume {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::swordfish::volume::v1_10_2::VolumeAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::volume::v1_10_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedPools")]
        pub allocated_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ALUA")]
        pub alua: Option<crate::swordfish::volume::v1_10_2::VolumeALUA>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capacity")]
        pub capacity: Option<crate::swordfish::capacity::Capacity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacitySources")]
        pub capacity_sources: Option<Vec<crate::swordfish::capacity::CapacitySource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacitySources@odata.count"
        )]
        pub capacity_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Compressed")]
        pub compressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Deduplicated")]
        pub deduplicated: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::volume::v1_10_2::VolumeDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Encrypted")]
        pub encrypted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionTypes")]
        pub encryption_types: Option<Vec<crate::swordfish::volume::EncryptionTypes>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitializeMethod")]
        pub initialize_method: Option<crate::swordfish::volume::v1_10_2::VolumeInitializeMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOPerfModeEnabled")]
        pub io_perf_mode_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::IOStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsBootCapable")]
        pub is_boot_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::volume::v1_10_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalUnitNumber")]
        pub logical_unit_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBlockSizeBytes")]
        pub max_block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaSpanCount")]
        pub media_span_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeNamespaceProperties"
        )]
        pub nvme_namespace_properties:
            Option<crate::swordfish::volume::v1_10_2::VolumeNVMeNamespaceProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operations")]
        pub operations: Option<Vec<crate::swordfish::volume::v1_10_2::Operation>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OptimumIOSizeBytes")]
        pub optimum_io_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisioningPolicy")]
        pub provisioning_policy:
            Option<crate::swordfish::volume::v1_10_2::VolumeProvisioningPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RAIDType")]
        pub raid_type: Option<crate::swordfish::volume::v1_10_2::VolumeRAIDType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadCachePolicy")]
        pub read_cache_policy: Option<crate::swordfish::volume::v1_10_2::VolumeReadCachePolicy>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteReplicaTargets"
        )]
        pub remote_replica_targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicationEnabled")]
        pub replication_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StripSizeBytes")]
        pub strip_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeType")]
        pub volume_type: Option<crate::swordfish::volume::v1_10_2::VolumeVolumeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeUsage")]
        pub volume_usage: Option<crate::swordfish::volume::v1_10_2::VolumeVolumeUsage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCachePolicy")]
        pub write_cache_policy: Option<crate::swordfish::volume::v1_10_2::VolumeWriteCachePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCacheState")]
        pub write_cache_state: Option<crate::swordfish::volume::v1_10_2::VolumeWriteCacheState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WriteHoleProtectionPolicy"
        )]
        pub write_hole_protection_policy:
            Option<crate::swordfish::volume::WriteHoleProtectionPolicyType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeALUA {
        V011002(crate::swordfish::volume::v1_10_2::ALUA),
        V000001(crate::swordfish::volume::v1_10_2::VolumeALUAN1),
    }
    impl Default for VolumeALUA {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeALUAN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeAccessCapabilities {
        V000001(crate::swordfish::volume::v1_10_2::VolumeAccessCapabilitiesN1),
        DataStorageLoSCapabilitiesStorageAccessCapability(
            crate::swordfish::data_storage_los_capabilities::StorageAccessCapability,
        ),
    }
    impl Default for VolumeAccessCapabilities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeDescription {
        V000001(crate::swordfish::volume::v1_10_2::VolumeDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VolumeDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInitializeMethod {
        V000001(crate::swordfish::volume::v1_10_2::VolumeInitializeMethodN1),
        VolumeInitializeMethod(crate::swordfish::volume::InitializeMethod),
    }
    impl Default for VolumeInitializeMethod {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInitializeMethodN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeNVMeNamespaceProperties {
        V011002(crate::swordfish::volume::v1_10_2::NVMeNamespaceProperties),
        V000001(crate::swordfish::volume::v1_10_2::VolumeNVMeNamespacePropertiesN1),
    }
    impl Default for VolumeNVMeNamespaceProperties {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeNVMeNamespacePropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeProvisioningPolicy {
        V000001(crate::swordfish::volume::v1_10_2::VolumeProvisioningPolicyN1),
        DataStorageLoSCapabilitiesProvisioningPolicy(
            crate::swordfish::data_storage_los_capabilities::ProvisioningPolicy,
        ),
    }
    impl Default for VolumeProvisioningPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeProvisioningPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeRAIDType {
        V000001(crate::swordfish::volume::v1_10_2::VolumeRAIDTypeN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for VolumeRAIDType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeRAIDTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeReadCachePolicy {
        V000001(crate::swordfish::volume::v1_10_2::VolumeReadCachePolicyN1),
        VolumeReadCachePolicyType(crate::swordfish::volume::ReadCachePolicyType),
    }
    impl Default for VolumeReadCachePolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeReadCachePolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeVolumeType {
        V000001(crate::swordfish::volume::v1_10_2::VolumeVolumeTypeN1),
        VolumeVolumeType(crate::swordfish::volume::VolumeType),
    }
    impl Default for VolumeVolumeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeVolumeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeVolumeUsage {
        V000001(crate::swordfish::volume::v1_10_2::VolumeVolumeUsageN1),
        VolumeVolumeUsageType(crate::swordfish::volume::VolumeUsageType),
    }
    impl Default for VolumeVolumeUsage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeVolumeUsageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeWriteCachePolicy {
        V000001(crate::swordfish::volume::v1_10_2::VolumeWriteCachePolicyN1),
        VolumeWriteCachePolicyType(crate::swordfish::volume::WriteCachePolicyType),
    }
    impl Default for VolumeWriteCachePolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeWriteCachePolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeWriteCacheState {
        V000001(crate::swordfish::volume::v1_10_2::VolumeWriteCacheStateN1),
        VolumeWriteCacheStateType(crate::swordfish::volume::WriteCacheStateType),
    }
    impl Default for VolumeWriteCacheState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeWriteCacheStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
