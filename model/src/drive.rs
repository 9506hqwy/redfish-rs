pub type Drive = crate::drive::v1_17_1::Drive;
pub mod v1_17_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_17_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_17_0::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_17_0::OemActions>,
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
        pub actions: Option<crate::drive::v1_17_0::Actions>,
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
        pub drive_form_factor: Option<crate::drive::v1_17_0::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_17_0::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_17_0::EncryptionStatus>,
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
        pub hotspare_replacement_mode: Option<crate::drive::v1_17_0::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_17_0::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_17_0::Links>,
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
        pub media_type: Option<crate::drive::v1_17_0::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_17_0::Operations>>,
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
        pub slot_form_factor: Option<crate::drive::v1_17_0::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_17_0::StatusIndicator>,
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
        #[serde(rename = "EDSFF_1U_Long")]
        EDSFF1ULong,
        #[serde(rename = "EDSFF_1U_Short")]
        EDSFF1UShort,
        #[serde(rename = "EDSFF_E3_Long")]
        EDSFFE3Long,
        #[serde(rename = "EDSFF_E3_Short")]
        EDSFFE3Short,
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
        pub sanitization_type: Option<crate::drive::v1_17_0::DataSanitizationType>,
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
pub mod v1_17_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_17_1::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_17_1::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_17_1::OemActions>,
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
        pub actions: Option<crate::drive::v1_17_1::Actions>,
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
        pub drive_form_factor: Option<crate::drive::v1_17_1::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_17_1::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_17_1::EncryptionStatus>,
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
        pub hotspare_replacement_mode: Option<crate::drive::v1_17_1::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_17_1::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_17_1::Links>,
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
        pub media_type: Option<crate::drive::v1_17_1::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_17_1::Operations>>,
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
        pub slot_form_factor: Option<crate::drive::v1_17_1::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_17_1::StatusIndicator>,
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
        #[serde(rename = "EDSFF_1U_Long")]
        EDSFF1ULong,
        #[serde(rename = "EDSFF_1U_Short")]
        EDSFF1UShort,
        #[serde(rename = "EDSFF_E3_Long")]
        EDSFFE3Long,
        #[serde(rename = "EDSFF_E3_Short")]
        EDSFFE3Short,
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
        pub sanitization_type: Option<crate::drive::v1_17_1::DataSanitizationType>,
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
