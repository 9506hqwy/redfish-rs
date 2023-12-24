use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Drive {
    V011700(crate::drive::v1_17_0::Drive),
    V011601(crate::drive::v1_16_1::Drive),
    V011501(crate::drive::v1_15_1::Drive),
    V011401(crate::drive::v1_14_1::Drive),
    V011302(crate::drive::v1_13_2::Drive),
    V011204(crate::drive::v1_12_4::Drive),
    V011105(crate::drive::v1_11_5::Drive),
    V011005(crate::drive::v1_10_5::Drive),
    V010907(crate::drive::v1_9_7::Drive),
    V010807(crate::drive::v1_8_7::Drive),
    V010707(crate::drive::v1_7_7::Drive),
    V010608(crate::drive::v1_6_8::Drive),
    V010510(crate::drive::v1_5_10::Drive),
    V010411(crate::drive::v1_4_11::Drive),
    V010311(crate::drive::v1_3_11::Drive),
    V010212(crate::drive::v1_2_12::Drive),
    V010114(crate::drive::v1_1_14::Drive),
    V010015(crate::drive::v1_0_15::Drive),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_15 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_0_15::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_0_15::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_0_15::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapableSpeedGbs")]
        pub capable_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_0_15::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_0_15::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_0_15::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_0_15::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_0_15::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_0_15::StatusIndicator>,
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
        #[serde(rename = "Unlocked")]
        Unlocked,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_1_14 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_1_14::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_1_14::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_1_14::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapableSpeedGbs")]
        pub capable_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_1_14::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_1_14::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_1_14::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_1_14::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_1_14::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_1_14::Operations>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_1_14::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_2_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_2_12::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_2_12::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_2_12::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapableSpeedGbs")]
        pub capable_speed_gbs: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityBytes")]
        pub capacity_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_2_12::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_2_12::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_2_12::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_2_12::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_2_12::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_2_12::Operations>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_2_12::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_3_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_3_11::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_3_11::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_3_11::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_3_11::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_3_11::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_3_11::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_3_11::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_3_11::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_3_11::Operations>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_3_11::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_4_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_4_11::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_4_11::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_4_11::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_4_11::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_4_11::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_4_11::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_4_11::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_4_11::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_4_11::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_4_11::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_5_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_5_10::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_5_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_5_10::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_5_10::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_5_10::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_5_10::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_5_10::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_5_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_5_10::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_5_10::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_5_10::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_6_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_6_8::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_6_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_6_8::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_6_8::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_6_8::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_6_8::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_6_8::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_6_8::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_6_8::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_6_8::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_6_8::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperationName")]
        pub operation_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageComplete")]
        pub percentage_complete: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureErase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseRequestBody {}
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
pub mod v1_7_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_7_7::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_7_7::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_7_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_7_7::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_7_7::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_7_7::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_7_7::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_7_7::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_7_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_7_7::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_7_7::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_7_7::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_8_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_8_7::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_8_7::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_8_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_8_7::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_8_7::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_8_7::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_8_7::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_8_7::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_8_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_8_7::MediaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        pub operations: Option<Vec<crate::drive::v1_8_7::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_8_7::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_9_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_9_7::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_9_7::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_9_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_9_7::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_9_7::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_9_7::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_9_7::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_9_7::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_9_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_9_7::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_9_7::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Revision")]
        pub revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RotationSpeedRPM")]
        pub rotation_speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_9_7::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_10_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_10_5::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_10_5::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_10_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_10_5::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_10_5::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_10_5::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_10_5::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_10_5::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_10_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_10_5::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_10_5::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_10_5::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_11_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_11_5::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_11_5::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_11_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_11_5::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_11_5::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_11_5::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_11_5::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_11_5::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_11_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<Vec<crate::resource::Location>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaType")]
        pub media_type: Option<crate::drive::v1_11_5::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_11_5::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_11_5::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_12_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_12_4::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_12_4::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_12_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_12_4::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_12_4::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_12_4::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_12_4::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_12_4::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_12_4::Links>,
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
        pub media_type: Option<crate::drive::v1_12_4::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_12_4::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_12_4::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_13_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_13_2::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_13_2::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_13_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_13_2::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_13_2::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_13_2::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_13_2::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_13_2::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_13_2::Links>,
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
        pub media_type: Option<crate::drive::v1_13_2::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_13_2::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_13_2::StatusIndicator>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_14_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_14_1::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_14_1::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_14_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Drive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive::v1_14_1::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_14_1::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_14_1::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_14_1::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_14_1::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_14_1::Links>,
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
        pub media_type: Option<crate::drive::v1_14_1::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_14_1::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_14_1::StatusIndicator>,
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
    pub struct SecureEraseRequestBody {}
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
pub mod v1_15_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_15_1::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_15_1::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_15_1::OemActions>,
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
        pub actions: Option<crate::drive::v1_15_1::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_15_1::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_15_1::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_15_1::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_15_1::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_15_1::Links>,
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
        pub media_type: Option<crate::drive::v1_15_1::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_15_1::Operations>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_15_1::StatusIndicator>,
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
        pub sanitization_type: Option<crate::drive::v1_15_1::DataSanitizationType>,
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
pub mod v1_16_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.Reset")]
        pub drive_reset: Option<crate::drive::v1_16_1::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Drive.SecureErase")]
        pub drive_secure_erase: Option<crate::drive::v1_16_1::SecureErase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive::v1_16_1::OemActions>,
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
        pub actions: Option<crate::drive::v1_16_1::Actions>,
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
        pub drive_form_factor: Option<crate::drive::v1_16_1::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionAbility")]
        pub encryption_ability: Option<crate::drive::v1_16_1::EncryptionAbility>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionStatus")]
        pub encryption_status: Option<crate::drive::v1_16_1::EncryptionStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FailurePredicted")]
        pub failure_predicted: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotspareReplacementMode"
        )]
        pub hotspare_replacement_mode: Option<crate::drive::v1_16_1::HotspareReplacementModeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotspareType")]
        pub hotspare_type: Option<crate::drive::v1_16_1::HotspareType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::drive::v1_16_1::Links>,
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
        pub media_type: Option<crate::drive::v1_16_1::MediaType>,
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
        pub operations: Option<Vec<crate::drive::v1_16_1::Operations>>,
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
        pub slot_form_factor: Option<crate::drive::v1_16_1::FormFactor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatusIndicator")]
        pub status_indicator: Option<crate::drive::v1_16_1::StatusIndicator>,
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
        pub sanitization_type: Option<crate::drive::v1_16_1::DataSanitizationType>,
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
