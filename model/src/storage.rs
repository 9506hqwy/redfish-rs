pub type Storage = crate::storage::v1_20_0::Storage;
pub type StorageController = crate::storage::v1_20_0::StorageController;
pub mod v1_19_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage::v1_19_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.ImportForeignDrives"
        )]
        pub storage_import_foreign_drives: Option<crate::storage::v1_19_0::ImportForeignDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.RekeyExternalKey"
        )]
        pub storage_rekey_external_key: Option<crate::storage::v1_19_0::RekeyExternalKey>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.ResetToDefaults"
        )]
        pub storage_reset_to_defaults: Option<crate::storage::v1_19_0::ResetToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.SetControllerPassword"
        )]
        pub storage_set_controller_password: Option<crate::storage::v1_19_0::SetControllerPassword>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.SetEncryptionKey"
        )]
        pub storage_set_encryption_key: Option<crate::storage::v1_19_0::SetEncryptionKey>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutoVolumeCreate {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "NonRAID")]
        NonRAID,
        #[serde(rename = "RAID0")]
        RAID0,
        #[serde(rename = "RAID1")]
        RAID1,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentCacheSizeMiB"
        )]
        pub persistent_cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TotalCacheSizeMiB")]
        pub total_cache_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConfigLockOptions {
        #[default]
        #[serde(rename = "CommandUnsupported")]
        CommandUnsupported,
        #[serde(rename = "LockdownUnsupported")]
        LockdownUnsupported,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConfigurationLock {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Partial")]
        Partial,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "PasswordOnly")]
        PasswordOnly,
        #[serde(rename = "PasswordWithExternalKey")]
        PasswordWithExternalKey,
        #[serde(rename = "PasswordWithLocalKey")]
        PasswordWithLocalKey,
        #[serde(rename = "UseExternalKey")]
        UseExternalKey,
        #[serde(rename = "UseLocalKey")]
        UseLocalKey,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareActivationPolicy {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OnDriveFailure")]
        OnDriveFailure,
        #[serde(rename = "OnDrivePredictedFailure")]
        OnDrivePredictedFailure,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ImportForeignDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ImportForeignDrivesRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerPassword")]
        pub controller_password: Option<String>,
        #[serde(rename = "DriveEncryptionKey")]
        pub drive_encryption_key: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DriveEncryptionKeyIdentifier"
        )]
        pub drive_encryption_key_identifier: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUnsupportedDrives"
        )]
        pub block_security_id_unsupported_drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUnsupportedDrives@odata.count"
        )]
        pub block_security_id_unsupported_drives_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUpdateUnsuccessfulDrives"
        )]
        pub block_security_id_update_unsuccessful_drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUpdateUnsuccessfulDrives@odata.count"
        )]
        pub block_security_id_update_unsuccessful_drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enclosures")]
        pub enclosures: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Enclosures@odata.count"
        )]
        pub enclosures_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingStorageSystems"
        )]
        pub hosting_storage_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingStorageSystems@odata.count"
        )]
        pub hosting_storage_systems_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeoFDiscoverySubsystems"
        )]
        pub nvme_of_discovery_subsystems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeoFDiscoverySubsystems@odata.count"
        )]
        pub nvme_of_discovery_subsystems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageServices@odata.count"
        )]
        pub storage_services_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeConfigurationLockState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareCommit")]
        pub firmware_commit:
            Option<crate::storage::v1_19_0::NVMeConfigurationLockStateFirmwareCommit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareImageDownload"
        )]
        pub firmware_image_download:
            Option<crate::storage::v1_19_0::NVMeConfigurationLockStateFirmwareImageDownload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lockdown")]
        pub lockdown: Option<crate::storage::v1_19_0::NVMeConfigurationLockStateLockdown>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecuritySend")]
        pub security_send: Option<crate::storage::v1_19_0::NVMeConfigurationLockStateSecuritySend>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VPDWrite")]
        pub vpd_write: Option<crate::storage::v1_19_0::NVMeConfigurationLockStateVPDWrite>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateFirmwareCommit {
        V011900(crate::storage::v1_19_0::ConfigLockOptions),
        V000001(crate::storage::v1_19_0::NVMeConfigurationLockStateFirmwareCommitN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareCommit {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateFirmwareCommitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateFirmwareImageDownload {
        V011900(crate::storage::v1_19_0::ConfigLockOptions),
        V000001(crate::storage::v1_19_0::NVMeConfigurationLockStateFirmwareImageDownloadN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareImageDownload {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateFirmwareImageDownloadN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateLockdown {
        V011900(crate::storage::v1_19_0::ConfigLockOptions),
        V000001(crate::storage::v1_19_0::NVMeConfigurationLockStateLockdownN1),
    }
    impl Default for NVMeConfigurationLockStateLockdown {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateLockdownN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateSecuritySend {
        V011900(crate::storage::v1_19_0::ConfigLockOptions),
        V000001(crate::storage::v1_19_0::NVMeConfigurationLockStateSecuritySendN1),
    }
    impl Default for NVMeConfigurationLockStateSecuritySend {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateSecuritySendN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateVPDWrite {
        V011900(crate::storage::v1_19_0::ConfigLockOptions),
        V000001(crate::storage::v1_19_0::NVMeConfigurationLockStateVPDWriteN1),
    }
    impl Default for NVMeConfigurationLockStateVPDWrite {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateVPDWriteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSubsystemProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockState"
        )]
        pub configuration_lock_state:
            Option<crate::storage::v1_19_0::NVMeSubsystemPropertiesConfigurationLockState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxNamespacesSupported"
        )]
        pub max_namespaces_supported: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedNamespaceControllerAttachmentSupported"
        )]
        pub shared_namespace_controller_attachment_supported: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeSubsystemPropertiesConfigurationLockState {
        V011900(crate::storage::v1_19_0::NVMeConfigurationLockState),
        V000001(crate::storage::v1_19_0::NVMeSubsystemPropertiesConfigurationLockStateN1),
    }
    impl Default for NVMeSubsystemPropertiesConfigurationLockState {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeSubsystemPropertiesConfigurationLockStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rates {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckRatePercent"
        )]
        pub consistency_check_rate_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RebuildRatePercent")]
        pub rebuild_rate_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransformationRatePercent"
        )]
        pub transformation_rate_percent: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyExternalKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyExternalKeyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {
        #[serde(rename = "ResetType")]
        pub reset_type: crate::storage::v1_19_0::ResetToDefaultsType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetToDefaultsType {
        #[default]
        #[serde(rename = "PreserveVolumes")]
        PreserveVolumes,
        #[serde(rename = "ResetAll")]
        ResetAll,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetControllerPassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetControllerPasswordRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPassword")]
        pub current_password: Option<String>,
        #[serde(rename = "NewPassword")]
        pub new_password: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityKey")]
        pub security_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionKeyRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentEncryptionKey"
        )]
        pub current_encryption_key: Option<String>,
        #[serde(rename = "EncryptionKey")]
        pub encryption_key: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EncryptionKeyIdentifier"
        )]
        pub encryption_key_identifier: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Storage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage::v1_19_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoVolumeCreate")]
        pub auto_volume_create: Option<crate::storage::v1_19_0::StorageAutoVolumeCreate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDPolicy"
        )]
        pub block_security_id_policy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfigurationLock")]
        pub configuration_lock: Option<crate::storage::v1_19_0::StorageConfigurationLock>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyGroups")]
        pub consistency_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::storage::v1_19_0::StorageDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionMode")]
        pub encryption_mode: Option<crate::storage::v1_19_0::StorageEncryptionMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointGroups")]
        pub endpoint_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystems")]
        pub file_systems: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareActivationPolicy"
        )]
        pub hotspare_activation_policy:
            Option<crate::storage::v1_19_0::StorageHotspareActivationPolicy>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage::v1_19_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalEncryptionKeyIdentifier"
        )]
        pub local_encryption_key_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::storage::v1_19_0::StorageMetrics>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeSubsystemProperties"
        )]
        pub nvme_subsystem_properties:
            Option<crate::storage::v1_19_0::StorageNVMeSubsystemProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageControllers")]
        pub storage_controllers: Option<Vec<crate::storage::v1_19_0::StorageController>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageControllers@odata.count"
        )]
        pub storage_controllers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StoragePools")]
        pub storage_pools: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetConfigurationLockLevel"
        )]
        pub target_configuration_lock_level:
            Option<crate::storage::v1_19_0::StorageTargetConfigurationLockLevel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageAutoVolumeCreate {
        V011900(crate::storage::v1_19_0::AutoVolumeCreate),
        V000001(crate::storage::v1_19_0::StorageAutoVolumeCreateN1),
    }
    impl Default for StorageAutoVolumeCreate {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageAutoVolumeCreateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageConfigurationLock {
        V011900(crate::storage::v1_19_0::ConfigurationLock),
        V000001(crate::storage::v1_19_0::StorageConfigurationLockN1),
    }
    impl Default for StorageConfigurationLock {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageConfigurationLockN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageController {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage::v1_19_0::StorageControllerActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSummary")]
        pub cache_summary: Option<crate::storage::v1_19_0::CacheSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerRates")]
        pub controller_rates: Option<crate::storage::v1_19_0::Rates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage::v1_19_0::StorageControllerLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedGbps")]
        pub speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedControllerProtocols"
        )]
        pub supported_controller_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDeviceProtocols"
        )]
        pub supported_device_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types:
            Option<Vec<crate::storage::v1_19_0::StorageControllerSupportedRAIDTypes>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage::v1_19_0::StorageControllerOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerLinks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageServices@odata.count"
        )]
        pub storage_services_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerOemActions {}
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerSupportedRAIDTypes {
        V000001(crate::storage::v1_19_0::StorageControllerSupportedRAIDTypesN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for StorageControllerSupportedRAIDTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerSupportedRAIDTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageDescription {
        V000001(crate::storage::v1_19_0::StorageDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageEncryptionMode {
        V011900(crate::storage::v1_19_0::EncryptionMode),
        V000001(crate::storage::v1_19_0::StorageEncryptionModeN1),
    }
    impl Default for StorageEncryptionMode {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageEncryptionModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageHotspareActivationPolicy {
        V011900(crate::storage::v1_19_0::HotspareActivationPolicy),
        V000001(crate::storage::v1_19_0::StorageHotspareActivationPolicyN1),
    }
    impl Default for StorageHotspareActivationPolicy {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageHotspareActivationPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageMetrics {
        V000001(crate::storage::v1_19_0::StorageMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for StorageMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageNVMeSubsystemProperties {
        V011900(crate::storage::v1_19_0::NVMeSubsystemProperties),
        V000001(crate::storage::v1_19_0::StorageNVMeSubsystemPropertiesN1),
    }
    impl Default for StorageNVMeSubsystemProperties {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageNVMeSubsystemPropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageTargetConfigurationLockLevel {
        V011900(crate::storage::v1_19_0::TargetConfigurationLockLevel),
        V000001(crate::storage::v1_19_0::StorageTargetConfigurationLockLevelN1),
    }
    impl Default for StorageTargetConfigurationLockLevel {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageTargetConfigurationLockLevelN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TargetConfigurationLockLevel {
        #[default]
        #[serde(rename = "Baseline")]
        Baseline,
    }
}
pub mod v1_20_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage::v1_20_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.ImportForeignDrives"
        )]
        pub storage_import_foreign_drives: Option<crate::storage::v1_20_0::ImportForeignDrives>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.RekeyExternalKey"
        )]
        pub storage_rekey_external_key: Option<crate::storage::v1_20_0::RekeyExternalKey>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.ResetToDefaults"
        )]
        pub storage_reset_to_defaults: Option<crate::storage::v1_20_0::ResetToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.SetControllerPassword"
        )]
        pub storage_set_controller_password: Option<crate::storage::v1_20_0::SetControllerPassword>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Storage.SetEncryptionKey"
        )]
        pub storage_set_encryption_key: Option<crate::storage::v1_20_0::SetEncryptionKey>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutoVolumeCreate {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "NonRAID")]
        NonRAID,
        #[serde(rename = "RAID0")]
        RAID0,
        #[serde(rename = "RAID1")]
        RAID1,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentCacheSizeMiB"
        )]
        pub persistent_cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TotalCacheSizeMiB")]
        pub total_cache_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConfigLockOptions {
        #[default]
        #[serde(rename = "CommandUnsupported")]
        CommandUnsupported,
        #[serde(rename = "LockdownUnsupported")]
        LockdownUnsupported,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConfigurationLock {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Partial")]
        Partial,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "PasswordOnly")]
        PasswordOnly,
        #[serde(rename = "PasswordWithExternalKey")]
        PasswordWithExternalKey,
        #[serde(rename = "PasswordWithLocalKey")]
        PasswordWithLocalKey,
        #[serde(rename = "UseExternalKey")]
        UseExternalKey,
        #[serde(rename = "UseLocalKey")]
        UseLocalKey,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareActivationPolicy {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OnDriveFailure")]
        OnDriveFailure,
        #[serde(rename = "OnDrivePredictedFailure")]
        OnDrivePredictedFailure,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ImportForeignDrives {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ImportForeignDrivesRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerPassword")]
        pub controller_password: Option<String>,
        #[serde(rename = "DriveEncryptionKey")]
        pub drive_encryption_key: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DriveEncryptionKeyIdentifier"
        )]
        pub drive_encryption_key_identifier: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUnsupportedDrives"
        )]
        pub block_security_id_unsupported_drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUnsupportedDrives@odata.count"
        )]
        pub block_security_id_unsupported_drives_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUpdateUnsuccessfulDrives"
        )]
        pub block_security_id_update_unsuccessful_drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDUpdateUnsuccessfulDrives@odata.count"
        )]
        pub block_security_id_update_unsuccessful_drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enclosures")]
        pub enclosures: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Enclosures@odata.count"
        )]
        pub enclosures_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingStorageSystems"
        )]
        pub hosting_storage_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingStorageSystems@odata.count"
        )]
        pub hosting_storage_systems_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeoFDiscoverySubsystems"
        )]
        pub nvme_of_discovery_subsystems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeoFDiscoverySubsystems@odata.count"
        )]
        pub nvme_of_discovery_subsystems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageServices@odata.count"
        )]
        pub storage_services_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UnassignedVolumes")]
        pub unassigned_volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnassignedVolumes@odata.count"
        )]
        pub unassigned_volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MPF {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredPhysicalFunctions"
        )]
        pub configured_physical_functions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaximumSupportedPhysicalFunctions"
        )]
        pub maximum_supported_physical_functions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolumeAssignmentPolicy"
        )]
        pub volume_assignment_policy: Option<crate::storage::v1_20_0::MPFVolumeAssignmentPolicy>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MPFVolumeAssignmentPolicy {
        V012000(crate::storage::v1_20_0::VolumeAssignmentPolicy),
        V000001(crate::storage::v1_20_0::MPFVolumeAssignmentPolicyN1),
    }
    impl Default for MPFVolumeAssignmentPolicy {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MPFVolumeAssignmentPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeConfigurationLockState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareCommit")]
        pub firmware_commit:
            Option<crate::storage::v1_20_0::NVMeConfigurationLockStateFirmwareCommit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareImageDownload"
        )]
        pub firmware_image_download:
            Option<crate::storage::v1_20_0::NVMeConfigurationLockStateFirmwareImageDownload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lockdown")]
        pub lockdown: Option<crate::storage::v1_20_0::NVMeConfigurationLockStateLockdown>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecuritySend")]
        pub security_send: Option<crate::storage::v1_20_0::NVMeConfigurationLockStateSecuritySend>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VPDWrite")]
        pub vpd_write: Option<crate::storage::v1_20_0::NVMeConfigurationLockStateVPDWrite>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateFirmwareCommit {
        V012000(crate::storage::v1_20_0::ConfigLockOptions),
        V000001(crate::storage::v1_20_0::NVMeConfigurationLockStateFirmwareCommitN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareCommit {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateFirmwareCommitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateFirmwareImageDownload {
        V012000(crate::storage::v1_20_0::ConfigLockOptions),
        V000001(crate::storage::v1_20_0::NVMeConfigurationLockStateFirmwareImageDownloadN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareImageDownload {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateFirmwareImageDownloadN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateLockdown {
        V012000(crate::storage::v1_20_0::ConfigLockOptions),
        V000001(crate::storage::v1_20_0::NVMeConfigurationLockStateLockdownN1),
    }
    impl Default for NVMeConfigurationLockStateLockdown {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateLockdownN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateSecuritySend {
        V012000(crate::storage::v1_20_0::ConfigLockOptions),
        V000001(crate::storage::v1_20_0::NVMeConfigurationLockStateSecuritySendN1),
    }
    impl Default for NVMeConfigurationLockStateSecuritySend {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateSecuritySendN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateVPDWrite {
        V012000(crate::storage::v1_20_0::ConfigLockOptions),
        V000001(crate::storage::v1_20_0::NVMeConfigurationLockStateVPDWriteN1),
    }
    impl Default for NVMeConfigurationLockStateVPDWrite {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateVPDWriteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSubsystemProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockState"
        )]
        pub configuration_lock_state:
            Option<crate::storage::v1_20_0::NVMeSubsystemPropertiesConfigurationLockState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxNamespacesSupported"
        )]
        pub max_namespaces_supported: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedNamespaceControllerAttachmentSupported"
        )]
        pub shared_namespace_controller_attachment_supported: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeSubsystemPropertiesConfigurationLockState {
        V012000(crate::storage::v1_20_0::NVMeConfigurationLockState),
        V000001(crate::storage::v1_20_0::NVMeSubsystemPropertiesConfigurationLockStateN1),
    }
    impl Default for NVMeSubsystemPropertiesConfigurationLockState {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeSubsystemPropertiesConfigurationLockStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rates {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckRatePercent"
        )]
        pub consistency_check_rate_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RebuildRatePercent")]
        pub rebuild_rate_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransformationRatePercent"
        )]
        pub transformation_rate_percent: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyExternalKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyExternalKeyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {
        #[serde(rename = "ResetType")]
        pub reset_type: crate::storage::v1_20_0::ResetToDefaultsType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetToDefaultsType {
        #[default]
        #[serde(rename = "PreserveVolumes")]
        PreserveVolumes,
        #[serde(rename = "ResetAll")]
        ResetAll,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetControllerPassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetControllerPasswordRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPassword")]
        pub current_password: Option<String>,
        #[serde(rename = "NewPassword")]
        pub new_password: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityKey")]
        pub security_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetEncryptionKeyRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentEncryptionKey"
        )]
        pub current_encryption_key: Option<String>,
        #[serde(rename = "EncryptionKey")]
        pub encryption_key: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EncryptionKeyIdentifier"
        )]
        pub encryption_key_identifier: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Storage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage::v1_20_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoVolumeCreate")]
        pub auto_volume_create: Option<crate::storage::v1_20_0::StorageAutoVolumeCreate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDPolicy"
        )]
        pub block_security_id_policy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfigurationLock")]
        pub configuration_lock: Option<crate::storage::v1_20_0::StorageConfigurationLock>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyGroups")]
        pub consistency_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::storage::v1_20_0::StorageDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionMode")]
        pub encryption_mode: Option<crate::storage::v1_20_0::StorageEncryptionMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointGroups")]
        pub endpoint_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystems")]
        pub file_systems: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareActivationPolicy"
        )]
        pub hotspare_activation_policy:
            Option<crate::storage::v1_20_0::StorageHotspareActivationPolicy>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage::v1_20_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalEncryptionKeyIdentifier"
        )]
        pub local_encryption_key_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::storage::v1_20_0::StorageMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MPF")]
        pub mpf: Option<crate::storage::v1_20_0::StorageMPF>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeSubsystemProperties"
        )]
        pub nvme_subsystem_properties:
            Option<crate::storage::v1_20_0::StorageNVMeSubsystemProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageControllers")]
        pub storage_controllers: Option<Vec<crate::storage::v1_20_0::StorageController>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageControllers@odata.count"
        )]
        pub storage_controllers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageGroups")]
        pub storage_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StoragePools")]
        pub storage_pools: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetConfigurationLockLevel"
        )]
        pub target_configuration_lock_level:
            Option<crate::storage::v1_20_0::StorageTargetConfigurationLockLevel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageAutoVolumeCreate {
        V012000(crate::storage::v1_20_0::AutoVolumeCreate),
        V000001(crate::storage::v1_20_0::StorageAutoVolumeCreateN1),
    }
    impl Default for StorageAutoVolumeCreate {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageAutoVolumeCreateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageConfigurationLock {
        V012000(crate::storage::v1_20_0::ConfigurationLock),
        V000001(crate::storage::v1_20_0::StorageConfigurationLockN1),
    }
    impl Default for StorageConfigurationLock {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageConfigurationLockN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageController {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage::v1_20_0::StorageControllerActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSummary")]
        pub cache_summary: Option<crate::storage::v1_20_0::CacheSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerRates")]
        pub controller_rates: Option<crate::storage::v1_20_0::Rates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage::v1_20_0::StorageControllerLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedGbps")]
        pub speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedControllerProtocols"
        )]
        pub supported_controller_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDeviceProtocols"
        )]
        pub supported_device_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types:
            Option<Vec<crate::storage::v1_20_0::StorageControllerSupportedRAIDTypes>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage::v1_20_0::StorageControllerOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerLinks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageServices@odata.count"
        )]
        pub storage_services_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerOemActions {}
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerSupportedRAIDTypes {
        V000001(crate::storage::v1_20_0::StorageControllerSupportedRAIDTypesN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for StorageControllerSupportedRAIDTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerSupportedRAIDTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageDescription {
        V000001(crate::storage::v1_20_0::StorageDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageEncryptionMode {
        V012000(crate::storage::v1_20_0::EncryptionMode),
        V000001(crate::storage::v1_20_0::StorageEncryptionModeN1),
    }
    impl Default for StorageEncryptionMode {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageEncryptionModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageHotspareActivationPolicy {
        V012000(crate::storage::v1_20_0::HotspareActivationPolicy),
        V000001(crate::storage::v1_20_0::StorageHotspareActivationPolicyN1),
    }
    impl Default for StorageHotspareActivationPolicy {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageHotspareActivationPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageMPF {
        V012000(crate::storage::v1_20_0::MPF),
        V000001(crate::storage::v1_20_0::StorageMPFN1),
    }
    impl Default for StorageMPF {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageMPFN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageMetrics {
        V000001(crate::storage::v1_20_0::StorageMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for StorageMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageNVMeSubsystemProperties {
        V012000(crate::storage::v1_20_0::NVMeSubsystemProperties),
        V000001(crate::storage::v1_20_0::StorageNVMeSubsystemPropertiesN1),
    }
    impl Default for StorageNVMeSubsystemProperties {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageNVMeSubsystemPropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageTargetConfigurationLockLevel {
        V012000(crate::storage::v1_20_0::TargetConfigurationLockLevel),
        V000001(crate::storage::v1_20_0::StorageTargetConfigurationLockLevelN1),
    }
    impl Default for StorageTargetConfigurationLockLevel {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageTargetConfigurationLockLevelN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TargetConfigurationLockLevel {
        #[default]
        #[serde(rename = "Baseline")]
        Baseline,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeAssignmentPolicy {
        #[default]
        #[serde(rename = "Supervisor")]
        Supervisor,
        #[serde(rename = "Unassigned")]
        Unassigned,
        #[serde(rename = "WeightedRoundRobin")]
        WeightedRoundRobin,
    }
}
