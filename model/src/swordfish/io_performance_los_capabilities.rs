use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IOPerformanceLoSCapabilities {
    V010300(
        crate::swordfish::io_performance_los_capabilities::v1_3_0::IOPerformanceLoSCapabilities,
    ),
    V010200(
        crate::swordfish::io_performance_los_capabilities::v1_2_0::IOPerformanceLoSCapabilities,
    ),
    V010103(
        crate::swordfish::io_performance_los_capabilities::v1_1_3::IOPerformanceLoSCapabilities,
    ),
    V010002(
        crate::swordfish::io_performance_los_capabilities::v1_0_2::IOPerformanceLoSCapabilities,
    ),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IOWorkload {
    V010300(crate::swordfish::io_performance_los_capabilities::v1_3_0::IOWorkload),
    V010200(crate::swordfish::io_performance_los_capabilities::v1_2_0::IOWorkload),
    V010103(crate::swordfish::io_performance_los_capabilities::v1_1_3::IOWorkload),
    V010002(crate::swordfish::io_performance_los_capabilities::v1_0_2::IOWorkload),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IOWorkloadComponent {
    V010300(crate::swordfish::io_performance_los_capabilities::v1_3_0::IOWorkloadComponent),
    V010200(crate::swordfish::io_performance_los_capabilities::v1_2_0::IOWorkloadComponent),
    V010103(crate::swordfish::io_performance_los_capabilities::v1_1_3::IOWorkloadComponent),
    V010002(crate::swordfish::io_performance_los_capabilities::v1_0_2::IOWorkloadComponent),
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOAccessPattern {
        #[default]
        #[serde(rename = "RandomReadAgain")]
        RandomReadAgain,
        #[serde(rename = "RandomReadNew")]
        RandomReadNew,
        #[serde(rename = "ReadWrite")]
        ReadWrite,
        #[serde(rename = "SequentialRead")]
        SequentialRead,
        #[serde(rename = "SequentialWrite")]
        SequentialWrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOLimitingIsSupported"
        )]
        pub io_limiting_is_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSamplePeriod")]
        pub max_sample_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSamplePeriod")]
        pub min_sample_period: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinSupportedIoOperationLatencyMicroseconds"
        )]
        pub min_supported_io_operation_latency_microseconds: Option<i64>,
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
            rename = "SupportedIOWorkloads"
        )]
        pub supported_io_workloads:
            Option<Vec<crate::swordfish::io_performance_los_capabilities::v1_0_0::IOWorkload>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Components")]
        pub components: Option<
            Vec<crate::swordfish::io_performance_los_capabilities::v1_0_0::IOWorkloadComponent>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkloadComponent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageIOBytes")]
        pub average_io_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Duration")]
        pub duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOAccessPattern")]
        pub io_access_pattern:
            Option<crate::swordfish::io_performance_los_capabilities::v1_0_0::IOAccessPattern>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfData")]
        pub percent_of_data: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfIOPS")]
        pub percent_of_iops: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
    }
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOAccessPattern {
        #[default]
        #[serde(rename = "RandomReadAgain")]
        RandomReadAgain,
        #[serde(rename = "RandomReadNew")]
        RandomReadNew,
        #[serde(rename = "ReadWrite")]
        ReadWrite,
        #[serde(rename = "SequentialRead")]
        SequentialRead,
        #[serde(rename = "SequentialWrite")]
        SequentialWrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOLimitingIsSupported"
        )]
        pub io_limiting_is_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSamplePeriod")]
        pub max_sample_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSamplePeriod")]
        pub min_sample_period: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinSupportedIoOperationLatencyMicroseconds"
        )]
        pub min_supported_io_operation_latency_microseconds: Option<i64>,
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
            rename = "SupportedIOWorkloads"
        )]
        pub supported_io_workloads:
            Option<Vec<crate::swordfish::io_performance_los_capabilities::v1_0_2::IOWorkload>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Components")]
        pub components: Option<
            Vec<crate::swordfish::io_performance_los_capabilities::v1_0_2::IOWorkloadComponent>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkloadComponent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageIOBytes")]
        pub average_io_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Duration")]
        pub duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOAccessPattern")]
        pub io_access_pattern:
            Option<crate::swordfish::io_performance_los_capabilities::v1_0_2::IOAccessPattern>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfData")]
        pub percent_of_data: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfIOPS")]
        pub percent_of_iops: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_performance_los_capabilities::v1_1_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOAccessPattern {
        #[default]
        #[serde(rename = "RandomReadAgain")]
        RandomReadAgain,
        #[serde(rename = "RandomReadNew")]
        RandomReadNew,
        #[serde(rename = "ReadWrite")]
        ReadWrite,
        #[serde(rename = "SequentialRead")]
        SequentialRead,
        #[serde(rename = "SequentialWrite")]
        SequentialWrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::io_performance_los_capabilities::v1_1_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOLimitingIsSupported"
        )]
        pub io_limiting_is_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSamplePeriod")]
        pub max_sample_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSamplePeriod")]
        pub min_sample_period: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinSupportedIoOperationLatencyMicroseconds"
        )]
        pub min_supported_io_operation_latency_microseconds: Option<i64>,
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
            rename = "SupportedIOWorkloads"
        )]
        pub supported_io_workloads:
            Option<Vec<crate::swordfish::io_performance_los_capabilities::v1_1_3::IOWorkload>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Components")]
        pub components: Option<
            Vec<crate::swordfish::io_performance_los_capabilities::v1_1_3::IOWorkloadComponent>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkloadComponent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageIOBytes")]
        pub average_io_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Duration")]
        pub duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOAccessPattern")]
        pub io_access_pattern:
            Option<crate::swordfish::io_performance_los_capabilities::v1_1_3::IOAccessPattern>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfData")]
        pub percent_of_data: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfIOPS")]
        pub percent_of_iops: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_performance_los_capabilities::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOAccessPattern {
        #[default]
        #[serde(rename = "RandomReadAgain")]
        RandomReadAgain,
        #[serde(rename = "RandomReadNew")]
        RandomReadNew,
        #[serde(rename = "ReadWrite")]
        ReadWrite,
        #[serde(rename = "SequentialRead")]
        SequentialRead,
        #[serde(rename = "SequentialWrite")]
        SequentialWrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::io_performance_los_capabilities::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOLimitingIsSupported"
        )]
        pub io_limiting_is_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSamplePeriod")]
        pub max_sample_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSamplePeriod")]
        pub min_sample_period: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinSupportedIoOperationLatencyMicroseconds"
        )]
        pub min_supported_io_operation_latency_microseconds: Option<i64>,
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
            rename = "SupportedIOWorkloads"
        )]
        pub supported_io_workloads:
            Option<Vec<crate::swordfish::io_performance_los_capabilities::v1_2_0::IOWorkload>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Components")]
        pub components: Option<
            Vec<crate::swordfish::io_performance_los_capabilities::v1_2_0::IOWorkloadComponent>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkloadComponent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageIOBytes")]
        pub average_io_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Duration")]
        pub duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOAccessPattern")]
        pub io_access_pattern:
            Option<crate::swordfish::io_performance_los_capabilities::v1_2_0::IOAccessPattern>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfData")]
        pub percent_of_data: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfIOPS")]
        pub percent_of_iops: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_performance_los_capabilities::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOAccessPattern {
        #[default]
        #[serde(rename = "RandomReadAgain")]
        RandomReadAgain,
        #[serde(rename = "RandomReadNew")]
        RandomReadNew,
        #[serde(rename = "ReadWrite")]
        ReadWrite,
        #[serde(rename = "SequentialRead")]
        SequentialRead,
        #[serde(rename = "SequentialWrite")]
        SequentialWrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOPerformanceLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::io_performance_los_capabilities::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOLimitingIsSupported"
        )]
        pub io_limiting_is_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSamplePeriod")]
        pub max_sample_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSamplePeriod")]
        pub min_sample_period: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinSupportedIoOperationLatencyMicroseconds"
        )]
        pub min_supported_io_operation_latency_microseconds: Option<i64>,
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
            rename = "SupportedIOWorkloads"
        )]
        pub supported_io_workloads:
            Option<Vec<crate::swordfish::io_performance_los_capabilities::v1_3_0::IOWorkload>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Components")]
        pub components: Option<
            Vec<crate::swordfish::io_performance_los_capabilities::v1_3_0::IOWorkloadComponent>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOWorkloadComponent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AverageIOBytes")]
        pub average_io_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Duration")]
        pub duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOAccessPattern")]
        pub io_access_pattern:
            Option<crate::swordfish::io_performance_los_capabilities::v1_3_0::IOAccessPattern>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfData")]
        pub percent_of_data: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentOfIOPS")]
        pub percent_of_iops: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
