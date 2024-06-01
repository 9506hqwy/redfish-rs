pub type BatteryMetrics = crate::battery_metrics::v1_0_3::BatteryMetrics;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::battery_metrics::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BatteryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::battery_metrics::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CellVoltages")]
        pub cell_voltages: Option<Vec<crate::sensor::SensorVoltageExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CellVoltages@odata.count"
        )]
        pub cell_voltages_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChargePercent")]
        pub charge_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DischargeCycles")]
        pub discharge_cycles: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputCurrentAmps")]
        pub input_current_amps: Option<crate::sensor::SensorCurrentExcerpt>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputCurrentAmps")]
        pub output_current_amps: Option<Vec<crate::sensor::SensorCurrentExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputCurrentAmps@odata.count"
        )]
        pub output_current_amps_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputVoltages")]
        pub output_voltages: Option<Vec<crate::sensor::SensorVoltageExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputVoltages@odata.count"
        )]
        pub output_voltages_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoredChargeAmpHours"
        )]
        pub stored_charge_amp_hours: Option<crate::sensor::SensorExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoredEnergyWattHours"
        )]
        pub stored_energy_watt_hours: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<crate::sensor::SensorExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::battery_metrics::v1_0_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BatteryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::battery_metrics::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CellVoltages")]
        pub cell_voltages: Option<Vec<crate::sensor::SensorVoltageExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CellVoltages@odata.count"
        )]
        pub cell_voltages_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChargePercent")]
        pub charge_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DischargeCycles")]
        pub discharge_cycles: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputCurrentAmps")]
        pub input_current_amps: Option<crate::sensor::SensorCurrentExcerpt>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputCurrentAmps")]
        pub output_current_amps: Option<Vec<crate::sensor::SensorCurrentExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputCurrentAmps@odata.count"
        )]
        pub output_current_amps_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputVoltages")]
        pub output_voltages: Option<Vec<crate::sensor::SensorVoltageExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputVoltages@odata.count"
        )]
        pub output_voltages_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoredChargeAmpHours"
        )]
        pub stored_charge_amp_hours: Option<crate::sensor::SensorExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StoredEnergyWattHours"
        )]
        pub stored_energy_watt_hours: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<crate::sensor::SensorExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
