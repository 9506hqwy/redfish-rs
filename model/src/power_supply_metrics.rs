pub type PowerSupplyMetrics = crate::power_supply_metrics::v1_1_0::PowerSupplyMetrics;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_supply_metrics::v1_1_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#PowerSupplyMetrics.ResetMetrics"
        )]
        pub power_supply_metrics_reset_metrics:
            Option<crate::power_supply_metrics::v1_1_0::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_supply_metrics::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedPercent")]
        pub fan_speed_percent: Option<crate::sensor::SensorFanExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanSpeedsPercent")]
        pub fan_speeds_percent: Option<Vec<crate::sensor::SensorFanArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FanSpeedsPercent@odata.count"
        )]
        pub fan_speeds_percent_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz: Option<crate::sensor::SensorExcerpt>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputCurrentAmps")]
        pub input_current_amps: Option<crate::sensor::SensorCurrentExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputPowerWatts")]
        pub input_power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputVoltage")]
        pub input_voltage: Option<crate::sensor::SensorVoltageExcerpt>,
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
        pub output_power_watts: Option<crate::sensor::SensorPowerExcerpt>,
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
        pub temperature_celsius: Option<crate::sensor::SensorExcerpt>,
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
}
