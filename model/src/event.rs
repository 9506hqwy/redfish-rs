use serde::{Deserialize, Serialize};
pub type Event = crate::event::v1_10_0::Event;
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
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event::v1_10_0::OemActions>,
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
        pub actions: Option<crate::event::v1_10_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Events")]
        pub events: Vec<crate::event::v1_10_0::EventRecord>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecord {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event::v1_10_0::EventRecordActions>,
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
        pub cper: Option<crate::event::v1_10_0::CPER>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticData")]
        pub diagnostic_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type: Option<crate::event::v1_10_0::DiagnosticDataTypes>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecordActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event::v1_10_0::EventRecordOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRecordOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
