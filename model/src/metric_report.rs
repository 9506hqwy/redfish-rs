pub type MetricReport = crate::metric_report::v1_5_2::MetricReport;
pub mod v1_5_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report::v1_5_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReport {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report::v1_5_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::metric_report::v1_5_2::MetricReportDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinition"
        )]
        pub metric_report_definition: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricValues")]
        pub metric_values: Option<Vec<crate::metric_report::v1_5_2::MetricValue>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportSequence")]
        pub report_sequence: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricReportDescription {
        V000001(crate::metric_report::v1_5_2::MetricReportDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for MetricReportDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricValue {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDefinition")]
        pub metric_definition: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperty")]
        pub metric_property: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricValue")]
        pub metric_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
