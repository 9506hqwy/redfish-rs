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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Sensor {
    SensorV1N0N9Sensor(crate::sensor::v1_0_9::Sensor),
    SensorV1N1N5Sensor(crate::sensor::v1_1_5::Sensor),
    SensorV1N2N3Sensor(crate::sensor::v1_2_3::Sensor),
    SensorV1N3N3Sensor(crate::sensor::v1_3_3::Sensor),
    SensorV1N4N2Sensor(crate::sensor::v1_4_2::Sensor),
    SensorV1N5N1Sensor(crate::sensor::v1_5_1::Sensor),
    SensorV1N6N1Sensor(crate::sensor::v1_6_1::Sensor),
    SensorV1N7N0Sensor(crate::sensor::v1_7_0::Sensor),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorArrayExcerpt {
    SensorV1N0N9SensorArrayExcerpt(crate::sensor::v1_0_9::SensorArrayExcerpt),
    SensorV1N1N5SensorArrayExcerpt(crate::sensor::v1_1_5::SensorArrayExcerpt),
    SensorV1N2N3SensorArrayExcerpt(crate::sensor::v1_2_3::SensorArrayExcerpt),
    SensorV1N3N3SensorArrayExcerpt(crate::sensor::v1_3_3::SensorArrayExcerpt),
    SensorV1N4N2SensorArrayExcerpt(crate::sensor::v1_4_2::SensorArrayExcerpt),
    SensorV1N5N1SensorArrayExcerpt(crate::sensor::v1_5_1::SensorArrayExcerpt),
    SensorV1N6N1SensorArrayExcerpt(crate::sensor::v1_6_1::SensorArrayExcerpt),
    SensorV1N7N0SensorArrayExcerpt(crate::sensor::v1_7_0::SensorArrayExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorCurrentExcerpt {
    SensorV1N1N5SensorCurrentExcerpt(crate::sensor::v1_1_5::SensorCurrentExcerpt),
    SensorV1N2N3SensorCurrentExcerpt(crate::sensor::v1_2_3::SensorCurrentExcerpt),
    SensorV1N3N3SensorCurrentExcerpt(crate::sensor::v1_3_3::SensorCurrentExcerpt),
    SensorV1N4N2SensorCurrentExcerpt(crate::sensor::v1_4_2::SensorCurrentExcerpt),
    SensorV1N5N1SensorCurrentExcerpt(crate::sensor::v1_5_1::SensorCurrentExcerpt),
    SensorV1N6N1SensorCurrentExcerpt(crate::sensor::v1_6_1::SensorCurrentExcerpt),
    SensorV1N7N0SensorCurrentExcerpt(crate::sensor::v1_7_0::SensorCurrentExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorEnergykWhExcerpt {
    SensorV1N0N9SensorEnergykWhExcerpt(crate::sensor::v1_0_9::SensorEnergykWhExcerpt),
    SensorV1N1N5SensorEnergykWhExcerpt(crate::sensor::v1_1_5::SensorEnergykWhExcerpt),
    SensorV1N2N3SensorEnergykWhExcerpt(crate::sensor::v1_2_3::SensorEnergykWhExcerpt),
    SensorV1N3N3SensorEnergykWhExcerpt(crate::sensor::v1_3_3::SensorEnergykWhExcerpt),
    SensorV1N4N2SensorEnergykWhExcerpt(crate::sensor::v1_4_2::SensorEnergykWhExcerpt),
    SensorV1N5N1SensorEnergykWhExcerpt(crate::sensor::v1_5_1::SensorEnergykWhExcerpt),
    SensorV1N6N1SensorEnergykWhExcerpt(crate::sensor::v1_6_1::SensorEnergykWhExcerpt),
    SensorV1N7N0SensorEnergykWhExcerpt(crate::sensor::v1_7_0::SensorEnergykWhExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorExcerpt {
    SensorV1N0N9SensorExcerpt(crate::sensor::v1_0_9::SensorExcerpt),
    SensorV1N1N5SensorExcerpt(crate::sensor::v1_1_5::SensorExcerpt),
    SensorV1N2N3SensorExcerpt(crate::sensor::v1_2_3::SensorExcerpt),
    SensorV1N3N3SensorExcerpt(crate::sensor::v1_3_3::SensorExcerpt),
    SensorV1N4N2SensorExcerpt(crate::sensor::v1_4_2::SensorExcerpt),
    SensorV1N5N1SensorExcerpt(crate::sensor::v1_5_1::SensorExcerpt),
    SensorV1N6N1SensorExcerpt(crate::sensor::v1_6_1::SensorExcerpt),
    SensorV1N7N0SensorExcerpt(crate::sensor::v1_7_0::SensorExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorFanArrayExcerpt {
    SensorV1N0N9SensorFanArrayExcerpt(crate::sensor::v1_0_9::SensorFanArrayExcerpt),
    SensorV1N1N5SensorFanArrayExcerpt(crate::sensor::v1_1_5::SensorFanArrayExcerpt),
    SensorV1N2N3SensorFanArrayExcerpt(crate::sensor::v1_2_3::SensorFanArrayExcerpt),
    SensorV1N3N3SensorFanArrayExcerpt(crate::sensor::v1_3_3::SensorFanArrayExcerpt),
    SensorV1N4N2SensorFanArrayExcerpt(crate::sensor::v1_4_2::SensorFanArrayExcerpt),
    SensorV1N5N1SensorFanArrayExcerpt(crate::sensor::v1_5_1::SensorFanArrayExcerpt),
    SensorV1N6N1SensorFanArrayExcerpt(crate::sensor::v1_6_1::SensorFanArrayExcerpt),
    SensorV1N7N0SensorFanArrayExcerpt(crate::sensor::v1_7_0::SensorFanArrayExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorFanExcerpt {
    SensorV1N2N3SensorFanExcerpt(crate::sensor::v1_2_3::SensorFanExcerpt),
    SensorV1N3N3SensorFanExcerpt(crate::sensor::v1_3_3::SensorFanExcerpt),
    SensorV1N4N2SensorFanExcerpt(crate::sensor::v1_4_2::SensorFanExcerpt),
    SensorV1N5N1SensorFanExcerpt(crate::sensor::v1_5_1::SensorFanExcerpt),
    SensorV1N6N1SensorFanExcerpt(crate::sensor::v1_6_1::SensorFanExcerpt),
    SensorV1N7N0SensorFanExcerpt(crate::sensor::v1_7_0::SensorFanExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorPowerArrayExcerpt {
    SensorV1N0N9SensorPowerArrayExcerpt(crate::sensor::v1_0_9::SensorPowerArrayExcerpt),
    SensorV1N1N5SensorPowerArrayExcerpt(crate::sensor::v1_1_5::SensorPowerArrayExcerpt),
    SensorV1N2N3SensorPowerArrayExcerpt(crate::sensor::v1_2_3::SensorPowerArrayExcerpt),
    SensorV1N3N3SensorPowerArrayExcerpt(crate::sensor::v1_3_3::SensorPowerArrayExcerpt),
    SensorV1N4N2SensorPowerArrayExcerpt(crate::sensor::v1_4_2::SensorPowerArrayExcerpt),
    SensorV1N5N1SensorPowerArrayExcerpt(crate::sensor::v1_5_1::SensorPowerArrayExcerpt),
    SensorV1N6N1SensorPowerArrayExcerpt(crate::sensor::v1_6_1::SensorPowerArrayExcerpt),
    SensorV1N7N0SensorPowerArrayExcerpt(crate::sensor::v1_7_0::SensorPowerArrayExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorPowerExcerpt {
    SensorV1N0N9SensorPowerExcerpt(crate::sensor::v1_0_9::SensorPowerExcerpt),
    SensorV1N1N5SensorPowerExcerpt(crate::sensor::v1_1_5::SensorPowerExcerpt),
    SensorV1N2N3SensorPowerExcerpt(crate::sensor::v1_2_3::SensorPowerExcerpt),
    SensorV1N3N3SensorPowerExcerpt(crate::sensor::v1_3_3::SensorPowerExcerpt),
    SensorV1N4N2SensorPowerExcerpt(crate::sensor::v1_4_2::SensorPowerExcerpt),
    SensorV1N5N1SensorPowerExcerpt(crate::sensor::v1_5_1::SensorPowerExcerpt),
    SensorV1N6N1SensorPowerExcerpt(crate::sensor::v1_6_1::SensorPowerExcerpt),
    SensorV1N7N0SensorPowerExcerpt(crate::sensor::v1_7_0::SensorPowerExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorPumpExcerpt {
    SensorV1N2N3SensorPumpExcerpt(crate::sensor::v1_2_3::SensorPumpExcerpt),
    SensorV1N3N3SensorPumpExcerpt(crate::sensor::v1_3_3::SensorPumpExcerpt),
    SensorV1N4N2SensorPumpExcerpt(crate::sensor::v1_4_2::SensorPumpExcerpt),
    SensorV1N5N1SensorPumpExcerpt(crate::sensor::v1_5_1::SensorPumpExcerpt),
    SensorV1N6N1SensorPumpExcerpt(crate::sensor::v1_6_1::SensorPumpExcerpt),
    SensorV1N7N0SensorPumpExcerpt(crate::sensor::v1_7_0::SensorPumpExcerpt),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensorVoltageExcerpt {
    SensorV1N1N5SensorVoltageExcerpt(crate::sensor::v1_1_5::SensorVoltageExcerpt),
    SensorV1N2N3SensorVoltageExcerpt(crate::sensor::v1_2_3::SensorVoltageExcerpt),
    SensorV1N3N3SensorVoltageExcerpt(crate::sensor::v1_3_3::SensorVoltageExcerpt),
    SensorV1N4N2SensorVoltageExcerpt(crate::sensor::v1_4_2::SensorVoltageExcerpt),
    SensorV1N5N1SensorVoltageExcerpt(crate::sensor::v1_5_1::SensorVoltageExcerpt),
    SensorV1N6N1SensorVoltageExcerpt(crate::sensor::v1_6_1::SensorVoltageExcerpt),
    SensorV1N7N0SensorVoltageExcerpt(crate::sensor::v1_7_0::SensorVoltageExcerpt),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum VoltageType {
    #[default]
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "DC")]
    DC,
}
pub mod v1_0_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_0_9::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_0_9::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_0_9::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_0_9::ReadingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_0_9::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::VoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorEnergykWhExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::sensor::v1_0_9::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_0_9::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_0_9::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_0_9::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_0_9::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_0_9::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_0_9::Threshold>,
    }
}
pub mod v1_1_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_1_5::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_1_5::ResetMetrics>,
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
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_1_5::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_1_5::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_1_5::ReadingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_1_5::Thresholds>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::sensor::VoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SensorPowerArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
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
        pub activation: Option<crate::sensor::v1_1_5::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_1_5::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_1_5::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_1_5::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_1_5::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_1_5::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_1_5::Threshold>,
    }
}
pub mod v1_2_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_2_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_2_3::ResetMetrics>,
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
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_2_3::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_2_3::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_2_3::ReadingType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_2_3::Thresholds>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
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
        pub activation: Option<crate::sensor::v1_2_3::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_2_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_2_3::Threshold>,
    }
}
pub mod v1_3_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_3_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_3_3::ResetMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AirFlow")]
        AirFlow,
        #[serde(rename = "Altitude")]
        Altitude,
        #[serde(rename = "Barometric")]
        Barometric,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "EnergyJoules")]
        EnergyJoules,
        #[serde(rename = "EnergykWh")]
        EnergykWh,
        #[serde(rename = "Frequency")]
        Frequency,
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_3_3::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApparentVA")]
        pub apparent_va: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CrestFactor")]
        pub crest_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::sensor::ElectricalContext>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::sensor::v1_3_3::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_3_3::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_3_3::ReadingType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorResetTime")]
        pub sensor_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedRPM")]
        pub speed_rpm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "THDPercent")]
        pub thd_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thresholds")]
        pub thresholds: Option<crate::sensor::v1_3_3::Thresholds>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
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
        pub activation: Option<crate::sensor::v1_3_3::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_3_3::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_3_3::Threshold>,
    }
}
pub mod v1_4_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_4_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_4_2::ResetMetrics>,
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
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AirFlow")]
        AirFlow,
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
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_4_2::Actions>,
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
        pub implementation: Option<crate::sensor::v1_4_2::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_4_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context: Option<crate::physical_context::PhysicalSubContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerFactor")]
        pub power_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReactiveVAR")]
        pub reactive_var: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_4_2::ReadingType>,
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
        pub thresholds: Option<crate::sensor::v1_4_2::Thresholds>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
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
        pub activation: Option<crate::sensor::v1_4_2::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_4_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_4_2::Threshold>,
    }
}
pub mod v1_5_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_5_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_5_1::ResetMetrics>,
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
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AbsoluteHumidity")]
        AbsoluteHumidity,
        #[serde(rename = "AirFlow")]
        AirFlow,
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
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
    pub struct Sensor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::sensor::v1_5_1::Actions>,
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
        pub implementation: Option<crate::sensor::v1_5_1::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_5_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_5_1::ReadingType>,
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
        pub thresholds: Option<crate::sensor::v1_5_1::Thresholds>,
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
        pub activation: Option<crate::sensor::v1_5_1::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_5_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_5_1::Threshold>,
    }
}
pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::sensor::v1_6_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetMetrics"
        )]
        pub sensor_reset_metrics: Option<crate::sensor::v1_6_1::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Sensor.ResetToDefaults"
        )]
        pub sensor_reset_to_defaults: Option<crate::sensor::v1_6_1::ResetToDefaults>,
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
    pub enum ReadingType {
        #[default]
        #[serde(rename = "AbsoluteHumidity")]
        AbsoluteHumidity,
        #[serde(rename = "AirFlow")]
        AirFlow,
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
        #[serde(rename = "Humidity")]
        Humidity,
        #[serde(rename = "LiquidFlow")]
        LiquidFlow,
        #[serde(rename = "LiquidLevel")]
        LiquidLevel,
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Pressure")]
        Pressure,
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
        pub actions: Option<crate::sensor::v1_6_1::Actions>,
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
        pub implementation: Option<crate::sensor::v1_6_1::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifetimeReading")]
        pub lifetime_reading: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::sensor::v1_6_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMax")]
        pub reading_range_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingRangeMin")]
        pub reading_range_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingTime")]
        pub reading_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingType")]
        pub reading_type: Option<crate::sensor::v1_6_1::ReadingType>,
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
        pub thresholds: Option<crate::sensor::v1_6_1::Thresholds>,
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
        pub activation: Option<crate::sensor::v1_6_1::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCaution")]
        pub lower_caution: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCautionUser")]
        pub lower_caution_user: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCriticalUser")]
        pub lower_critical_user: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerFatal")]
        pub lower_fatal: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCaution")]
        pub upper_caution: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCautionUser")]
        pub upper_caution_user: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCriticalUser")]
        pub upper_critical_user: Option<crate::sensor::v1_6_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperFatal")]
        pub upper_fatal: Option<crate::sensor::v1_6_1::Threshold>,
    }
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
