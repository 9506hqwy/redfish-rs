use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ElectricalContext {
    #[default]
    #[serde(rename = "Line1")]
    Line1,
    #[serde(rename = "Line1ToLine2")]
    Line1ToLine2,
    #[serde(rename = "Line1ToNeutral")]
    Line1ToNeutral,
    #[serde(rename = "Line1ToNeutralAndL1L2")]
    Line1ToNeutralAndL1L2,
    #[serde(rename = "Line2")]
    Line2,
    #[serde(rename = "Line2ToLine3")]
    Line2ToLine3,
    #[serde(rename = "Line2ToNeutral")]
    Line2ToNeutral,
    #[serde(rename = "Line2ToNeutralAndL1L2")]
    Line2ToNeutralAndL1L2,
    #[serde(rename = "Line2ToNeutralAndL2L3")]
    Line2ToNeutralAndL2L3,
    #[serde(rename = "Line3")]
    Line3,
    #[serde(rename = "Line3ToLine1")]
    Line3ToLine1,
    #[serde(rename = "Line3ToNeutral")]
    Line3ToNeutral,
    #[serde(rename = "Line3ToNeutralAndL3L1")]
    Line3ToNeutralAndL3L1,
    #[serde(rename = "LineToLine")]
    LineToLine,
    #[serde(rename = "LineToNeutral")]
    LineToNeutral,
    #[serde(rename = "Neutral")]
    Neutral,
    #[serde(rename = "Total")]
    Total,
}
pub type Sensor = crate::sensor::v1_8_1::Sensor;
pub type SensorArrayExcerpt = crate::sensor::v1_8_1::SensorArrayExcerpt;
pub type SensorCurrentExcerpt = crate::sensor::v1_8_1::SensorCurrentExcerpt;
pub type SensorEnergykWhExcerpt = crate::sensor::v1_8_1::SensorEnergykWhExcerpt;
pub type SensorExcerpt = crate::sensor::v1_8_1::SensorExcerpt;
pub type SensorFanArrayExcerpt = crate::sensor::v1_8_1::SensorFanArrayExcerpt;
pub type SensorFanExcerpt = crate::sensor::v1_8_1::SensorFanExcerpt;
pub type SensorPowerArrayExcerpt = crate::sensor::v1_8_1::SensorPowerArrayExcerpt;
pub type SensorPowerExcerpt = crate::sensor::v1_8_1::SensorPowerExcerpt;
pub type SensorPumpExcerpt = crate::sensor::v1_8_1::SensorPumpExcerpt;
pub type SensorVoltageExcerpt = crate::sensor::v1_8_1::SensorVoltageExcerpt;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum VoltageType {
    #[default]
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "DC")]
    DC,
}
pub mod v1_7_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_7_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_7_0::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_7_0::ResetToDefaults>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Reported")]
        Reported,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedControls")]
        pub associated_controls: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedControls@odata.count"
        )]
        pub associated_controls_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingBasisType {
        #[default]
        #[serde(rename = "Delta")]
        Delta,
        #[serde(rename = "Headroom")]
        Headroom,
        #[serde(rename = "Zero")]
        Zero,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AbsoluteHumidity")]
        AbsoluteHumidity,
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "AirFlowCMM")]
        AirFlowCMM,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "ChargeAh")]
        ChargeAh,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergyWh")]
        EnergyWh,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Heat")]
        Heat,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidFlowLPM")]
        LiquidFlowLPM,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
        #[serde(rename = "PressurePa")]
        PressurePa,
        #[serde(rename = "PressurekPa")]
        PressurekPa,
        #[serde(rename = "Rotational")]
        Rotational,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Voltage")]
        Voltage,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetricsRequestBody {}
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_7_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentkVAh")]
        pub apparent_kvah: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageReading")]
        pub average_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AveragingInterval")]
        pub averaging_interval: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragingIntervalAchieved"
        )]
        pub averaging_interval_achieved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CalibrationTime")]
        pub calibration_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_7_0::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_7_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReading")]
        pub lowest_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReadingTime")]
        pub lowest_reading_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<f64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReading")]
        pub peak_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReadingTime")]
        pub peak_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactivekVARh")]
        pub reactive_kvarh: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingBasis")]
        pub reading_basis: Option<crate::sensor::v1_7_0::ReadingBasisType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_7_0::ReadingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorGroup")]
        pub sensor_group: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_7_0::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::VoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorCurrentExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorEnergykWhExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentkVAh")]
        pub apparent_kvah: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactivekVARh")]
        pub reactive_kvarh: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorFanArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorFanExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPumpExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorVoltageExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_7_0::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisDuration")]
        pub hysteresis_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisReading")]
        pub hysteresis_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_7_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_7_0::Threshold>,
    }
}
pub mod v1_8_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_8_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_8_1::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_8_1::ResetToDefaults>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Reported")]
        Reported,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedControls")]
        pub associated_controls: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedControls@odata.count"
        )]
        pub associated_controls_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingBasisType {
        #[default]
        #[serde(rename = "Delta")]
        Delta,
        #[serde(rename = "Headroom")]
        Headroom,
        #[serde(rename = "Zero")]
        Zero,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AbsoluteHumidity")]
        AbsoluteHumidity,
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "AirFlowCMM")]
        AirFlowCMM,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "ChargeAh")]
        ChargeAh,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergyWh")]
        EnergyWh,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Heat")]
        Heat,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidFlowLPM")]
        LiquidFlowLPM,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
        #[serde(rename = "PressurePa")]
        PressurePa,
        #[serde(rename = "PressurekPa")]
        PressurekPa,
        #[serde(rename = "Rotational")]
        Rotational,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Voltage")]
        Voltage,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetricsRequestBody {}
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_8_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentkVAh")]
        pub apparent_kvah: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageReading")]
        pub average_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AveragingInterval")]
        pub averaging_interval: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragingIntervalAchieved"
        )]
        pub averaging_interval_achieved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CalibrationTime")]
        pub calibration_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_8_1::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_8_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReading")]
        pub lowest_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReadingTime")]
        pub lowest_reading_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<f64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReading")]
        pub peak_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReadingTime")]
        pub peak_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactivekVARh")]
        pub reactive_kvarh: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingAccuracy")]
        pub reading_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingBasis")]
        pub reading_basis: Option<crate::sensor::v1_8_1::ReadingBasisType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_8_1::ReadingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorGroup")]
        pub sensor_group: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_8_1::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::VoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorCurrentExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorEnergykWhExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentkVAh")]
        pub apparent_kvah: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactivekVARh")]
        pub reactive_kvarh: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorFanArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorFanExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPumpExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorVoltageExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_8_1::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisDuration")]
        pub hysteresis_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisReading")]
        pub hysteresis_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_8_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_8_1::Threshold>,
    }
}
