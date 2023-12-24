use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Control {
    V010300(crate::control::v1_3_0::Control),
    V010200(crate::control::v1_2_0::Control),
    V010100(crate::control::v1_1_0::Control),
    V010001(crate::control::v1_0_1::Control),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ControlExcerpt {
    V010300(crate::control::v1_3_0::ControlExcerpt),
    V010200(crate::control::v1_2_0::ControlExcerpt),
    V010100(crate::control::v1_1_0::ControlExcerpt),
    V010001(crate::control::v1_0_1::ControlExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ControlRangeExcerpt {
    V010300(crate::control::v1_3_0::ControlRangeExcerpt),
    V010200(crate::control::v1_2_0::ControlRangeExcerpt),
    V010100(crate::control::v1_1_0::ControlRangeExcerpt),
    V010001(crate::control::v1_0_1::ControlRangeExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ControlSingleExcerpt {
    V010300(crate::control::v1_3_0::ControlSingleExcerpt),
    V010200(crate::control::v1_2_0::ControlSingleExcerpt),
    V010100(crate::control::v1_1_0::ControlSingleExcerpt),
    V010001(crate::control::v1_0_1::ControlSingleExcerpt),
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::control::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Control {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::control::v1_0_1::Actions>,
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
        pub control_loop: Option<crate::control::v1_0_1::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_0_1::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlType")]
        pub control_type: Option<crate::control::v1_0_1::ControlType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeadBand")]
        pub dead_band: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::control::v1_0_1::ImplementationType>,
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
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensor")]
        pub sensor: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointType")]
        pub set_point_type: Option<crate::control::v1_0_1::SetPointType>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_0_1::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
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
        pub control_mode: Option<crate::control::v1_0_1::ControlMode>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_0_1::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlType {
        #[default]
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Temperature")]
        Temperature,
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
    pub enum SetPointType {
        #[default]
        #[serde(rename = "Range")]
        Range,
        #[serde(rename = "Single")]
        Single,
    }
}
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::control::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Control {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::control::v1_1_0::Actions>,
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
        pub control_loop: Option<crate::control::v1_1_0::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_1_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlType")]
        pub control_type: Option<crate::control::v1_1_0::ControlType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeadBand")]
        pub dead_band: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::control::v1_1_0::ImplementationType>,
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
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensor")]
        pub sensor: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointType")]
        pub set_point_type: Option<crate::control::v1_1_0::SetPointType>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_1_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
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
        pub control_mode: Option<crate::control::v1_1_0::ControlMode>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_1_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlType {
        #[default]
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "FrequencyMHz")]
        FrequencyMHz,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
        #[serde(rename = "Temperature")]
        Temperature,
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
    pub enum SetPointType {
        #[default]
        #[serde(rename = "Range")]
        Range,
        #[serde(rename = "Single")]
        Single,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Control.ResetToDefaults"
        )]
        pub control_reset_to_defaults: Option<crate::control::v1_2_0::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::control::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Control {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::control::v1_2_0::Actions>,
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
        pub control_loop: Option<crate::control::v1_2_0::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_2_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlType")]
        pub control_type: Option<crate::control::v1_2_0::ControlType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeadBand")]
        pub dead_band: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::control::v1_2_0::ImplementationType>,
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
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensor")]
        pub sensor: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointType")]
        pub set_point_type: Option<crate::control::v1_2_0::SetPointType>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_2_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
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
        pub control_mode: Option<crate::control::v1_2_0::ControlMode>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_2_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlType {
        #[default]
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "FrequencyMHz")]
        FrequencyMHz,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
        #[serde(rename = "Temperature")]
        Temperature,
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
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Control.ResetToDefaults"
        )]
        pub control_reset_to_defaults: Option<crate::control::v1_3_0::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::control::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Control {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::control::v1_3_0::Actions>,
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
        pub control_loop: Option<crate::control::v1_3_0::ControlLoop>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_3_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlType")]
        pub control_type: Option<crate::control::v1_3_0::ControlType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeadBand")]
        pub dead_band: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DefaultSetPoint")]
        pub default_set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::control::v1_3_0::ImplementationType>,
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
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensor")]
        pub sensor: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPoint")]
        pub set_point: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SetPointType")]
        pub set_point_type: Option<crate::control::v1_3_0::SetPointType>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_3_0::ControlMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
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
        pub control_mode: Option<crate::control::v1_3_0::ControlMode>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControlSingleExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMax")]
        pub allowable_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableMin")]
        pub allowable_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControlMode")]
        pub control_mode: Option<crate::control::v1_3_0::ControlMode>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ControlType {
        #[default]
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "FrequencyMHz")]
        FrequencyMHz,
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
