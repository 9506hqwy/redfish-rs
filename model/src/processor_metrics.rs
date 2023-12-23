pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::processor_metrics::v1_6_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ProcessorMetrics.ClearCurrentPeriod"
        )]
        pub processor_metrics_clear_current_period:
            Option<crate::processor_metrics::v1_6_1::ClearCurrentPeriod>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CStateResidency {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Level")]
        pub level: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResidencyPercent")]
        pub residency_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheMiss")]
        pub cache_miss: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CacheMissesPerInstruction"
        )]
        pub cache_misses_per_instruction: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HitRatio")]
        pub hit_ratio: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Level")]
        pub level: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OccupancyBytes")]
        pub occupancy_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OccupancyPercent")]
        pub occupancy_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheMetricsTotal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::processor_metrics::v1_6_1::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::processor_metrics::v1_6_1::LifeTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoreMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CStateResidency")]
        pub c_state_residency: Option<Vec<crate::processor_metrics::v1_6_1::CStateResidency>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCache")]
        pub core_cache: Option<Vec<crate::processor_metrics::v1_6_1::CacheMetrics>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreId")]
        pub core_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableCoreErrorCount"
        )]
        pub correctable_core_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableOtherErrorCount"
        )]
        pub correctable_other_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InstructionsPerCycle"
        )]
        pub instructions_per_cycle: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStallCount")]
        pub io_stall_count: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryStallCount")]
        pub memory_stall_count: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableCoreErrorCount"
        )]
        pub uncorrectable_core_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableOtherErrorCount"
        )]
        pub uncorrectable_other_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UnhaltedCycles")]
        pub unhalted_cycles: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::processor_metrics::v1_6_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageFrequencyMHz"
        )]
        pub average_frequency_mhz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cache")]
        pub cache: Option<Vec<crate::processor_metrics::v1_6_1::CacheMetrics>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheMetricsTotal")]
        pub cache_metrics_total: Option<crate::processor_metrics::v1_6_1::CacheMetricsTotal>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsumedPowerWatt")]
        pub consumed_power_watt: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreMetrics")]
        pub core_metrics: Option<Vec<crate::processor_metrics::v1_6_1::CoreMetrics>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreVoltage")]
        pub core_voltage: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableCoreErrorCount"
        )]
        pub correctable_core_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableOtherErrorCount"
        )]
        pub correctable_other_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyRatio")]
        pub frequency_ratio: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelPercent")]
        pub kernel_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalMemoryBandwidthBytes"
        )]
        pub local_memory_bandwidth_bytes: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeErrors")]
        pub pcie_errors: Option<crate::pcie_device::PCIeErrors>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerLimitThrottleDuration"
        )]
        pub power_limit_throttle_duration: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteMemoryBandwidthBytes"
        )]
        pub remote_memory_bandwidth_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalLimitThrottleDuration"
        )]
        pub thermal_limit_throttle_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThrottlingCelsius")]
        pub throttling_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableCoreErrorCount"
        )]
        pub uncorrectable_core_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableOtherErrorCount"
        )]
        pub uncorrectable_other_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserPercent")]
        pub user_percent: Option<f64>,
    }
}
