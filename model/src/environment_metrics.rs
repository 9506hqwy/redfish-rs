pub type EnvironmentMetrics = crate::environment_metrics::v1_3_2::EnvironmentMetrics;
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
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetMetrics"
        )]
        pub environment_metrics_reset_metrics:
            Option<crate::environment_metrics::v1_3_2::ResetMetrics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EnvironmentMetrics.ResetToDefaults"
        )]
        pub environment_metrics_reset_to_defaults:
            Option<crate::environment_metrics::v1_3_2::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::environment_metrics::v1_3_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnvironmentMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AbsoluteHumidity")]
        pub absolute_humidity:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsAbsoluteHumidity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::environment_metrics::v1_3_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DewPointCelsius")]
        pub dew_point_celsius:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsDewPointCelsius>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergyJoules")]
        pub energy_joules:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsEnergyJoules>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsEnergykWh>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedsPercent")]
        pub fan_speeds_percent: Option<Vec<crate::sensor::SensorFanArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FanSpeedsPercent@odata.count"
        )]
        pub fan_speeds_percent_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HumidityPercent")]
        pub humidity_percent:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsHumidityPercent>,
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
        pub power_limit_watts:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerLimitWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLoadPercent")]
        pub power_load_percent:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerLoadPercent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius:
            Option<crate::environment_metrics::v1_3_2::EnvironmentMetricsTemperatureCelsius>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsAbsoluteHumidity {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsAbsoluteHumidityN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsAbsoluteHumidity {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsAbsoluteHumidityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsDescription {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EnvironmentMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsDewPointCelsius {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsDewPointCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsDewPointCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsDewPointCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsEnergyJoules {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsEnergyJoulesN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsEnergyJoules {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsEnergyJoulesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsEnergykWh {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_0::SensorEnergykWhExcerpt),
    }
    impl Default for EnvironmentMetricsEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsHumidityPercent {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsHumidityPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsHumidityPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsHumidityPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsPowerLimitWatts {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerLimitWattsN1),
        ControlControlSingleExcerpt(crate::control::v1_5_1::ControlSingleExcerpt),
    }
    impl Default for EnvironmentMetricsPowerLimitWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsPowerLimitWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsPowerLoadPercent {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerLoadPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsPowerLoadPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsPowerLoadPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsPowerWatts {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_0::SensorPowerExcerpt),
    }
    impl Default for EnvironmentMetricsPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnvironmentMetricsTemperatureCelsius {
        V000001(crate::environment_metrics::v1_3_2::EnvironmentMetricsTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_0::SensorExcerpt),
    }
    impl Default for EnvironmentMetricsTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentMetricsTemperatureCelsiusN1 {
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
