pub type MetricReportDefinition = crate::metric_report_definition::v1_4_7::MetricReportDefinition;
pub mod v1_4_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_report_definition::v1_4_7::OemActions>,
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
            Option<crate::metric_report_definition::v1_4_7::MetricCollectionFunction>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CollectionTimeScope"
        )]
        pub collection_time_scope:
            Option<crate::metric_report_definition::v1_4_7::MetricCollectionTimeScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricId")]
        pub metric_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricCollectionFunction {
        V010407(crate::metric_report_definition::v1_4_7::CalculationAlgorithmEnum),
        V000001(crate::metric_report_definition::v1_4_7::MetricCollectionFunctionN1),
    }
    impl Default for MetricCollectionFunction {
        fn default() -> Self {
            Self::V010407(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricCollectionFunctionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricCollectionTimeScope {
        V010407(crate::metric_report_definition::v1_4_7::CollectionTimeScope),
        V000001(crate::metric_report_definition::v1_4_7::MetricCollectionTimeScopeN1),
    }
    impl Default for MetricCollectionTimeScope {
        fn default() -> Self {
            Self::V010407(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricCollectionTimeScopeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricReportDefinition { # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: metric_report_definition :: v1_4_7 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "AppendLimit")] pub append_limit : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: metric_report_definition :: v1_4_7 :: MetricReportDefinitionDescription > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Links")] pub links : Option < crate :: metric_report_definition :: v1_4_7 :: Links > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MetricProperties")] pub metric_properties : Option < Vec < String > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MetricReport")] pub metric_report : Option < crate :: odata_v4 :: IdRef > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MetricReportDefinitionEnabled")] pub metric_report_definition_enabled : Option < bool > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MetricReportDefinitionType")] pub metric_report_definition_type : Option < crate :: metric_report_definition :: v1_4_7 :: MetricReportDefinitionMetricReportDefinitionType > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MetricReportHeartbeatInterval")] pub metric_report_heartbeat_interval : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Metrics")] pub metrics : Option < Vec < crate :: metric_report_definition :: v1_4_7 :: Metric > > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReportActions")] pub report_actions : Option < Vec < crate :: metric_report_definition :: v1_4_7 :: ReportActionsEnum > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReportTimespan")] pub report_timespan : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReportUpdates")] pub report_updates : Option < crate :: metric_report_definition :: v1_4_7 :: ReportUpdatesEnum > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Schedule")] pub schedule : Option < crate :: schedule :: Schedule > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Status")] pub status : Option < crate :: resource :: Status > , # [serde (skip_serializing_if = "Option::is_none" , rename = "SuppressRepeatedMetricValue")] pub suppress_repeated_metric_value : Option < bool > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Wildcards")] pub wildcards : Option < Vec < crate :: metric_report_definition :: v1_4_7 :: Wildcard > > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricReportDefinitionDescription {
        V000001(crate::metric_report_definition::v1_4_7::MetricReportDefinitionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for MetricReportDefinitionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricReportDefinitionMetricReportDefinitionType {
        V010407 (crate :: metric_report_definition :: v1_4_7 :: MetricReportDefinitionType) , V000001 (crate :: metric_report_definition :: v1_4_7 :: MetricReportDefinitionMetricReportDefinitionTypeN1) }
    impl Default for MetricReportDefinitionMetricReportDefinitionType {
        fn default() -> Self {
            Self::V010407(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricReportDefinitionMetricReportDefinitionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
