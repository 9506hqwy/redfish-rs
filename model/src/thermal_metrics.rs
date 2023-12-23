pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_metrics::v1_2_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ThermalMetrics.ResetMetrics"
        )]
        pub thermal_metrics_reset_metrics: Option<crate::thermal_metrics::v1_2_0::ResetMetrics>,
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
        pub actions: Option<crate::thermal_metrics::v1_2_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AirFlowCubicMetersPerMinute"
        )]
        pub air_flow_cubic_meters_per_minute: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeaterSummary")]
        pub heater_summary: Option<crate::thermal_metrics::v1_2_0::HeaterSummary>,
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
        pub temperature_summary_celsius: Option<crate::thermal_metrics::v1_2_0::TemperatureSummary>,
    }
}
