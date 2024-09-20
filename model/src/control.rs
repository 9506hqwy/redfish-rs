pub type Control = crate::control::v1_5_2::Control;
pub type ControlExcerpt = crate::control::v1_5_2::ControlExcerpt;
pub type ControlRangeExcerpt = crate::control::v1_5_2::ControlRangeExcerpt;
pub type ControlSingleExcerpt = crate::control::v1_5_2::ControlSingleExcerpt;
pub type ControlSingleLoopExcerpt = crate::control::v1_5_2::ControlSingleLoopExcerpt;
pub mod v1_5_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Control.ResetToDefaults"
        )]
        pub control_reset_to_defaults: Option<crate::control::v1_5_2::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::control::v1_5_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Control {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::control::v1_5_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowableNumericValues"
        )]
        pub allowable_numeric_values: Option<Vec<f64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedSensors")]
        pub associated_sensors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedSensors@odata.count"
        )]
        pub associated_sensors_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ControlDelaySeconds"
        )]
        pub control_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlLoop")]
        pub control_loop: Option<crate::control::v1_5_2::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_5_2::ControlControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlType")]
        pub control_type: Option<crate::control::v1_5_2::ControlControlType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeadBand")]
        pub dead_band: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DefaultSetPoint")]
        pub default_set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::control::v1_5_2::ControlDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::control::v1_5_2::ControlImplementation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Increment")]
        pub increment: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::control::v1_5_2::ControlPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::control::v1_5_2::ControlPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensor")]
        pub sensor: Option<crate::control::v1_5_2::ControlSensor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointAccuracy")]
        pub set_point_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointType")]
        pub set_point_type: Option<crate::control::v1_5_2::ControlSetPointType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointUnits")]
        pub set_point_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointUpdateTime")]
        pub set_point_update_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SettingMax")]
        pub setting_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SettingMin")]
        pub setting_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlControlMode {
        V010502(crate::control::v1_5_2::ControlMode),
        V000001(crate::control::v1_5_2::ControlControlModeN1),
    }
    impl Default for ControlControlMode {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlControlModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlControlType {
        V010502(crate::control::v1_5_2::ControlType),
        V000001(crate::control::v1_5_2::ControlControlTypeN1),
    }
    impl Default for ControlControlType {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlControlTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlDescription {
        V000001(crate::control::v1_5_2::ControlDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ControlDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_5_2::ControlExcerptControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlExcerptControlMode {
        V010502(crate::control::v1_5_2::ControlMode),
        V000001(crate::control::v1_5_2::ControlExcerptControlModeN1),
    }
    impl Default for ControlExcerptControlMode {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlExcerptControlModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlImplementation {
        V010502(crate::control::v1_5_2::ImplementationType),
        V000001(crate::control::v1_5_2::ControlImplementationN1),
    }
    impl Default for ControlImplementation {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlImplementationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlLoop {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoefficientUpdateTime"
        )]
        pub coefficient_update_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Differential")]
        pub differential: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Integral")]
        pub integral: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Proportional")]
        pub proportional: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Manual")]
        Manual,
        #[serde(rename = "Override")]
        Override,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlPhysicalContext {
        V000001(crate::control::v1_5_2::ControlPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for ControlPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlPhysicalSubContext {
        V000001(crate::control::v1_5_2::ControlPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for ControlPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlRangeExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowableNumericValues"
        )]
        pub allowable_numeric_values: Option<Vec<f64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_5_2::ControlRangeExcerptControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SettingMax")]
        pub setting_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SettingMin")]
        pub setting_min: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlRangeExcerptControlMode {
        V010502(crate::control::v1_5_2::ControlMode),
        V000001(crate::control::v1_5_2::ControlRangeExcerptControlModeN1),
    }
    impl Default for ControlRangeExcerptControlMode {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlRangeExcerptControlModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlSensor {
        V000001(crate::control::v1_5_2::ControlSensorN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for ControlSensor {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlSensorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlSetPointType {
        V010502(crate::control::v1_5_2::SetPointType),
        V000001(crate::control::v1_5_2::ControlSetPointTypeN1),
    }
    impl Default for ControlSetPointType {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlSetPointTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_5_2::ControlSingleExcerptControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DefaultSetPoint")]
        pub default_set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlSingleExcerptControlMode {
        V010502(crate::control::v1_5_2::ControlMode),
        V000001(crate::control::v1_5_2::ControlSingleExcerptControlModeN1),
    }
    impl Default for ControlSingleExcerptControlMode {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlSingleExcerptControlModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleLoopExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlLoop")]
        pub control_loop: Option<crate::control::v1_5_2::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_5_2::ControlSingleLoopExcerptControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ControlSingleLoopExcerptControlMode {
        V010502(crate::control::v1_5_2::ControlMode),
        V000001(crate::control::v1_5_2::ControlSingleLoopExcerptControlModeN1),
    }
    impl Default for ControlSingleLoopExcerptControlMode {
        fn default() -> Self {
            Self::V010502(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlSingleLoopExcerptControlModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlType {
        #[default]
        #[serde(rename = "DutyCycle")]
        DutyCycle,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "FrequencyMHz")]
        FrequencyMHz,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
        #[serde(rename = "PressurekPa")]
        PressurekPa,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Valve")]
        Valve,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "Direct")]
        Direct,
        #[serde(rename = "Monitored")]
        Monitored,
        #[serde(rename = "Programmable")]
        Programmable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SetPointType {
        #[default]
        #[serde(rename = "Range")]
        Range,
        #[serde(rename = "Single")]
        Single,
    }
}
