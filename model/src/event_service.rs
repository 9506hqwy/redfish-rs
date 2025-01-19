pub type EventService = crate::event_service::v1_10_3::EventService;
pub mod v1_10_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_10_2::SubmitTestEvent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.TestEventSubscription"
        )]
        pub event_service_test_event_subscription:
            Option<crate::event_service::v1_10_2::TestEventSubscription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_10_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_10_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryAttempts"
        )]
        pub delivery_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryIntervalSeconds"
        )]
        pub delivery_retry_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::event_service::v1_10_2::EventServiceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types:
            Option<Vec<crate::event_service::v1_10_2::EventServiceEventFormatTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcludeMessageId")]
        pub exclude_message_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExcludeRegistryPrefix"
        )]
        pub exclude_registry_prefix: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludeOriginOfConditionSupported"
        )]
        pub include_origin_of_condition_supported: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefixes")]
        pub registry_prefixes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceTypes")]
        pub resource_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerSentEventUri")]
        pub server_sent_event_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severities")]
        pub severities: Option<Vec<crate::event_service::v1_10_2::EventServiceSeverities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_10_2::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_10_2::SSEFilterPropertiesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResourcesSupported"
        )]
        pub subordinate_resources_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subscriptions")]
        pub subscriptions: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceDescription {
        V000001(crate::event_service::v1_10_2::EventServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EventServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceEventFormatTypes {
        V000001(crate::event_service::v1_10_2::EventServiceEventFormatTypesN1),
        EventDestinationEventFormatType(crate::event_destination::EventFormatType),
    }
    impl Default for EventServiceEventFormatTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceEventFormatTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceSeverities {
        V000001(crate::event_service::v1_10_2::EventServiceSeveritiesN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for EventServiceSeverities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceSeveritiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_10_2::SMTPAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_10_2::SMTPConnectionProtocolAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SMTPAuthentication {
        V011002(crate::event_service::v1_10_2::SMTPAuthenticationMethods),
        V000001(crate::event_service::v1_10_2::SMTPAuthenticationN1),
    }
    impl Default for SMTPAuthentication {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPAuthenticationMethods {
        #[default]
        #[serde(rename = "AutoDetect")]
        AutoDetect,
        #[serde(rename = "CRAM_MD5")]
        CRAMMD5,
        #[serde(rename = "Login")]
        Login,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Plain")]
        Plain,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPConnectionProtocol {
        #[default]
        #[serde(rename = "AutoDetect")]
        AutoDetect,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "StartTLS")]
        StartTLS,
        #[serde(rename = "TLS_SSL")]
        TLSSSL,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SMTPConnectionProtocolAnony {
        V011002(crate::event_service::v1_10_2::SMTPConnectionProtocol),
        V000001(crate::event_service::v1_10_2::SMTPConnectionProtocolN1),
    }
    impl Default for SMTPConnectionProtocolAnony {
        fn default() -> Self {
            Self::V011002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPConnectionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SSEFilterPropertiesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatType")]
        pub event_format_type: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageId")]
        pub message_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinition"
        )]
        pub metric_report_definition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginResource")]
        pub origin_resource: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefix")]
        pub registry_prefix: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceType")]
        pub resource_type: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResources"
        )]
        pub subordinate_resources: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEvent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEventRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventGroupId")]
        pub event_group_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventId")]
        pub event_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTimestamp")]
        pub event_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<crate::event::EventType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(rename = "MessageId")]
        pub message_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageSeverity")]
        pub message_severity: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TestEventSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TestEventSubscriptionRequestBody {}
}
pub mod v1_10_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_10_3::SubmitTestEvent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.TestEventSubscription"
        )]
        pub event_service_test_event_subscription:
            Option<crate::event_service::v1_10_3::TestEventSubscription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_10_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_10_3::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryAttempts"
        )]
        pub delivery_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryIntervalSeconds"
        )]
        pub delivery_retry_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::event_service::v1_10_3::EventServiceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types:
            Option<Vec<crate::event_service::v1_10_3::EventServiceEventFormatTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcludeMessageId")]
        pub exclude_message_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExcludeRegistryPrefix"
        )]
        pub exclude_registry_prefix: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludeOriginOfConditionSupported"
        )]
        pub include_origin_of_condition_supported: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefixes")]
        pub registry_prefixes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceTypes")]
        pub resource_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerSentEventUri")]
        pub server_sent_event_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severities")]
        pub severities: Option<Vec<crate::event_service::v1_10_3::EventServiceSeverities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_10_3::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_10_3::SSEFilterPropertiesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResourcesSupported"
        )]
        pub subordinate_resources_supported: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subscriptions")]
        pub subscriptions: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceDescription {
        V000001(crate::event_service::v1_10_3::EventServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EventServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceEventFormatTypes {
        V000001(crate::event_service::v1_10_3::EventServiceEventFormatTypesN1),
        EventDestinationEventFormatType(crate::event_destination::EventFormatType),
    }
    impl Default for EventServiceEventFormatTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceEventFormatTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventServiceSeverities {
        V000001(crate::event_service::v1_10_3::EventServiceSeveritiesN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for EventServiceSeverities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventServiceSeveritiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_10_3::SMTPAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_10_3::SMTPConnectionProtocolAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SMTPAuthentication {
        V011003(crate::event_service::v1_10_3::SMTPAuthenticationMethods),
        V000001(crate::event_service::v1_10_3::SMTPAuthenticationN1),
    }
    impl Default for SMTPAuthentication {
        fn default() -> Self {
            Self::V011003(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPAuthenticationMethods {
        #[default]
        #[serde(rename = "AutoDetect")]
        AutoDetect,
        #[serde(rename = "CRAM_MD5")]
        CRAMMD5,
        #[serde(rename = "Login")]
        Login,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Plain")]
        Plain,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPConnectionProtocol {
        #[default]
        #[serde(rename = "AutoDetect")]
        AutoDetect,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "StartTLS")]
        StartTLS,
        #[serde(rename = "TLS_SSL")]
        TLSSSL,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SMTPConnectionProtocolAnony {
        V011003(crate::event_service::v1_10_3::SMTPConnectionProtocol),
        V000001(crate::event_service::v1_10_3::SMTPConnectionProtocolN1),
    }
    impl Default for SMTPConnectionProtocolAnony {
        fn default() -> Self {
            Self::V011003(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SMTPConnectionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SSEFilterPropertiesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatType")]
        pub event_format_type: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageId")]
        pub message_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MetricReportDefinition"
        )]
        pub metric_report_definition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginResource")]
        pub origin_resource: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefix")]
        pub registry_prefix: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceType")]
        pub resource_type: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResources"
        )]
        pub subordinate_resources: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEvent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEventRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventGroupId")]
        pub event_group_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventId")]
        pub event_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTimestamp")]
        pub event_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<crate::event::EventType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(rename = "MessageId")]
        pub message_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageSeverity")]
        pub message_severity: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TestEventSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TestEventSubscriptionRequestBody {}
}
