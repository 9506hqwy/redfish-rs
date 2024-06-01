pub type EnvironmentMetrics = crate::environment_metrics::v1_3_1::EnvironmentMetrics;
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetMetrics"
        )]
        pub environment_metrics_reset_metrics:
            Option<crate::environment_metrics::v1_3_0::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetToDefaults"
        )]
        pub environment_metrics_reset_to_defaults:
            Option<crate::environment_metrics::v1_3_0::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::environment_metrics::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnvironmentMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AbsoluteHumidity")]
        pub absolute_humidity: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::environment_metrics::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DewPointCelsius")]
        pub dew_point_celsius: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergyJoules")]
        pub energy_joules: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedsPercent")]
        pub fan_speeds_percent: Option<Vec<crate::sensor::SensorFanArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FanSpeedsPercent@odata.count"
        )]
        pub fan_speeds_percent_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HumidityPercent")]
        pub humidity_percent: Option<crate::sensor::SensorExcerpt>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimitWatts")]
        pub power_limit_watts: Option<crate::control::ControlSingleExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLoadPercent")]
        pub power_load_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<crate::sensor::SensorExcerpt>,
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
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {}
}
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetMetrics"
        )]
        pub environment_metrics_reset_metrics:
            Option<crate::environment_metrics::v1_3_1::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetToDefaults"
        )]
        pub environment_metrics_reset_to_defaults:
            Option<crate::environment_metrics::v1_3_1::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::environment_metrics::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnvironmentMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AbsoluteHumidity")]
        pub absolute_humidity: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::environment_metrics::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DewPointCelsius")]
        pub dew_point_celsius: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergyJoules")]
        pub energy_joules: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedsPercent")]
        pub fan_speeds_percent: Option<Vec<crate::sensor::SensorFanArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FanSpeedsPercent@odata.count"
        )]
        pub fan_speeds_percent_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HumidityPercent")]
        pub humidity_percent: Option<crate::sensor::SensorExcerpt>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimitWatts")]
        pub power_limit_watts: Option<crate::control::ControlSingleExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLoadPercent")]
        pub power_load_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<crate::sensor::SensorExcerpt>,
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
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {}
}
