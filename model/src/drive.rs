pub type Drive = crate::drive::v1_20_1::Drive;
pub mod v1_18_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_18_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Drive.RevertToOriginalFactoryState"
        )]
        pub drive_revert_to_original_factory_state:
            Option<crate::drive::v1_18_0::RevertToOriginalFactoryState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_18_0::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_18_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSanitizationType {
        #[default]
        #[serde(rename = "BlockErase")]
        BlockErase,
        #[serde(rename = "CryptographicErase")]
        CryptographicErase,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_18_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapableSpeedGbs")]
        pub capable_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DriveFormFactor")]
        pub drive_form_factor: Option<crate::drive::v1_18_0::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_18_0::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_18_0::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_18_0::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_18_0::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_18_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_18_0::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Multipath")]
        pub multipath: Option<bool>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NegotiatedSpeedGbs")]
        pub negotiated_speed_gbs: Option<f64>,
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
        pub operations: Option<Vec<crate::drive::v1_18_0::Operations>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalLocation")]
        pub physical_location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SlotCapableProtocols"
        )]
        pub slot_capable_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotFormFactor")]
        pub slot_form_factor: Option<crate::drive::v1_18_0::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_18_0::StatusIndicator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCacheEnabled")]
        pub write_cache_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionAbility {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "SelfEncryptingDrive")]
        SelfEncryptingDrive,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionStatus {
        #[default]
        #[serde(rename = "Foreign")]
        Foreign,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Unecrypted")]
        Unecrypted,
        #[serde(rename = "Unencrypted")]
        Unencrypted,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FormFactor {
        #[default]
        #[serde(rename = "Drive2_5")]
        Drive2N5,
        #[serde(rename = "Drive3_5")]
        Drive3N5,
        #[serde(rename = "EDSFF")]
        EDSFF,
        #[serde(rename = "EDSFF_1U_Long")]
        EDSFF1ULong,
        #[serde(rename = "EDSFF_1U_Short")]
        EDSFF1UShort,
        #[serde(rename = "EDSFF_E3_Long")]
        EDSFFE3Long,
        #[serde(rename = "EDSFF_E3_Short")]
        EDSFFE3Short,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "M2_22110")]
        M2N22110,
        #[serde(rename = "M2_2230")]
        M2N2230,
        #[serde(rename = "M2_2242")]
        M2N2242,
        #[serde(rename = "M2_2260")]
        M2N2260,
        #[serde(rename = "M2_2280")]
        M2N2280,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PCIeHalfLength")]
        PCIeHalfLength,
        #[serde(rename = "PCIeSlotFullLength")]
        PCIeSlotFullLength,
        #[serde(rename = "PCIeSlotLowProfile")]
        PCIeSlotLowProfile,
        #[serde(rename = "U2")]
        U2,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareReplacementModeType {
        #[default]
        #[serde(rename = "NonRevertible")]
        NonRevertible,
        #[serde(rename = "Revertible")]
        Revertible,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareType {
        #[default]
        #[serde(rename = "Chassis")]
        Chassis,
        #[serde(rename = "Dedicated")]
        Dedicated,
        #[serde(rename = "Global")]
        Global,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StoragePools")]
        pub storage_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoragePools@odata.count"
        )]
        pub storage_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediaType {
        #[default]
        #[serde(rename = "HDD")]
        HDD,
        #[serde(rename = "SMR")]
        SMR,
        #[serde(rename = "SSD")]
        SSD,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Operations {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedTask")]
        pub associated_task: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operation")]
        pub operation: Option<crate::swordfish::volume::OperationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RevertToOriginalFactoryState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RevertToOriginalFactoryStateRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSecureID")]
        pub physical_secure_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverwritePasses")]
        pub overwrite_passes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SanitizationType")]
        pub sanitization_type: Option<crate::drive::v1_18_0::DataSanitizationType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StatusIndicator {
        #[default]
        #[serde(rename = "Fail")]
        Fail,
        #[serde(rename = "Hotspare")]
        Hotspare,
        #[serde(rename = "InACriticalArray")]
        InACriticalArray,
        #[serde(rename = "InAFailedArray")]
        InAFailedArray,
        #[serde(rename = "OK")]
        OK,
        #[serde(rename = "PredictiveFailureAnalysis")]
        PredictiveFailureAnalysis,
        #[serde(rename = "Rebuild")]
        Rebuild,
    }
}
pub mod v1_20_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_20_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Drive.RevertToOriginalFactoryState"
        )]
        pub drive_revert_to_original_factory_state:
            Option<crate::drive::v1_20_1::RevertToOriginalFactoryState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_20_1::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_20_1::OemActions>,
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
    pub enum DataSanitizationType {
        #[default]
        #[serde(rename = "BlockErase")]
        BlockErase,
        #[serde(rename = "CryptographicErase")]
        CryptographicErase,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_20_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BlockSecurityIDEnabled"
        )]
        pub block_security_id_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapableSpeedGbs")]
        pub capable_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfigurationLock")]
        pub configuration_lock: Option<crate::drive::v1_20_1::DriveConfigurationLock>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::drive::v1_20_1::DriveDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DriveFormFactor")]
        pub drive_form_factor: Option<crate::drive::v1_20_1::DriveDriveFormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_20_1::DriveEncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_20_1::DriveEncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_20_1::DriveHotspareReplacementMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_20_1::DriveHotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::drive::v1_20_1::DriveIndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_20_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_20_1::DriveMediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::drive::v1_20_1::DriveMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Multipath")]
        pub multipath: Option<bool>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NegotiatedSpeedGbs")]
        pub negotiated_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMe")]
        pub nvme: Option<crate::drive::v1_20_1::DriveNVMe>,
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
        pub operations: Option<Vec<crate::drive::v1_20_1::Operations>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalLocation")]
        pub physical_location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::drive::v1_20_1::DriveProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SlotCapableProtocols"
        )]
        pub slot_capable_protocols: Option<Vec<crate::drive::v1_20_1::DriveSlotCapableProtocols>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotFormFactor")]
        pub slot_form_factor: Option<crate::drive::v1_20_1::DriveSlotFormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_20_1::DriveStatusIndicator>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetConfigurationLockLevel"
        )]
        pub target_configuration_lock_level:
            Option<crate::drive::v1_20_1::DriveTargetConfigurationLockLevel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteCacheEnabled")]
        pub write_cache_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveConfigurationLock {
        V012001(crate::drive::v1_20_1::ConfigurationLock),
        V000001(crate::drive::v1_20_1::DriveConfigurationLockN1),
    }
    impl Default for DriveConfigurationLock {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveConfigurationLockN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveDescription {
        V000001(crate::drive::v1_20_1::DriveDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for DriveDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveDriveFormFactor {
        V012001(crate::drive::v1_20_1::FormFactor),
        V000001(crate::drive::v1_20_1::DriveDriveFormFactorN1),
    }
    impl Default for DriveDriveFormFactor {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveDriveFormFactorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveEncryptionAbility {
        V012001(crate::drive::v1_20_1::EncryptionAbility),
        V000001(crate::drive::v1_20_1::DriveEncryptionAbilityN1),
    }
    impl Default for DriveEncryptionAbility {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveEncryptionAbilityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveEncryptionStatus {
        V012001(crate::drive::v1_20_1::EncryptionStatus),
        V000001(crate::drive::v1_20_1::DriveEncryptionStatusN1),
    }
    impl Default for DriveEncryptionStatus {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveEncryptionStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveHotspareReplacementMode {
        V012001(crate::drive::v1_20_1::HotspareReplacementModeType),
        V000001(crate::drive::v1_20_1::DriveHotspareReplacementModeN1),
    }
    impl Default for DriveHotspareReplacementMode {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveHotspareReplacementModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveHotspareType {
        V012001(crate::drive::v1_20_1::HotspareType),
        V000001(crate::drive::v1_20_1::DriveHotspareTypeN1),
    }
    impl Default for DriveHotspareType {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveHotspareTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveIndicatorLED {
        V000001(crate::drive::v1_20_1::DriveIndicatorLEDN1),
        ResourceIndicatorLED(crate::resource::IndicatorLED),
    }
    impl Default for DriveIndicatorLED {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveIndicatorLEDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveMediaType {
        V012001(crate::drive::v1_20_1::MediaType),
        V000001(crate::drive::v1_20_1::DriveMediaTypeN1),
    }
    impl Default for DriveMediaType {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveMediaTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveMetrics {
        V000001(crate::drive::v1_20_1::DriveMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for DriveMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveNVMe {
        V012001(crate::drive::v1_20_1::NVMe),
        V000001(crate::drive::v1_20_1::DriveNVMeN1),
    }
    impl Default for DriveNVMe {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveNVMeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveProtocol {
        V000001(crate::drive::v1_20_1::DriveProtocolN1),
        ProtocolProtocol(crate::protocol::Protocol),
    }
    impl Default for DriveProtocol {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveSlotCapableProtocols {
        V000001(crate::drive::v1_20_1::DriveSlotCapableProtocolsN1),
        ProtocolProtocol(crate::protocol::Protocol),
    }
    impl Default for DriveSlotCapableProtocols {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveSlotCapableProtocolsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveSlotFormFactor {
        V012001(crate::drive::v1_20_1::FormFactor),
        V000001(crate::drive::v1_20_1::DriveSlotFormFactorN1),
    }
    impl Default for DriveSlotFormFactor {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveSlotFormFactorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveStatusIndicator {
        V012001(crate::drive::v1_20_1::StatusIndicator),
        V000001(crate::drive::v1_20_1::DriveStatusIndicatorN1),
    }
    impl Default for DriveStatusIndicator {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveStatusIndicatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveTargetConfigurationLockLevel {
        V012001(crate::drive::v1_20_1::TargetConfigurationLockLevel),
        V000001(crate::drive::v1_20_1::DriveTargetConfigurationLockLevelN1),
    }
    impl Default for DriveTargetConfigurationLockLevel {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveTargetConfigurationLockLevelN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionAbility {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "SelfEncryptingDrive")]
        SelfEncryptingDrive,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EncryptionStatus {
        #[default]
        #[serde(rename = "Foreign")]
        Foreign,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Unecrypted")]
        Unecrypted,
        #[serde(rename = "Unencrypted")]
        Unencrypted,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FormFactor {
        #[default]
        #[serde(rename = "Drive2_5")]
        Drive2N5,
        #[serde(rename = "Drive3_5")]
        Drive3N5,
        #[serde(rename = "EDSFF")]
        EDSFF,
        #[serde(rename = "EDSFF_1U_Long")]
        EDSFF1ULong,
        #[serde(rename = "EDSFF_1U_Short")]
        EDSFF1UShort,
        #[serde(rename = "EDSFF_E3_Long")]
        EDSFFE3Long,
        #[serde(rename = "EDSFF_E3_Short")]
        EDSFFE3Short,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "M2_22110")]
        M2N22110,
        #[serde(rename = "M2_2230")]
        M2N2230,
        #[serde(rename = "M2_2242")]
        M2N2242,
        #[serde(rename = "M2_2260")]
        M2N2260,
        #[serde(rename = "M2_2280")]
        M2N2280,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PCIeHalfLength")]
        PCIeHalfLength,
        #[serde(rename = "PCIeSlotFullLength")]
        PCIeSlotFullLength,
        #[serde(rename = "PCIeSlotLowProfile")]
        PCIeSlotLowProfile,
        #[serde(rename = "U2")]
        U2,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareReplacementModeType {
        #[default]
        #[serde(rename = "NonRevertible")]
        NonRevertible,
        #[serde(rename = "Revertible")]
        Revertible,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HotspareType {
        #[default]
        #[serde(rename = "Chassis")]
        Chassis,
        #[serde(rename = "Dedicated")]
        Dedicated,
        #[serde(rename = "Global")]
        Global,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StoragePools")]
        pub storage_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoragePools@odata.count"
        )]
        pub storage_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediaType {
        #[default]
        #[serde(rename = "HDD")]
        HDD,
        #[serde(rename = "SMR")]
        SMR,
        #[serde(rename = "SSD")]
        SSD,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMe {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockState"
        )]
        pub configuration_lock_state:
            Option<crate::drive::v1_20_1::NVMeConfigurationLockStateAnony>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeConfigurationLockState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareCommit")]
        pub firmware_commit:
            Option<crate::drive::v1_20_1::NVMeConfigurationLockStateFirmwareCommit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareImageDownload"
        )]
        pub firmware_image_download:
            Option<crate::drive::v1_20_1::NVMeConfigurationLockStateFirmwareImageDownload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lockdown")]
        pub lockdown: Option<crate::drive::v1_20_1::NVMeConfigurationLockStateLockdown>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecuritySend")]
        pub security_send: Option<crate::drive::v1_20_1::NVMeConfigurationLockStateSecuritySend>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VPDWrite")]
        pub vpd_write: Option<crate::drive::v1_20_1::NVMeConfigurationLockStateVPDWrite>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateAnony {
        V012001(crate::drive::v1_20_1::NVMeConfigurationLockState),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateN1),
    }
    impl Default for NVMeConfigurationLockStateAnony {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateFirmwareCommit {
        V012001(crate::drive::v1_20_1::ConfigLockOptions),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateFirmwareCommitN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareCommit {
        fn default() -> Self {
            Self::V012001(Default::default())
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
        V012001(crate::drive::v1_20_1::ConfigLockOptions),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateFirmwareImageDownloadN1),
    }
    impl Default for NVMeConfigurationLockStateFirmwareImageDownload {
        fn default() -> Self {
            Self::V012001(Default::default())
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
        V012001(crate::drive::v1_20_1::ConfigLockOptions),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateLockdownN1),
    }
    impl Default for NVMeConfigurationLockStateLockdown {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateLockdownN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeConfigurationLockStateSecuritySend {
        V012001(crate::drive::v1_20_1::ConfigLockOptions),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateSecuritySendN1),
    }
    impl Default for NVMeConfigurationLockStateSecuritySend {
        fn default() -> Self {
            Self::V012001(Default::default())
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
        V012001(crate::drive::v1_20_1::ConfigLockOptions),
        V000001(crate::drive::v1_20_1::NVMeConfigurationLockStateVPDWriteN1),
    }
    impl Default for NVMeConfigurationLockStateVPDWrite {
        fn default() -> Self {
            Self::V012001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeConfigurationLockStateVPDWriteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Operations {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedTask")]
        pub associated_task: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operation")]
        pub operation: Option<crate::drive::v1_20_1::OperationsOperation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperationsOperation {
        V000001(crate::drive::v1_20_1::OperationsOperationN1),
        VolumeOperationType(crate::swordfish::volume::OperationType),
    }
    impl Default for OperationsOperation {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationsOperationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RevertToOriginalFactoryState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RevertToOriginalFactoryStateRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSecureID")]
        pub physical_secure_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverwritePasses")]
        pub overwrite_passes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SanitizationType")]
        pub sanitization_type: Option<crate::drive::v1_20_1::DataSanitizationType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StatusIndicator {
        #[default]
        #[serde(rename = "Fail")]
        Fail,
        #[serde(rename = "Hotspare")]
        Hotspare,
        #[serde(rename = "InACriticalArray")]
        InACriticalArray,
        #[serde(rename = "InAFailedArray")]
        InAFailedArray,
        #[serde(rename = "OK")]
        OK,
        #[serde(rename = "PredictiveFailureAnalysis")]
        PredictiveFailureAnalysis,
        #[serde(rename = "Rebuild")]
        Rebuild,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TargetConfigurationLockLevel {
        #[default]
        #[serde(rename = "Baseline")]
        Baseline,
    }
}
