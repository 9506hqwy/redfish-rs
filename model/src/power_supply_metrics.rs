pub type PowerSupplyMetrics = crate::power_supply_metrics::v1_2_0::PowerSupplyMetrics;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_supply_metrics::v1_2_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#PowerSupplyMetrics.ResetMetrics"
        )]
        pub power_supply_metrics_reset_metrics:
            Option<crate::power_supply_metrics::v1_2_0::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1")]
        pub line1: Option<crate::power_supply_metrics::v1_2_0::CurrentSensorsLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2")]
        pub line2: Option<crate::power_supply_metrics::v1_2_0::CurrentSensorsLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3")]
        pub line3: Option<crate::power_supply_metrics::v1_2_0::CurrentSensorsLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neutral")]
        pub neutral: Option<crate::power_supply_metrics::v1_2_0::CurrentSensorsNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine1 {
        V000001(crate::power_supply_metrics::v1_2_0::CurrentSensorsLine1N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine2 {
        V000001(crate::power_supply_metrics::v1_2_0::CurrentSensorsLine2N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine3 {
        V000001(crate::power_supply_metrics::v1_2_0::CurrentSensorsLine3N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::CurrentSensorsNeutralN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnergySensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::EnergySensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine1ToLine2 {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine1ToLine2N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine1ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine1ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine2ToLine3 {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine2ToLine3N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine2ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine2ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine3ToLine1 {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine3ToLine1N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine3ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::EnergySensorsLine3ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine3ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::PowerSensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine1ToLine2 {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine1ToLine2N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine1ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine1ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine2ToLine3 {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine2ToLine3N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine2ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine2ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine3ToLine1 {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine3ToLine1N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine3ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSensorsLine3ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine3ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_supply_metrics::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsEnergykWh>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedPercent")]
        pub fan_speed_percent:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsFanSpeedPercent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedsPercent")]
        pub fan_speeds_percent: Option<Vec<crate::sensor::SensorFanArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FanSpeedsPercent@odata.count"
        )]
        pub fan_speeds_percent_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsFrequencyHz>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputCurrentAmps")]
        pub input_current_amps:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputPowerWatts")]
        pub input_power_watts:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputPowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputVoltage")]
        pub input_voltage:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputVoltage>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputPowerWatts")]
        pub output_power_watts:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsOutputPowerWatts>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhaseCurrentAmps"
        )]
        pub poly_phase_current_amps:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseEnergykWh")]
        pub poly_phase_energyk_wh:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseEnergykWh>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhasePowerWatts"
        )]
        pub poly_phase_power_watts:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhasePowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseVoltage")]
        pub poly_phase_voltage:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseVoltage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RailCurrentAmps")]
        pub rail_current_amps: Option<Vec<crate::sensor::SensorCurrentExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RailCurrentAmps@odata.count"
        )]
        pub rail_current_amps_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RailPowerWatts")]
        pub rail_power_watts: Option<Vec<crate::sensor::SensorPowerExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RailPowerWatts@odata.count"
        )]
        pub rail_power_watts_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RailVoltage")]
        pub rail_voltage: Option<Vec<crate::sensor::SensorVoltageExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RailVoltage@odata.count"
        )]
        pub rail_voltage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius:
            Option<crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsTemperatureCelsius>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsDescription {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PowerSupplyMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsEnergykWh {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_3::SensorEnergykWhExcerpt),
    }
    impl Default for PowerSupplyMetricsEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsFanSpeedPercent {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsFanSpeedPercentN1),
        SensorSensorFanExcerpt(crate::sensor::v1_9_3::SensorFanExcerpt),
    }
    impl Default for PowerSupplyMetricsFanSpeedPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsFanSpeedPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsFrequencyHz {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsFrequencyHzN1),
        SensorSensorExcerpt(crate::sensor::v1_9_3::SensorExcerpt),
    }
    impl Default for PowerSupplyMetricsFrequencyHz {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsFrequencyHzN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsInputCurrentAmps {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputCurrentAmpsN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for PowerSupplyMetricsInputCurrentAmps {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsInputCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsInputPowerWatts {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSupplyMetricsInputPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsInputPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsInputVoltage {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsInputVoltageN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for PowerSupplyMetricsInputVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsInputVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsOutputPowerWatts {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsOutputPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_3::SensorPowerExcerpt),
    }
    impl Default for PowerSupplyMetricsOutputPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsOutputPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsPolyPhaseCurrentAmps {
        V010200(crate::power_supply_metrics::v1_2_0::CurrentSensors),
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseCurrentAmpsN1),
    }
    impl Default for PowerSupplyMetricsPolyPhaseCurrentAmps {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsPolyPhaseCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsPolyPhaseEnergykWh {
        V010200(crate::power_supply_metrics::v1_2_0::EnergySensors),
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseEnergykWhN1),
    }
    impl Default for PowerSupplyMetricsPolyPhaseEnergykWh {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsPolyPhaseEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsPolyPhasePowerWatts {
        V010200(crate::power_supply_metrics::v1_2_0::PowerSensors),
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhasePowerWattsN1),
    }
    impl Default for PowerSupplyMetricsPolyPhasePowerWatts {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsPolyPhasePowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsPolyPhaseVoltage {
        V010200(crate::power_supply_metrics::v1_2_0::VoltageSensors),
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsPolyPhaseVoltageN1),
    }
    impl Default for PowerSupplyMetricsPolyPhaseVoltage {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsPolyPhaseVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSupplyMetricsTemperatureCelsius {
        V000001(crate::power_supply_metrics::v1_2_0::PowerSupplyMetricsTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_3::SensorExcerpt),
    }
    impl Default for PowerSupplyMetricsTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyMetricsTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    pub struct VoltageSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral:
            Option<crate::power_supply_metrics::v1_2_0::VoltageSensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine1ToLine2 {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine1ToLine2N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine1ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine1ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine2ToLine3 {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine2ToLine3N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine2ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine2ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine3ToLine1 {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine3ToLine1N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine3ToNeutral {
        V000001(crate::power_supply_metrics::v1_2_0::VoltageSensorsLine3ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine3ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
