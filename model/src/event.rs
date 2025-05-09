use serde::{Deserialize, Serialize};
pub type Event = crate::event::v1_11_1::Event;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EventType {
    #[default]
    #[serde(rename = "Alert")]
    Alert,
    #[serde(rename = "MetricReport")]
    MetricReport,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "ResourceAdded")]
    ResourceAdded,
    #[serde(rename = "ResourceRemoved")]
    ResourceRemoved,
    #[serde(rename = "ResourceUpdated")]
    ResourceUpdated,
    #[serde(rename = "StatusChange")]
    StatusChange,
}
pub mod v1_11_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event::v1_11_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CPER {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NotificationType")]
        pub notification_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SectionType")]
        pub section_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DiagnosticDataTypes {
        #[default]
        #[serde(rename = "CPER")]
        CPER,
        #[serde(rename = "CPERSection")]
        CPERSection,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "PreOS")]
        PreOS,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Event {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event::v1_11_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::event::v1_11_1::EventDescription>,
        #[serde(rename = "Events")]
        pub events: Vec<crate::event::v1_11_1::EventRecord>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Events@odata.count")]
        pub events_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(rename = "@odata.type")]
        pub odata_type: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDescription {
        V000001(crate::event::v1_11_1::EventDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EventDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecord {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event::v1_11_1::EventRecordActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalDataSizeBytes"
        )]
        pub additional_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalDataURI")]
        pub additional_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPER")]
        pub cper: Option<crate::event::v1_11_1::CPER>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticData")]
        pub diagnostic_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type: Option<crate::event::v1_11_1::EventRecordDiagnosticDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventGroupId")]
        pub event_group_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventId")]
        pub event_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTimestamp")]
        pub event_timestamp: Option<String>,
        #[serde(rename = "EventType")]
        pub event_type: crate::event::EventType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntry")]
        pub log_entry: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(rename = "MessageId")]
        pub message_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageSeverity")]
        pub message_severity: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
        pub resolution: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
        pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpecificEventExistsInGroup"
        )]
        pub specific_event_exists_in_group: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserAuthenticationSource"
        )]
        pub user_authentication_source: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecordActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event::v1_11_1::EventRecordOemActions>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventRecordDiagnosticDataType {
        V011101(crate::event::v1_11_1::DiagnosticDataTypes),
        V000001(crate::event::v1_11_1::EventRecordDiagnosticDataTypeN1),
    }
    impl Default for EventRecordDiagnosticDataType {
        fn default() -> Self {
            Self::V011101(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventRecordDiagnosticDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecordOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
