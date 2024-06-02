pub type FileSystemMetrics = crate::swordfish::file_system_metrics::v1_0_0::FileSystemMetrics;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_system_metrics::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileSystemMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_system_metrics::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::swordfish::file_system_metrics::v1_0_0::FileSystemMetricsDescription>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FileSystemMetricsDescription {
        V000001(crate::swordfish::file_system_metrics::v1_0_0::FileSystemMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for FileSystemMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FileSystemMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
