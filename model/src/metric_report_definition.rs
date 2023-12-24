use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MetricReportDefinition {
    V010403(crate::metric_report_definition::v1_4_3::MetricReportDefinition),
    V010307(crate::metric_report_definition::v1_3_7::MetricReportDefinition),
    V010208(crate::metric_report_definition::v1_2_8::MetricReportDefinition),
    V010109(crate::metric_report_definition::v1_1_9::MetricReportDefinition),
    V010010(crate::metric_report_definition::v1_0_10::MetricReportDefinition),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_0_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
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
    pub enum CollectionTimeScope {
        #[default]
        #[serde(rename = "Interval")]
        Interval,
        #[serde(rename = "Point")]
        Point,
        #[serde(rename = "StartupInterval")]
        StartupInterval,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Metric {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionDuration")]
        pub collection_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionFunction")]
        pub collection_function:
            Option<crate::metric_report_definition::v1_0_10::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_0_10::CollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report_definition::v1_0_10::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AppendLimit")]
        pub append_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReport")]
        pub metric_report: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionType"
        )]
        pub metric_report_definition_type:
            Option<crate::metric_report_definition::v1_0_10::MetricReportDefinitionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<Vec<crate::metric_report_definition::v1_0_10::Metric>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportActions")]
        pub report_actions:
            Option<Vec<crate::metric_report_definition::v1_0_10::ReportActionsEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportUpdates")]
        pub report_updates: Option<crate::metric_report_definition::v1_0_10::ReportUpdatesEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_report_definition::v1_0_10::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionType {
        #[default]
        #[serde(rename = "OnChange")]
        OnChange,
        #[serde(rename = "OnRequest")]
        OnRequest,
        #[serde(rename = "Periodic")]
        Periodic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportActionsEnum {
        #[default]
        #[serde(rename = "LogToMetricReportsCollection")]
        LogToMetricReportsCollection,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportUpdatesEnum {
        #[default]
        #[serde(rename = "AppendStopsWhenFull")]
        AppendStopsWhenFull,
        #[serde(rename = "AppendWrapsWhenFull")]
        AppendWrapsWhenFull,
        #[serde(rename = "NewReport")]
        NewReport,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
}
pub mod v1_1_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_1_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
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
    pub enum CollectionTimeScope {
        #[default]
        #[serde(rename = "Interval")]
        Interval,
        #[serde(rename = "Point")]
        Point,
        #[serde(rename = "StartupInterval")]
        StartupInterval,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Metric {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionDuration")]
        pub collection_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionFunction")]
        pub collection_function:
            Option<crate::metric_report_definition::v1_1_9::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_1_9::CollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report_definition::v1_1_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AppendLimit")]
        pub append_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReport")]
        pub metric_report: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionType"
        )]
        pub metric_report_definition_type:
            Option<crate::metric_report_definition::v1_1_9::MetricReportDefinitionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<Vec<crate::metric_report_definition::v1_1_9::Metric>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportActions")]
        pub report_actions: Option<Vec<crate::metric_report_definition::v1_1_9::ReportActionsEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportUpdates")]
        pub report_updates: Option<crate::metric_report_definition::v1_1_9::ReportUpdatesEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_report_definition::v1_1_9::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionType {
        #[default]
        #[serde(rename = "OnChange")]
        OnChange,
        #[serde(rename = "OnRequest")]
        OnRequest,
        #[serde(rename = "Periodic")]
        Periodic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportActionsEnum {
        #[default]
        #[serde(rename = "LogToMetricReportsCollection")]
        LogToMetricReportsCollection,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportUpdatesEnum {
        #[default]
        #[serde(rename = "AppendStopsWhenFull")]
        AppendStopsWhenFull,
        #[serde(rename = "AppendWrapsWhenFull")]
        AppendWrapsWhenFull,
        #[serde(rename = "NewReport")]
        NewReport,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_2_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_2_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
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
    pub enum CollectionTimeScope {
        #[default]
        #[serde(rename = "Interval")]
        Interval,
        #[serde(rename = "Point")]
        Point,
        #[serde(rename = "StartupInterval")]
        StartupInterval,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Triggers")]
        pub triggers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Triggers@odata.count"
        )]
        pub triggers_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Metric {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionDuration")]
        pub collection_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionFunction")]
        pub collection_function:
            Option<crate::metric_report_definition::v1_2_8::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_2_8::CollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report_definition::v1_2_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AppendLimit")]
        pub append_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::metric_report_definition::v1_2_8::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReport")]
        pub metric_report: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionEnabled"
        )]
        pub metric_report_definition_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionType"
        )]
        pub metric_report_definition_type:
            Option<crate::metric_report_definition::v1_2_8::MetricReportDefinitionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportHeartbeatInterval"
        )]
        pub metric_report_heartbeat_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<Vec<crate::metric_report_definition::v1_2_8::Metric>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportActions")]
        pub report_actions: Option<Vec<crate::metric_report_definition::v1_2_8::ReportActionsEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportUpdates")]
        pub report_updates: Option<crate::metric_report_definition::v1_2_8::ReportUpdatesEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SuppressRepeatedMetricValue"
        )]
        pub suppress_repeated_metric_value: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_report_definition::v1_2_8::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionType {
        #[default]
        #[serde(rename = "OnChange")]
        OnChange,
        #[serde(rename = "OnRequest")]
        OnRequest,
        #[serde(rename = "Periodic")]
        Periodic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportActionsEnum {
        #[default]
        #[serde(rename = "LogToMetricReportsCollection")]
        LogToMetricReportsCollection,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportUpdatesEnum {
        #[default]
        #[serde(rename = "AppendStopsWhenFull")]
        AppendStopsWhenFull,
        #[serde(rename = "AppendWrapsWhenFull")]
        AppendWrapsWhenFull,
        #[serde(rename = "NewReport")]
        NewReport,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_3_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_3_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
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
    pub enum CollectionTimeScope {
        #[default]
        #[serde(rename = "Interval")]
        Interval,
        #[serde(rename = "Point")]
        Point,
        #[serde(rename = "StartupInterval")]
        StartupInterval,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Triggers")]
        pub triggers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Triggers@odata.count"
        )]
        pub triggers_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Metric {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionDuration")]
        pub collection_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionFunction")]
        pub collection_function:
            Option<crate::metric_report_definition::v1_3_7::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_3_7::CollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report_definition::v1_3_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AppendLimit")]
        pub append_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::metric_report_definition::v1_3_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReport")]
        pub metric_report: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionEnabled"
        )]
        pub metric_report_definition_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionType"
        )]
        pub metric_report_definition_type:
            Option<crate::metric_report_definition::v1_3_7::MetricReportDefinitionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportHeartbeatInterval"
        )]
        pub metric_report_heartbeat_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<Vec<crate::metric_report_definition::v1_3_7::Metric>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportActions")]
        pub report_actions: Option<Vec<crate::metric_report_definition::v1_3_7::ReportActionsEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportTimespan")]
        pub report_timespan: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportUpdates")]
        pub report_updates: Option<crate::metric_report_definition::v1_3_7::ReportUpdatesEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SuppressRepeatedMetricValue"
        )]
        pub suppress_repeated_metric_value: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_report_definition::v1_3_7::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionType {
        #[default]
        #[serde(rename = "OnChange")]
        OnChange,
        #[serde(rename = "OnRequest")]
        OnRequest,
        #[serde(rename = "Periodic")]
        Periodic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportActionsEnum {
        #[default]
        #[serde(rename = "LogToMetricReportsCollection")]
        LogToMetricReportsCollection,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportUpdatesEnum {
        #[default]
        #[serde(rename = "AppendStopsWhenFull")]
        AppendStopsWhenFull,
        #[serde(rename = "AppendWrapsWhenFull")]
        AppendWrapsWhenFull,
        #[serde(rename = "NewReport")]
        NewReport,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_4_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_4_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
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
    pub enum CollectionTimeScope {
        #[default]
        #[serde(rename = "Interval")]
        Interval,
        #[serde(rename = "Point")]
        Point,
        #[serde(rename = "StartupInterval")]
        StartupInterval,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Triggers")]
        pub triggers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Triggers@odata.count"
        )]
        pub triggers_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Metric {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionDuration")]
        pub collection_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CollectionFunction")]
        pub collection_function:
            Option<crate::metric_report_definition::v1_4_3::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_4_3::CollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_report_definition::v1_4_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AppendLimit")]
        pub append_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::metric_report_definition::v1_4_3::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricReport")]
        pub metric_report: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionEnabled"
        )]
        pub metric_report_definition_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitionType"
        )]
        pub metric_report_definition_type:
            Option<crate::metric_report_definition::v1_4_3::MetricReportDefinitionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportHeartbeatInterval"
        )]
        pub metric_report_heartbeat_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<Vec<crate::metric_report_definition::v1_4_3::Metric>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportActions")]
        pub report_actions: Option<Vec<crate::metric_report_definition::v1_4_3::ReportActionsEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportTimespan")]
        pub report_timespan: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportUpdates")]
        pub report_updates: Option<crate::metric_report_definition::v1_4_3::ReportUpdatesEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SuppressRepeatedMetricValue"
        )]
        pub suppress_repeated_metric_value: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_report_definition::v1_4_3::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionType {
        #[default]
        #[serde(rename = "OnChange")]
        OnChange,
        #[serde(rename = "OnRequest")]
        OnRequest,
        #[serde(rename = "Periodic")]
        Periodic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportActionsEnum {
        #[default]
        #[serde(rename = "LogToMetricReportsCollection")]
        LogToMetricReportsCollection,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReportUpdatesEnum {
        #[default]
        #[serde(rename = "AppendStopsWhenFull")]
        AppendStopsWhenFull,
        #[serde(rename = "AppendWrapsWhenFull")]
        AppendWrapsWhenFull,
        #[serde(rename = "NewReport")]
        NewReport,
        #[serde(rename = "Overwrite")]
        Overwrite,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
