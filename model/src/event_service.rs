use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EventService {
    V011000(crate::event_service::v1_10_0::EventService),
    V010901(crate::event_service::v1_9_1::EventService),
    V010802(crate::event_service::v1_8_2::EventService),
    V010705(crate::event_service::v1_7_5::EventService),
    V010606(crate::event_service::v1_6_6::EventService),
    V010507(crate::event_service::v1_5_7::EventService),
    V010407(crate::event_service::v1_4_7::EventService),
    V010307(crate::event_service::v1_3_7::EventService),
    V010208(crate::event_service::v1_2_8::EventService),
    V010108(crate::event_service::v1_1_8::EventService),
    V010015(crate::event_service::v1_0_15::EventService),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_15 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_0_15::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_0_15::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_0_15::Actions>,
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
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subscriptions")]
        pub subscriptions: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEvent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEventRequestBody {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_1_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_1_8::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_1_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_1_8::Actions>,
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
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerSentEventUri")]
        pub server_sent_event_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subscriptions")]
        pub subscriptions: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEvent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitTestEventRequestBody {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_2_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_2_8::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_2_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_2_8::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_2_8::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_3_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_3_7::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_3_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_3_7::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_3_7::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_4_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_4_7::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_4_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_4_7::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_4_7::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_5_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_5_7::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_5_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_5_7::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_5_7::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_5_7::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_5_7::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_5_7::SMTPConnectionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_6_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_6_6::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_6_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_6_6::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_6_6::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_6_6::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_6_6::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_6_6::SMTPConnectionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_7_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_7_5::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_7_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_7_5::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EventTypesForSubscription"
        )]
        pub event_types_for_subscription: Option<Vec<crate::event::EventType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_7_5::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_7_5::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_7_5::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_7_5::SMTPConnectionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_8_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_8_2::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_8_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_8_2::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_8_2::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_8_2::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_8_2::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_8_2::SMTPConnectionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FromAddress")]
        pub from_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerAddress")]
        pub server_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_9_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_9_1::SubmitTestEvent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_9_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_9_1::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
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
        pub severities: Option<Vec<crate::resource::Health>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_9_1::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_9_1::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_9_1::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_9_1::SMTPConnectionProtocol>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.SubmitTestEvent"
        )]
        pub event_service_submit_test_event: Option<crate::event_service::v1_10_0::SubmitTestEvent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventService.TestEventSubscription"
        )]
        pub event_service_test_event_subscription:
            Option<crate::event_service::v1_10_0::TestEventSubscription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_service::v1_10_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_service::v1_10_0::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatTypes")]
        pub event_format_types: Option<Vec<crate::event_destination::EventFormatType>>,
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
        pub severities: Option<Vec<crate::resource::Health>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SMTP")]
        pub smtp: Option<crate::event_service::v1_10_0::SMTP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SSEFilterPropertiesSupported"
        )]
        pub sse_filter_properties_supported:
            Option<crate::event_service::v1_10_0::SSEFilterPropertiesSupported>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SMTP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::event_service::v1_10_0::SMTPAuthenticationMethods>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionProtocol")]
        pub connection_protocol: Option<crate::event_service::v1_10_0::SMTPConnectionProtocol>,
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
