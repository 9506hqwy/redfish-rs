use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ManagerDiagnosticData {
    V010200(crate::manager_diagnostic_data::v1_2_0::ManagerDiagnosticData),
    V010100(crate::manager_diagnostic_data::v1_1_0::ManagerDiagnosticData),
    V010000(crate::manager_diagnostic_data::v1_0_0::ManagerDiagnosticData),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerDiagnosticData.ResetMetrics"
        )]
        pub manager_diagnostic_data_reset_metrics:
            Option<crate::manager_diagnostic_data::v1_0_0::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_diagnostic_data::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootTimeStatistics {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareTimeSeconds"
        )]
        pub firmware_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitrdTimeSeconds")]
        pub initrd_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoaderTimeSeconds")]
        pub loader_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserSpaceTimeSeconds"
        )]
        pub user_space_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct I2CBusStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusErrorCount")]
        pub bus_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBusName")]
        pub i2c_bus_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NACKCount")]
        pub nack_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalTransactionCount"
        )]
        pub total_transaction_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_diagnostic_data::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootTimeStatistics")]
        pub boot_time_statistics:
            Option<crate::manager_diagnostic_data::v1_0_0::BootTimeStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FreeStorageSpaceKiB"
        )]
        pub free_storage_space_kib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBuses")]
        pub i2c_buses: Option<Vec<crate::manager_diagnostic_data::v1_0_0::I2CBusStatistics>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryECCStatistics"
        )]
        pub memory_ecc_statistics:
            Option<crate::manager_diagnostic_data::v1_0_0::MemoryECCStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryStatistics")]
        pub memory_statistics: Option<crate::manager_diagnostic_data::v1_0_0::MemoryStatistics>,
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
            rename = "ProcessorStatistics"
        )]
        pub processor_statistics:
            Option<crate::manager_diagnostic_data::v1_0_0::ProcessorStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TopProcesses")]
        pub top_processes: Option<Vec<crate::manager_diagnostic_data::v1_0_0::ProcessStatistics>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryECCStatistics {
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
    pub struct MemoryStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AvailableBytes")]
        pub available_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BuffersAndCacheBytes"
        )]
        pub buffers_and_cache_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FreeBytes")]
        pub free_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharedBytes")]
        pub shared_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalBytes")]
        pub total_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsedBytes")]
        pub used_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommandLine")]
        pub command_line: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResidentSetSizeBytes"
        )]
        pub resident_set_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserTimeSeconds")]
        pub user_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelPercent")]
        pub kernel_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserPercent")]
        pub user_percent: Option<f64>,
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
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerDiagnosticData.ResetMetrics"
        )]
        pub manager_diagnostic_data_reset_metrics:
            Option<crate::manager_diagnostic_data::v1_1_0::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_diagnostic_data::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootTimeStatistics {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareTimeSeconds"
        )]
        pub firmware_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitrdTimeSeconds")]
        pub initrd_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoaderTimeSeconds")]
        pub loader_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserSpaceTimeSeconds"
        )]
        pub user_space_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct I2CBusStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusErrorCount")]
        pub bus_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBusName")]
        pub i2c_bus_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NACKCount")]
        pub nack_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalTransactionCount"
        )]
        pub total_transaction_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_diagnostic_data::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootTimeStatistics")]
        pub boot_time_statistics:
            Option<crate::manager_diagnostic_data::v1_1_0::BootTimeStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FreeStorageSpaceKiB"
        )]
        pub free_storage_space_kib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBuses")]
        pub i2c_buses: Option<Vec<crate::manager_diagnostic_data::v1_1_0::I2CBusStatistics>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryECCStatistics"
        )]
        pub memory_ecc_statistics:
            Option<crate::manager_diagnostic_data::v1_1_0::MemoryECCStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryStatistics")]
        pub memory_statistics: Option<crate::manager_diagnostic_data::v1_1_0::MemoryStatistics>,
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
            rename = "ProcessorStatistics"
        )]
        pub processor_statistics:
            Option<crate::manager_diagnostic_data::v1_1_0::ProcessorStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TopProcesses")]
        pub top_processes: Option<Vec<crate::manager_diagnostic_data::v1_1_0::ProcessStatistics>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryECCStatistics {
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
    pub struct MemoryStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AvailableBytes")]
        pub available_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BuffersAndCacheBytes"
        )]
        pub buffers_and_cache_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FreeBytes")]
        pub free_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharedBytes")]
        pub shared_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalBytes")]
        pub total_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsedBytes")]
        pub used_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommandLine")]
        pub command_line: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResidentSetSizeBytes"
        )]
        pub resident_set_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestartAfterFailureCount"
        )]
        pub restart_after_failure_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RestartCount")]
        pub restart_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UptimeSeconds")]
        pub uptime_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserTimeSeconds")]
        pub user_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelPercent")]
        pub kernel_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserPercent")]
        pub user_percent: Option<f64>,
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
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerDiagnosticData.ResetMetrics"
        )]
        pub manager_diagnostic_data_reset_metrics:
            Option<crate::manager_diagnostic_data::v1_2_0::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_diagnostic_data::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootTimeStatistics {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareTimeSeconds"
        )]
        pub firmware_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitrdTimeSeconds")]
        pub initrd_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoaderTimeSeconds")]
        pub loader_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserSpaceTimeSeconds"
        )]
        pub user_space_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct I2CBusStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusErrorCount")]
        pub bus_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBusName")]
        pub i2c_bus_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NACKCount")]
        pub nack_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalTransactionCount"
        )]
        pub total_transaction_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_diagnostic_data::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootTimeStatistics")]
        pub boot_time_statistics:
            Option<crate::manager_diagnostic_data::v1_2_0::BootTimeStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FreeStorageSpaceKiB"
        )]
        pub free_storage_space_kib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "I2CBuses")]
        pub i2c_buses: Option<Vec<crate::manager_diagnostic_data::v1_2_0::I2CBusStatistics>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryECCStatistics"
        )]
        pub memory_ecc_statistics:
            Option<crate::manager_diagnostic_data::v1_2_0::MemoryECCStatistics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryStatistics")]
        pub memory_statistics: Option<crate::manager_diagnostic_data::v1_2_0::MemoryStatistics>,
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
            rename = "ProcessorStatistics"
        )]
        pub processor_statistics:
            Option<crate::manager_diagnostic_data::v1_2_0::ProcessorStatistics>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceRootUptimeSeconds"
        )]
        pub service_root_uptime_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TopProcesses")]
        pub top_processes: Option<Vec<crate::manager_diagnostic_data::v1_2_0::ProcessStatistics>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryECCStatistics {
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
    pub struct MemoryStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AvailableBytes")]
        pub available_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BuffersAndCacheBytes"
        )]
        pub buffers_and_cache_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FreeBytes")]
        pub free_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharedBytes")]
        pub shared_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalBytes")]
        pub total_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsedBytes")]
        pub used_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommandLine")]
        pub command_line: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelTimeSeconds")]
        pub kernel_time_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResidentSetSizeBytes"
        )]
        pub resident_set_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestartAfterFailureCount"
        )]
        pub restart_after_failure_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RestartCount")]
        pub restart_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UptimeSeconds")]
        pub uptime_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserTimeSeconds")]
        pub user_time_seconds: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelPercent")]
        pub kernel_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserPercent")]
        pub user_percent: Option<f64>,
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
