pub type HeaterMetrics = crate::heater_metrics::v1_0_2::HeaterMetrics;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#HeaterMetrics.ResetMetrics"
        )]
        pub heater_metrics_reset_metrics: Option<crate::heater_metrics::v1_0_0::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::heater_metrics::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HeaterMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::heater_metrics::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
            rename = "PrePowerOnHeatingTimeSeconds"
        )]
        pub pre_power_on_heating_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RuntimeHeatingTimeSeconds"
        )]
        pub runtime_heating_time_seconds: Option<i64>,
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
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#HeaterMetrics.ResetMetrics"
        )]
        pub heater_metrics_reset_metrics: Option<crate::heater_metrics::v1_0_2::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::heater_metrics::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HeaterMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::heater_metrics::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::heater_metrics::v1_0_2::HeaterMetricsDescription>,
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
        pub power_watts: Option<crate::heater_metrics::v1_0_2::HeaterMetricsPowerWatts>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrePowerOnHeatingTimeSeconds"
        )]
        pub pre_power_on_heating_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RuntimeHeatingTimeSeconds"
        )]
        pub runtime_heating_time_seconds: Option<i64>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum HeaterMetricsDescription {
        V000001(crate::heater_metrics::v1_0_2::HeaterMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for HeaterMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HeaterMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum HeaterMetricsPowerWatts {
        V000001(crate::heater_metrics::v1_0_2::HeaterMetricsPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_0::SensorPowerExcerpt),
    }
    impl Default for HeaterMetricsPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HeaterMetricsPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
}
