pub type ThermalMetrics = crate::thermal_metrics::v1_3_2::ThermalMetrics;
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_metrics::v1_3_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ThermalMetrics.ResetMetrics"
        )]
        pub thermal_metrics_reset_metrics: Option<crate::thermal_metrics::v1_3_1::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HeaterSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalPrePowerOnHeatingTimeSeconds"
        )]
        pub total_pre_power_on_heating_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalRuntimeHeatingTimeSeconds"
        )]
        pub total_runtime_heating_time_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
    pub struct TemperatureSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ambient")]
        pub ambient: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Exhaust")]
        pub exhaust: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Intake")]
        pub intake: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Internal")]
        pub internal: Option<crate::sensor::SensorExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal_metrics::v1_3_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AirFlowCubicMetersPerMinute"
        )]
        pub air_flow_cubic_meters_per_minute: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeaterSummary")]
        pub heater_summary: Option<crate::thermal_metrics::v1_3_1::HeaterSummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureReadingsCelsius"
        )]
        pub temperature_readings_celsius: Option<Vec<crate::sensor::SensorArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureReadingsCelsius@odata.count"
        )]
        pub temperature_readings_celsius_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureSummaryCelsius"
        )]
        pub temperature_summary_celsius: Option<crate::thermal_metrics::v1_3_1::TemperatureSummary>,
    }
}
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_metrics::v1_3_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ThermalMetrics.ResetMetrics"
        )]
        pub thermal_metrics_reset_metrics: Option<crate::thermal_metrics::v1_3_2::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HeaterSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalPrePowerOnHeatingTimeSeconds"
        )]
        pub total_pre_power_on_heating_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalRuntimeHeatingTimeSeconds"
        )]
        pub total_runtime_heating_time_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
    pub struct TemperatureSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ambient")]
        pub ambient: Option<crate::thermal_metrics::v1_3_2::TemperatureSummaryAmbient>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Exhaust")]
        pub exhaust: Option<crate::thermal_metrics::v1_3_2::TemperatureSummaryExhaust>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Intake")]
        pub intake: Option<crate::thermal_metrics::v1_3_2::TemperatureSummaryIntake>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Internal")]
        pub internal: Option<crate::thermal_metrics::v1_3_2::TemperatureSummaryInternal>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TemperatureSummaryAmbient {
        V000001(crate::thermal_metrics::v1_3_2::TemperatureSummaryAmbientN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for TemperatureSummaryAmbient {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TemperatureSummaryAmbientN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TemperatureSummaryExhaust {
        V000001(crate::thermal_metrics::v1_3_2::TemperatureSummaryExhaustN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for TemperatureSummaryExhaust {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TemperatureSummaryExhaustN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TemperatureSummaryIntake {
        V000001(crate::thermal_metrics::v1_3_2::TemperatureSummaryIntakeN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for TemperatureSummaryIntake {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TemperatureSummaryIntakeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TemperatureSummaryInternal {
        V000001(crate::thermal_metrics::v1_3_2::TemperatureSummaryInternalN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for TemperatureSummaryInternal {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TemperatureSummaryInternalN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal_metrics::v1_3_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AirFlowCubicMetersPerMinute"
        )]
        pub air_flow_cubic_meters_per_minute:
            Option<crate::thermal_metrics::v1_3_2::ThermalMetricsAirFlowCubicMetersPerMinute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa:
            Option<crate::thermal_metrics::v1_3_2::ThermalMetricsDeltaPressurekPa>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::thermal_metrics::v1_3_2::ThermalMetricsDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::thermal_metrics::v1_3_2::ThermalMetricsEnergykWh>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeaterSummary")]
        pub heater_summary: Option<crate::thermal_metrics::v1_3_2::HeaterSummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::thermal_metrics::v1_3_2::ThermalMetricsPowerWatts>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureReadingsCelsius"
        )]
        pub temperature_readings_celsius: Option<Vec<crate::sensor::SensorArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureReadingsCelsius@odata.count"
        )]
        pub temperature_readings_celsius_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureSummaryCelsius"
        )]
        pub temperature_summary_celsius: Option<crate::thermal_metrics::v1_3_2::TemperatureSummary>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalMetricsAirFlowCubicMetersPerMinute {
        V000001(crate::thermal_metrics::v1_3_2::ThermalMetricsAirFlowCubicMetersPerMinuteN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for ThermalMetricsAirFlowCubicMetersPerMinute {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalMetricsAirFlowCubicMetersPerMinuteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalMetricsDeltaPressurekPa {
        V000001(crate::thermal_metrics::v1_3_2::ThermalMetricsDeltaPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_9_1::SensorExcerpt),
    }
    impl Default for ThermalMetricsDeltaPressurekPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalMetricsDeltaPressurekPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalMetricsDescription {
        V000001(crate::thermal_metrics::v1_3_2::ThermalMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ThermalMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalMetricsEnergykWh {
        V000001(crate::thermal_metrics::v1_3_2::ThermalMetricsEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_1::SensorEnergykWhExcerpt),
    }
    impl Default for ThermalMetricsEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalMetricsEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalMetricsPowerWatts {
        V000001(crate::thermal_metrics::v1_3_2::ThermalMetricsPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_1::SensorPowerExcerpt),
    }
    impl Default for ThermalMetricsPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalMetricsPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
