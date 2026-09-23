use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ConditionType {
    #[default]
    #[serde(rename = "And")]
    And,
    #[serde(rename = "Average")]
    Average,
    #[serde(rename = "Compare")]
    Compare,
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "Or")]
    Or,
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "Sum")]
    Sum,
}
pub type Policy = crate::policy::v1_0_0::Policy;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ResponseAction {
    #[default]
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "OEM")]
    OEM,
    #[serde(rename = "Recheck")]
    Recheck,
    #[serde(rename = "Reset")]
    Reset,
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::policy::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Condition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompareValue")]
        pub compare_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operator")]
        pub operator: Option<crate::policy::v1_0_0::ConditionOperator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PropertyPath")]
        pub property_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourcePath")]
        pub resource_path: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateConditions"
        )]
        pub subordinate_conditions:
            Option<Vec<crate::policy::v1_0_0::ConditionSubordinateConditions>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::policy::v1_0_0::ConditionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConditionOperator {
        V010000(crate::policy::v1_0_0::Operator),
        V000001(crate::policy::v1_0_0::ConditionOperatorN1),
    }
    impl Default for ConditionOperator {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConditionOperatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConditionSubordinateConditions {
        V010000(crate::policy::v1_0_0::Condition),
        V000001(crate::policy::v1_0_0::ConditionSubordinateConditionsN1),
    }
    impl Default for ConditionSubordinateConditions {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConditionSubordinateConditionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConditionType {
        V000001(crate::policy::v1_0_0::ConditionTypeN1),
        PolicyConditionType(crate::policy::ConditionType),
    }
    impl Default for ConditionType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConditionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageId")]
        pub message_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MessageOriginOfCondition"
        )]
        pub message_origin_of_condition: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPHeaderProperty {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Body")]
        pub body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Headers")]
        pub headers: Option<crate::policy::v1_0_0::HTTPHeaderProperty>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Operation")]
        pub operation: Option<crate::policy::v1_0_0::HTTPResponseOperation>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum HTTPResponseOperation {
        V010000(crate::policy::v1_0_0::HTTPVerb),
        V000001(crate::policy::v1_0_0::HTTPResponseOperationN1),
    }
    impl Default for HTTPResponseOperation {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HTTPResponseOperationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HTTPVerb {
        #[default]
        #[serde(rename = "DELETE")]
        DELETE,
        #[serde(rename = "GET")]
        GET,
        #[serde(rename = "PATCH")]
        PATCH,
        #[serde(rename = "POST")]
        POST,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Operator {
        #[default]
        #[serde(rename = "Equal")]
        Equal,
        #[serde(rename = "GreaterThan")]
        GreaterThan,
        #[serde(rename = "GreaterThanOrEqual")]
        GreaterThanOrEqual,
        #[serde(rename = "LessThan")]
        LessThan,
        #[serde(rename = "LessThanOrEqual")]
        LessThanOrEqual,
        #[serde(rename = "NotEqual")]
        NotEqual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Policy {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::policy::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::policy::v1_0_0::PolicyDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::policy::v1_0_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Predefined")]
        pub predefined: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Response")]
        pub response: Option<Vec<crate::policy::v1_0_0::PolicyResponse>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerCondition")]
        pub trigger_condition: Option<crate::policy::v1_0_0::PolicyTriggerCondition>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PolicyDescription {
        V000001(crate::policy::v1_0_0::PolicyDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PolicyDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PolicyDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PolicyResponse {
        V010000(crate::policy::v1_0_0::Response),
        V000001(crate::policy::v1_0_0::PolicyResponseN1),
    }
    impl Default for PolicyResponse {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PolicyResponseN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PolicyTriggerCondition {
        V010000(crate::policy::v1_0_0::Condition),
        V000001(crate::policy::v1_0_0::PolicyTriggerConditionN1),
    }
    impl Default for PolicyTriggerCondition {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PolicyTriggerConditionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Response {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DelaySeconds")]
        pub delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Event")]
        pub event: Option<crate::policy::v1_0_0::ResponseEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTP")]
        pub http: Option<crate::policy::v1_0_0::ResponseHTTP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::policy::v1_0_0::ResponseResetType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponseAction")]
        pub response_action: Option<crate::policy::v1_0_0::ResponseResponseAction>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetURI")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResponseEvent {
        V010000(crate::policy::v1_0_0::EventResponse),
        V000001(crate::policy::v1_0_0::ResponseEventN1),
    }
    impl Default for ResponseEvent {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResponseEventN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResponseHTTP {
        V010000(crate::policy::v1_0_0::HTTPResponse),
        V000001(crate::policy::v1_0_0::ResponseHTTPN1),
    }
    impl Default for ResponseHTTP {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResponseHTTPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResponseResetType {
        V000001(crate::policy::v1_0_0::ResponseResetTypeN1),
        ResourceResetType(crate::resource::ResetType),
    }
    impl Default for ResponseResetType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResponseResetTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResponseResponseAction {
        V000001(crate::policy::v1_0_0::ResponseResponseActionN1),
        PolicyResponseAction(crate::policy::ResponseAction),
    }
    impl Default for ResponseResponseAction {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResponseResponseActionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
