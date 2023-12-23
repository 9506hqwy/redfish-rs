pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_performance_line_of_service::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::io_performance_line_of_service::v1_1_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageIOOperationLatencyMicroseconds"
        )]
        pub average_io_operation_latency_microseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOOperationsPerSecondIsLimited"
        )]
        pub io_operations_per_second_is_limited: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOWorkload")]
        pub io_workload:
            Option<crate::swordfish::io_performance_los_capabilities::v1_0_0::IOWorkload>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxIOOperationsPerSecondPerTerabyte"
        )]
        pub max_io_operations_per_second_per_terabyte: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SamplePeriod")]
        pub sample_period: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
