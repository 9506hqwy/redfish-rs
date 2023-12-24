pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::triggers::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DirectionOfCrossingEnum {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DiscreteTrigger {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Value")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DiscreteTriggerConditionEnum {
        #[default]
        #[serde(rename = "Changed")]
        Changed,
        #[serde(rename = "Specified")]
        Specified,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitions"
        )]
        pub metric_report_definitions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinitions@odata.count"
        )]
        pub metric_report_definitions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricTypeEnum {
        #[default]
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Numeric")]
        Numeric,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Threshold {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Activation")]
        pub activation: Option<crate::triggers::v1_3_1::ThresholdActivation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DwellTime")]
        pub dwell_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivation {
        #[default]
        #[serde(rename = "Decreasing")]
        Decreasing,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Either")]
        Either,
        #[serde(rename = "Increasing")]
        Increasing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::triggers::v1_3_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerWarning")]
        pub lower_warning: Option<crate::triggers::v1_3_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::triggers::v1_3_1::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperWarning")]
        pub upper_warning: Option<crate::triggers::v1_3_1::Threshold>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TriggerActionEnum {
        #[default]
        #[serde(rename = "LogToLogService")]
        LogToLogService,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
        #[serde(rename = "RedfishMetricReport")]
        RedfishMetricReport,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Triggers {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::triggers::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiscreteTriggerCondition"
        )]
        pub discrete_trigger_condition:
            Option<crate::triggers::v1_3_1::DiscreteTriggerConditionEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteTriggers")]
        pub discrete_triggers: Option<Vec<crate::triggers::v1_3_1::DiscreteTrigger>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTriggers")]
        pub event_triggers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisDuration")]
        pub hysteresis_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisReading")]
        pub hysteresis_reading: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::triggers::v1_3_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricIds")]
        pub metric_ids: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::triggers::v1_3_1::MetricTypeEnum>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NumericThresholds")]
        pub numeric_thresholds: Option<crate::triggers::v1_3_1::Thresholds>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerActions")]
        pub trigger_actions: Option<Vec<crate::triggers::v1_3_1::TriggerActionEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::triggers::v1_3_1::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
