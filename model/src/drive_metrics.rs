pub type DriveMetrics = crate::drive_metrics::v1_3_0::DriveMetrics;
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive_metrics::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DriveMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive_metrics::v1_2_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BadBlockCount")]
        pub bad_block_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableIOReadErrorCount"
        )]
        pub correctable_io_read_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableIOWriteErrorCount"
        )]
        pub correctable_io_write_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::drive_metrics::v1_2_1::DriveMetricsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NativeCommandQueueDepth"
        )]
        pub native_command_queue_depth: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOnHours")]
        pub power_on_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIOKiBytes")]
        pub read_io_kibytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableIOReadErrorCount"
        )]
        pub uncorrectable_io_read_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableIOWriteErrorCount"
        )]
        pub uncorrectable_io_write_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIOKiBytes")]
        pub write_io_kibytes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveMetricsDescription {
        V000001(crate::drive_metrics::v1_2_1::DriveMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for DriveMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::drive_metrics::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DriveMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::drive_metrics::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BadBlockCount")]
        pub bad_block_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableIOReadErrorCount"
        )]
        pub correctable_io_read_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableIOWriteErrorCount"
        )]
        pub correctable_io_write_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::drive_metrics::v1_3_0::DriveMetricsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LifetimeStartDateTime"
        )]
        pub lifetime_start_date_time: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NativeCommandQueueDepth"
        )]
        pub native_command_queue_depth: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOnHours")]
        pub power_on_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIOKiBytes")]
        pub read_io_kibytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableIOReadErrorCount"
        )]
        pub uncorrectable_io_read_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableIOWriteErrorCount"
        )]
        pub uncorrectable_io_write_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIOKiBytes")]
        pub write_io_kibytes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DriveMetricsDescription {
        V000001(crate::drive_metrics::v1_3_0::DriveMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for DriveMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DriveMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
