pub type Triggers = crate::triggers::v1_4_0::Triggers;
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::triggers::v1_3_2::OemActions>,
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
        pub activation: Option<crate::triggers::v1_3_2::ThresholdActivation>,
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
        pub lower_critical: Option<crate::triggers::v1_3_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerWarning")]
        pub lower_warning: Option<crate::triggers::v1_3_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::triggers::v1_3_2::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperWarning")]
        pub upper_warning: Option<crate::triggers::v1_3_2::Threshold>,
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
        pub actions: Option<crate::triggers::v1_3_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiscreteTriggerCondition"
        )]
        pub discrete_trigger_condition:
            Option<crate::triggers::v1_3_2::DiscreteTriggerConditionEnum>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteTriggers")]
        pub discrete_triggers: Option<Vec<crate::triggers::v1_3_2::DiscreteTrigger>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTriggers")]
        pub event_triggers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisDuration")]
        pub hysteresis_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisReading")]
        pub hysteresis_reading: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::triggers::v1_3_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricIds")]
        pub metric_ids: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::triggers::v1_3_2::MetricTypeEnum>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NumericThresholds")]
        pub numeric_thresholds: Option<crate::triggers::v1_3_2::Thresholds>,
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
        pub trigger_actions: Option<Vec<crate::triggers::v1_3_2::TriggerActionEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::triggers::v1_3_2::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::triggers::v1_4_0::OemActions>,
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
        pub severity: Option<crate::triggers::v1_4_0::DiscreteTriggerSeverity>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DiscreteTriggerSeverity {
        V000001(crate::triggers::v1_4_0::DiscreteTriggerSeverityN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for DiscreteTriggerSeverity {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DiscreteTriggerSeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub activation: Option<crate::triggers::v1_4_0::ThresholdActivationAnony>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThresholdActivationAnony {
        V010400(crate::triggers::v1_4_0::ThresholdActivation),
        V000001(crate::triggers::v1_4_0::ThresholdActivationN1),
    }
    impl Default for ThresholdActivationAnony {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdActivationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thresholds {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerCritical")]
        pub lower_critical: Option<crate::triggers::v1_4_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerWarning")]
        pub lower_warning: Option<crate::triggers::v1_4_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperCritical")]
        pub upper_critical: Option<crate::triggers::v1_4_0::Threshold>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperWarning")]
        pub upper_warning: Option<crate::triggers::v1_4_0::Threshold>,
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
    pub enum TriggerActionMessage {
        #[default]
        #[serde(rename = "ConnectionSpeed")]
        ConnectionSpeed,
        #[serde(rename = "DriveMediaLife")]
        DriveMediaLife,
        #[serde(rename = "Telemetry")]
        Telemetry,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Triggers {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::triggers::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::triggers::v1_4_0::TriggersDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiscreteTriggerCondition"
        )]
        pub discrete_trigger_condition:
            Option<crate::triggers::v1_4_0::TriggersDiscreteTriggerCondition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteTriggers")]
        pub discrete_triggers: Option<Vec<crate::triggers::v1_4_0::DiscreteTrigger>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTriggers")]
        pub event_triggers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisDuration")]
        pub hysteresis_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HysteresisReading")]
        pub hysteresis_reading: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::triggers::v1_4_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricIds")]
        pub metric_ids: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::triggers::v1_4_0::TriggersMetricType>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NumericThresholds")]
        pub numeric_thresholds: Option<crate::triggers::v1_4_0::Thresholds>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TriggerActionMessage"
        )]
        pub trigger_action_message: Option<crate::triggers::v1_4_0::TriggersTriggerActionMessage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerActions")]
        pub trigger_actions: Option<Vec<crate::triggers::v1_4_0::TriggerActionEnum>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerEnabled")]
        pub trigger_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::triggers::v1_4_0::Wildcard>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TriggersDescription {
        V000001(crate::triggers::v1_4_0::TriggersDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for TriggersDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TriggersDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TriggersDiscreteTriggerCondition {
        V010400(crate::triggers::v1_4_0::DiscreteTriggerConditionEnum),
        V000001(crate::triggers::v1_4_0::TriggersDiscreteTriggerConditionN1),
    }
    impl Default for TriggersDiscreteTriggerCondition {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TriggersDiscreteTriggerConditionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TriggersMetricType {
        V010400(crate::triggers::v1_4_0::MetricTypeEnum),
        V000001(crate::triggers::v1_4_0::TriggersMetricTypeN1),
    }
    impl Default for TriggersMetricType {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TriggersMetricTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TriggersTriggerActionMessage {
        V010400(crate::triggers::v1_4_0::TriggerActionMessage),
        V000001(crate::triggers::v1_4_0::TriggersTriggerActionMessageN1),
    }
    impl Default for TriggersTriggerActionMessage {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TriggersTriggerActionMessageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
