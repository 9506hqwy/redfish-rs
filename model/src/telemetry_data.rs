use serde::{Deserialize, Serialize};
pub type TelemetryData = crate::telemetry_data::v1_0_0::TelemetryData;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum TelemetryDataTypes {
    #[default]
    #[serde(rename = "OEM")]
    OEM,
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::telemetry_data::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TelemetryData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::telemetry_data::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalData")]
        pub additional_data: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalDataSizeBytes"
        )]
        pub additional_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalDataURI")]
        pub additional_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::telemetry_data::v1_0_0::TelemetryDataDescription>,
        #[serde(rename = "Id")]
        pub id: String,
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
            rename = "OEMTelemetryDataType"
        )]
        pub oem_telemetry_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TelemetryDataType")]
        pub telemetry_data_type:
            Option<crate::telemetry_data::v1_0_0::TelemetryDataTelemetryDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TelemetryDataDescription {
        V000001(crate::telemetry_data::v1_0_0::TelemetryDataDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for TelemetryDataDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TelemetryDataDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TelemetryDataTelemetryDataType {
        V000001(crate::telemetry_data::v1_0_0::TelemetryDataTelemetryDataTypeN1),
        TelemetryDataTelemetryDataTypes(crate::telemetry_data::TelemetryDataTypes),
    }
    impl Default for TelemetryDataTelemetryDataType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TelemetryDataTelemetryDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
