pub type StorageMetrics = crate::storage_metrics::v1_0_0::StorageMetrics;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage_metrics::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage_metrics::v1_0_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompressionSavingsBytes"
        )]
        pub compression_savings_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeduplicationSavingsBytes"
        )]
        pub deduplication_savings_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::storage_metrics::v1_0_0::StorageMetricsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IOStatistics")]
        pub io_statistics: Option<crate::swordfish::io_statistics::IOStatistics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StateChangeCount")]
        pub state_change_count: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThinProvisioningSavingsBytes"
        )]
        pub thin_provisioning_savings_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageMetricsDescription {
        V000001(crate::storage_metrics::v1_0_0::StorageMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
