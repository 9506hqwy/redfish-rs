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
pub type Sensor = crate::sensor::v1_11_0::Sensor;
pub type SensorArrayExcerpt = crate::sensor::v1_9_2::SensorArrayExcerpt;
pub type SensorCurrentExcerpt = crate::sensor::v1_9_2::SensorCurrentExcerpt;
pub type SensorEnergykWhExcerpt = crate::sensor::v1_9_2::SensorEnergykWhExcerpt;
pub type SensorExcerpt = crate::sensor::v1_9_2::SensorExcerpt;
pub type SensorFanArrayExcerpt = crate::sensor::v1_9_2::SensorFanArrayExcerpt;
pub type SensorFanExcerpt = crate::sensor::v1_9_2::SensorFanExcerpt;
pub type SensorPowerArrayExcerpt = crate::sensor::v1_9_2::SensorPowerArrayExcerpt;
pub type SensorPowerExcerpt = crate::sensor::v1_9_2::SensorPowerExcerpt;
pub type SensorPumpExcerpt = crate::sensor::v1_9_2::SensorPumpExcerpt;
pub type SensorVoltageExcerpt = crate::sensor::v1_9_2::SensorVoltageExcerpt;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum VoltageType {
    #[default]
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "DC")]
    DC,
}
pub mod v1_9_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_9_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_9_2::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_9_2::ResetToDefaults>,
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
        pub actions: Option<crate::sensor::v1_9_2::Actions>,
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
        pub description: Option<crate::sensor::v1_9_2::SensorDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::v1_9_2::SensorElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_9_2::SensorImplementation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LifetimeStartDateTime"
        )]
        pub lifetime_start_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_9_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReading")]
        pub lowest_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReadingTime")]
        pub lowest_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReading")]
        pub peak_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReadingTime")]
        pub peak_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_9_2::SensorPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::sensor::v1_9_2::SensorPhysicalSubContext>,
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
        pub reading_basis: Option<crate::sensor::v1_9_2::SensorReadingBasis>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_9_2::SensorReadingType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_9_2::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::v1_9_2::SensorVoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_9_2::SensorArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_9_2::SensorArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_9_2::SensorArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_9_2::SensorArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorDescription {
        V000001(crate::sensor::v1_9_2::SensorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SensorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorElectricalContext {
        V000001(crate::sensor::v1_9_2::SensorElectricalContextN1),
        SensorElectricalContext(crate::sensor::ElectricalContext),
    }
    impl Default for SensorElectricalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorElectricalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context: Option<crate::sensor::v1_9_2::SensorFanArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_9_2::SensorFanArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_9_2::SensorFanArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_9_2::SensorFanArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorImplementation {
        V010902(crate::sensor::v1_9_2::ImplementationType),
        V000001(crate::sensor::v1_9_2::SensorImplementationN1),
    }
    impl Default for SensorImplementation {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorImplementationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalContext {
        V000001(crate::sensor::v1_9_2::SensorPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalSubContext {
        V000001(crate::sensor::v1_9_2::SensorPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context: Option<crate::sensor::v1_9_2::SensorPowerArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_9_2::SensorPowerArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_9_2::SensorPowerArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_9_2::SensorPowerArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingBasis {
        V010902(crate::sensor::v1_9_2::ReadingBasisType),
        V000001(crate::sensor::v1_9_2::SensorReadingBasisN1),
    }
    impl Default for SensorReadingBasis {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingBasisN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingType {
        V010902(crate::sensor::v1_9_2::ReadingType),
        V000001(crate::sensor::v1_9_2::SensorReadingTypeN1),
    }
    impl Default for SensorReadingType {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorVoltageType {
        V000001(crate::sensor::v1_9_2::SensorVoltageTypeN1),
        SensorVoltageType(crate::sensor::VoltageType),
    }
    impl Default for SensorVoltageType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorVoltageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_9_2::ThresholdActivationAnony>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThresholdActivationAnony {
        V010902(crate::sensor::v1_9_2::ThresholdActivation),
        V000001(crate::sensor::v1_9_2::ThresholdActivationN1),
    }
    impl Default for ThresholdActivationAnony {
        fn default() -> Self {
            Self::V010902(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_9_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_9_2::Threshold>,
    }
}
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_10_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_10_0::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_10_0::ResetToDefaults>,
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
        pub actions: Option<crate::sensor::v1_10_0::Actions>,
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
        pub description: Option<crate::sensor::v1_10_0::SensorDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::v1_10_0::SensorElectricalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_10_0::SensorImplementation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LifetimeStartDateTime"
        )]
        pub lifetime_start_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_10_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReading")]
        pub lowest_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReadingTime")]
        pub lowest_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReading")]
        pub peak_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReadingTime")]
        pub peak_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_10_0::SensorPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::sensor::v1_10_0::SensorPhysicalSubContext>,
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
        pub reading_basis: Option<crate::sensor::v1_10_0::SensorReadingBasis>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_10_0::SensorReadingType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_10_0::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::v1_10_0::SensorVoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_10_0::SensorArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_10_0::SensorArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_10_0::SensorArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_10_0::SensorArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorDescription {
        V000001(crate::sensor::v1_10_0::SensorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SensorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorElectricalContext {
        V000001(crate::sensor::v1_10_0::SensorElectricalContextN1),
        SensorElectricalContext(crate::sensor::ElectricalContext),
    }
    impl Default for SensorElectricalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorElectricalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context: Option<crate::sensor::v1_10_0::SensorFanArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_10_0::SensorFanArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_10_0::SensorFanArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_10_0::SensorFanArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorImplementation {
        V011000(crate::sensor::v1_10_0::ImplementationType),
        V000001(crate::sensor::v1_10_0::SensorImplementationN1),
    }
    impl Default for SensorImplementation {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorImplementationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalContext {
        V000001(crate::sensor::v1_10_0::SensorPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalSubContext {
        V000001(crate::sensor::v1_10_0::SensorPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context:
            Option<crate::sensor::v1_10_0::SensorPowerArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_10_0::SensorPowerArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_10_0::SensorPowerArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_10_0::SensorPowerArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingBasis {
        V011000(crate::sensor::v1_10_0::ReadingBasisType),
        V000001(crate::sensor::v1_10_0::SensorReadingBasisN1),
    }
    impl Default for SensorReadingBasis {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingBasisN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingType {
        V011000(crate::sensor::v1_10_0::ReadingType),
        V000001(crate::sensor::v1_10_0::SensorReadingTypeN1),
    }
    impl Default for SensorReadingType {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorVoltageType {
        V000001(crate::sensor::v1_10_0::SensorVoltageTypeN1),
        SensorVoltageType(crate::sensor::VoltageType),
    }
    impl Default for SensorVoltageType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorVoltageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_10_0::ThresholdActivationAnony>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThresholdActivationAnony {
        V011000(crate::sensor::v1_10_0::ThresholdActivation),
        V000001(crate::sensor::v1_10_0::ThresholdActivationN1),
    }
    impl Default for ThresholdActivationAnony {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_10_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_10_0::Threshold>,
    }
}
pub mod v1_11_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_11_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_11_0::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_11_0::ResetToDefaults>,
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
        #[serde(rename = "LinearAcceleration")]
        LinearAcceleration,
        #[serde(rename = "LinearPosition")]
        LinearPosition,
        #[serde(rename = "LinearVelocity")]
        LinearVelocity,
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
        #[serde(rename = "RotationalAcceleration")]
        RotationalAcceleration,
        #[serde(rename = "RotationalPosition")]
        RotationalPosition,
        #[serde(rename = "RotationalVelocity")]
        RotationalVelocity,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Valve")]
        Valve,
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
        pub actions: Option<crate::sensor::v1_11_0::Actions>,
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
        pub description: Option<crate::sensor::v1_11_0::SensorDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::v1_11_0::SensorElectricalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_11_0::SensorImplementation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LifetimeStartDateTime"
        )]
        pub lifetime_start_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_11_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReading")]
        pub lowest_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestReadingTime")]
        pub lowest_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReading")]
        pub peak_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeakReadingTime")]
        pub peak_reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseAngleDegrees")]
        pub phase_angle_degrees: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_11_0::SensorPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::sensor::v1_11_0::SensorPhysicalSubContext>,
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
        pub reading_basis: Option<crate::sensor::v1_11_0::SensorReadingBasis>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_11_0::SensorReadingType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_11_0::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::v1_11_0::SensorVoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::sensor::v1_11_0::SensorArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_11_0::SensorArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_11_0::SensorArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_11_0::SensorArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorDescription {
        V000001(crate::sensor::v1_11_0::SensorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SensorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorElectricalContext {
        V000001(crate::sensor::v1_11_0::SensorElectricalContextN1),
        SensorElectricalContext(crate::sensor::ElectricalContext),
    }
    impl Default for SensorElectricalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorElectricalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context: Option<crate::sensor::v1_11_0::SensorFanArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_11_0::SensorFanArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_11_0::SensorFanArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorFanArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_11_0::SensorFanArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorFanArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorFanArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorImplementation {
        V011100(crate::sensor::v1_11_0::ImplementationType),
        V000001(crate::sensor::v1_11_0::SensorImplementationN1),
    }
    impl Default for SensorImplementation {
        fn default() -> Self {
            Self::V011100(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorImplementationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalContext {
        V000001(crate::sensor::v1_11_0::SensorPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPhysicalSubContext {
        V000001(crate::sensor::v1_11_0::SensorPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub physical_context:
            Option<crate::sensor::v1_11_0::SensorPowerArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::sensor::v1_11_0::SensorPowerArrayExcerptPhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalContext {
        V000001(crate::sensor::v1_11_0::SensorPowerArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorPowerArrayExcerptPhysicalSubContext {
        V000001(crate::sensor::v1_11_0::SensorPowerArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for SensorPowerArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorPowerArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingBasis {
        V011100(crate::sensor::v1_11_0::ReadingBasisType),
        V000001(crate::sensor::v1_11_0::SensorReadingBasisN1),
    }
    impl Default for SensorReadingBasis {
        fn default() -> Self {
            Self::V011100(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingBasisN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorReadingType {
        V011100(crate::sensor::v1_11_0::ReadingType),
        V000001(crate::sensor::v1_11_0::SensorReadingTypeN1),
    }
    impl Default for SensorReadingType {
        fn default() -> Self {
            Self::V011100(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorReadingTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SensorVoltageType {
        V000001(crate::sensor::v1_11_0::SensorVoltageTypeN1),
        SensorVoltageType(crate::sensor::VoltageType),
    }
    impl Default for SensorVoltageType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorVoltageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_11_0::ThresholdActivationAnony>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThresholdActivationAnony {
        V011100(crate::sensor::v1_11_0::ThresholdActivation),
        V000001(crate::sensor::v1_11_0::ThresholdActivationN1),
    }
    impl Default for ThresholdActivationAnony {
        fn default() -> Self {
            Self::V011100(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_11_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_11_0::Threshold>,
    }
}
