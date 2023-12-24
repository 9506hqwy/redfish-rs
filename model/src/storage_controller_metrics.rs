use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NVMeSMARTMetrics {
    StorageControllerMetricsV1N0N0NVMeSMARTMetrics(
        crate::storage_controller_metrics::v1_0_0::NVMeSMARTMetrics,
    ),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StorageControllerMetrics {
    StorageControllerMetricsV1N0N0StorageControllerMetrics(
        crate::storage_controller_metrics::v1_0_0::StorageControllerMetrics,
    ),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage_controller_metrics::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EGCriticalWarningSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NamespacesInReadOnlyMode"
        )]
        pub namespaces_in_read_only_mode: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReliabilityDegraded"
        )]
        pub reliability_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareCapacityUnderThreshold"
        )]
        pub spare_capacity_under_threshold: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSMARTMetrics {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AvailableSparePercent"
        )]
        pub available_spare_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AvailableSpareThresholdPercent"
        )]
        pub available_spare_threshold_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompositeTemperatureCelsius"
        )]
        pub composite_temperature_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ControllerBusyTimeMinutes"
        )]
        pub controller_busy_time_minutes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CriticalCompositeTempTimeMinutes"
        )]
        pub critical_composite_temp_time_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CriticalWarnings")]
        pub critical_warnings: Option<crate::storage_controller::NVMeSMARTCriticalWarnings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataUnitsRead")]
        pub data_units_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataUnitsWritten")]
        pub data_units_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EGCriticalWarningSummary"
        )]
        pub eg_critical_warning_summary:
            Option<crate::storage_controller_metrics::v1_0_0::EGCriticalWarningSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostReadCommands")]
        pub host_read_commands: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWriteCommands")]
        pub host_write_commands: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MediaAndDataIntegrityErrors"
        )]
        pub media_and_data_integrity_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberOfErrorInformationLogEntries"
        )]
        pub number_of_error_information_log_entries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentageUsed")]
        pub percentage_used: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCycles")]
        pub power_cycles: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOnHours")]
        pub power_on_hours: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureSensorsCelsius"
        )]
        pub temperature_sensors_celsius: Option<Vec<f64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalMgmtTemp1TotalTimeSeconds"
        )]
        pub thermal_mgmt_temp1_total_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalMgmtTemp1TransitionCount"
        )]
        pub thermal_mgmt_temp1_transition_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalMgmtTemp2TotalTimeSeconds"
        )]
        pub thermal_mgmt_temp2_total_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalMgmtTemp2TransitionCount"
        )]
        pub thermal_mgmt_temp2_transition_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UnsafeShutdowns")]
        pub unsafe_shutdowns: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WarningCompositeTempTimeMinutes"
        )]
        pub warning_composite_temp_time_minutes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageControllerMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage_controller_metrics::v1_0_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableParityErrorCount"
        )]
        pub correctable_parity_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeSMART")]
        pub nvme_smart: Option<crate::storage_controller_metrics::NVMeSMARTMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StateChangeCount")]
        pub state_change_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableParityErrorCount"
        )]
        pub uncorrectable_parity_error_count: Option<i64>,
    }
}
