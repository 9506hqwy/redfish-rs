pub type Pump = crate::pump::v1_2_0::Pump;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pump::v1_2_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Pump.SetMode")]
        pub pump_set_mode: Option<crate::pump::v1_2_0::SetMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Pump {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pump::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::pump::v1_2_0::PumpDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Filters")]
        pub filters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InletPressurekPa")]
        pub inlet_pressurek_pa: Option<crate::pump::v1_2_0::PumpInletPressurekPa>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProductionDate")]
        pub production_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PumpSpeedPercent")]
        pub pump_speed_percent: Option<crate::pump::v1_2_0::PumpPumpSpeedPercent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PumpType")]
        pub pump_type: Option<crate::pump::v1_2_0::PumpPumpType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceHours")]
        pub service_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpeedControlPercent"
        )]
        pub speed_control_percent: Option<crate::pump::v1_2_0::PumpSpeedControlPercent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PumpDescription {
        V000001(crate::pump::v1_2_0::PumpDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PumpDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PumpInletPressurekPa {
        V000001(crate::pump::v1_2_0::PumpInletPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for PumpInletPressurekPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpInletPressurekPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PumpPumpSpeedPercent {
        V000001(crate::pump::v1_2_0::PumpPumpSpeedPercentN1),
        SensorSensorPumpExcerpt(crate::sensor::v1_9_2::SensorPumpExcerpt),
    }
    impl Default for PumpPumpSpeedPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpPumpSpeedPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PumpPumpType {
        V010200(crate::pump::v1_2_0::PumpType),
        V000001(crate::pump::v1_2_0::PumpPumpTypeN1),
    }
    impl Default for PumpPumpType {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpPumpTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PumpSpeedControlPercent {
        V000001(crate::pump::v1_2_0::PumpSpeedControlPercentN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for PumpSpeedControlPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpSpeedControlPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PumpType {
        #[default]
        #[serde(rename = "Compressor")]
        Compressor,
        #[serde(rename = "Liquid")]
        Liquid,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetMode {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetModeRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mode")]
        pub mode: Option<crate::pump::v1_2_0::PumpMode>,
    }
}
