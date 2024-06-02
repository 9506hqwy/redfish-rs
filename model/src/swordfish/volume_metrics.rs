pub type VolumeMetrics = crate::swordfish::volume_metrics::v1_1_0::VolumeMetrics;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::volume_metrics::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VolumeMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::volume_metrics::v1_1_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckCount"
        )]
        pub consistency_check_count: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckErrorCount"
        )]
        pub consistency_check_error_count: Option<f64>,
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
        pub description: Option<crate::swordfish::volume_metrics::v1_1_0::VolumeMetricsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::v1_0_1::IOStatistics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RebuildErrorCount")]
        pub rebuild_error_count: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StateChangeCount")]
        pub state_change_count: Option<f64>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeMetricsDescription {
        V000001(crate::swordfish::volume_metrics::v1_1_0::VolumeMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VolumeMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
