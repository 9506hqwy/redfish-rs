pub type TelemetryService = crate::telemetry_service::v1_3_4::TelemetryService;
pub mod v1_3_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::telemetry_service::v1_3_4::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#TelemetryService.ClearMetricReports"
        )]
        pub telemetry_service_clear_metric_reports:
            Option<crate::telemetry_service::v1_3_4::ClearMetricReports>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#TelemetryService.ResetMetricReportDefinitionsToDefaults"
        )]
        pub telemetry_service_reset_metric_report_definitions_to_defaults:
            Option<crate::telemetry_service::v1_3_4::ResetMetricReportDefinitionsToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#TelemetryService.ResetTriggersToDefaults"
        )]
        pub telemetry_service_reset_triggers_to_defaults:
            Option<crate::telemetry_service::v1_3_4::ResetTriggersToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#TelemetryService.SubmitTestMetricReport"
        )]
        pub telemetry_service_submit_test_metric_report:
            Option<crate::telemetry_service::v1_3_4::SubmitTestMetricReport>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearMetricReports {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearMetricReportsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CollectionFunction {
        #[default]
        #[serde(rename = "Average")]
        Average,
        #[serde(rename = "Maximum")]
        Maximum,
        #[serde(rename = "Minimum")]
        Minimum,
        #[serde(rename = "Summation")]
        Summation,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetricReportDefinitionsToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetricReportDefinitionsToDefaultsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetTriggersToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetTriggersToDefaultsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestMetricReport {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestMetricReportRequestBody {
        #[serde(rename = "GeneratedMetricReportValues")]
        pub generated_metric_report_values: Vec<crate::telemetry_service::v1_3_4::MetricValue>,
        #[serde(rename = "MetricReportName")]
        pub metric_report_name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReportValues")]
        pub metric_report_values: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TelemetryService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::telemetry_service::v1_3_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::telemetry_service::v1_3_4::TelemetryServiceDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogService")]
        pub log_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReports")]
        pub max_reports: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDefinitions")]
        pub metric_definitions: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitions"
        )]
        pub metric_report_definitions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReports")]
        pub metric_reports: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinCollectionInterval"
        )]
        pub min_collection_interval: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedCollectionFunctions"
        )]
        pub supported_collection_functions: Option<
            Vec<crate::telemetry_service::v1_3_4::TelemetryServiceSupportedCollectionFunctions>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Triggers")]
        pub triggers: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TelemetryServiceDescription {
        V000001(crate::telemetry_service::v1_3_4::TelemetryServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for TelemetryServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TelemetryServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TelemetryServiceSupportedCollectionFunctions {
        V010304(crate::telemetry_service::v1_3_4::CollectionFunction),
        V000001(crate::telemetry_service::v1_3_4::TelemetryServiceSupportedCollectionFunctionsN1),
    }
    impl Default for TelemetryServiceSupportedCollectionFunctions {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TelemetryServiceSupportedCollectionFunctionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
