use serde::{Deserialize, Serialize};
pub type EventDestination = crate::event_destination::v1_15_1::EventDestination;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EventFormatType {
    #[default]
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "MetricReport")]
    MetricReport,
}
pub mod v1_14_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventDestination.ResumeSubscription"
        )]
        pub event_destination_resume_subscription:
            Option<crate::event_destination::v1_14_1::ResumeSubscription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventDestination.SuspendSubscription"
        )]
        pub event_destination_suspend_subscription:
            Option<crate::event_destination::v1_14_1::SuspendSubscription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_destination::v1_14_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeliveryRetryPolicy {
        #[default]
        #[serde(rename = "RetryForever")]
        RetryForever,
        #[serde(rename = "RetryForeverWithBackoff")]
        RetryForeverWithBackoff,
        #[serde(rename = "SuspendRetries")]
        SuspendRetries,
        #[serde(rename = "TerminateAfterRetries")]
        TerminateAfterRetries,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventDestination {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_destination::v1_14_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Context")]
        pub context: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryPolicy"
        )]
        pub delivery_retry_policy:
            Option<crate::event_destination::v1_14_1::EventDestinationDeliveryRetryPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::event_destination::v1_14_1::EventDestinationDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Destination")]
        pub destination: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatType")]
        pub event_format_type:
            Option<crate::event_destination::v1_14_1::EventDestinationEventFormatType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTypes")]
        pub event_types: Option<Vec<crate::event::EventType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcludeMessageIds")]
        pub exclude_message_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExcludeRegistryPrefixes"
        )]
        pub exclude_registry_prefixes: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HeartbeatIntervalMinutes"
        )]
        pub heartbeat_interval_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<crate::event_destination::v1_14_1::HttpHeaderProperty>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludeOriginOfCondition"
        )]
        pub include_origin_of_condition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageIds")]
        pub message_ids: Option<Vec<String>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMProtocol")]
        pub oem_protocol: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMSubscriptionType"
        )]
        pub oem_subscription_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginResources")]
        pub origin_resources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OriginResources@odata.count"
        )]
        pub origin_resources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::event_destination::v1_14_1::EventDestinationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefixes")]
        pub registry_prefixes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceTypes")]
        pub resource_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SendHeartbeat")]
        pub send_heartbeat: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severities")]
        pub severities: Option<Vec<crate::event_destination::v1_14_1::EventDestinationSeverities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::event_destination::v1_14_1::SNMPSettings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResources"
        )]
        pub subordinate_resources: Option<bool>,
        #[serde(rename = "SubscriptionType")]
        pub subscription_type: crate::event_destination::v1_14_1::EventDestinationSubscriptionType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyslogFilters")]
        pub syslog_filters:
            Option<Vec<crate::event_destination::v1_14_1::EventDestinationSyslogFilters>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationDeliveryRetryPolicy {
        V011401(crate::event_destination::v1_14_1::DeliveryRetryPolicy),
        V000001(crate::event_destination::v1_14_1::EventDestinationDeliveryRetryPolicyN1),
    }
    impl Default for EventDestinationDeliveryRetryPolicy {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationDeliveryRetryPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationDescription {
        V000001(crate::event_destination::v1_14_1::EventDestinationDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EventDestinationDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationEventFormatType {
        V000001(crate::event_destination::v1_14_1::EventDestinationEventFormatTypeN1),
        EventDestinationEventFormatType(crate::event_destination::EventFormatType),
    }
    impl Default for EventDestinationEventFormatType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationEventFormatTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationProtocol {
        #[default]
        #[serde(rename = "Kafka")]
        Kafka,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Redfish")]
        Redfish,
        #[serde(rename = "SMTP")]
        SMTP,
        #[serde(rename = "SNMPv1")]
        SNMPv1,
        #[serde(rename = "SNMPv2c")]
        SNMPv2c,
        #[serde(rename = "SNMPv3")]
        SNMPv3,
        #[serde(rename = "SyslogRELP")]
        SyslogRELP,
        #[serde(rename = "SyslogTCP")]
        SyslogTCP,
        #[serde(rename = "SyslogTLS")]
        SyslogTLS,
        #[serde(rename = "SyslogUDP")]
        SyslogUDP,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSeverities {
        V000001(crate::event_destination::v1_14_1::EventDestinationSeveritiesN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for EventDestinationSeverities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSeveritiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSubscriptionType {
        V011401(crate::event_destination::v1_14_1::SubscriptionType),
        V000001(crate::event_destination::v1_14_1::EventDestinationSubscriptionTypeN1),
    }
    impl Default for EventDestinationSubscriptionType {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSubscriptionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSyslogFilters {
        V011401(crate::event_destination::v1_14_1::SyslogFilter),
        V000001(crate::event_destination::v1_14_1::EventDestinationSyslogFiltersN1),
    }
    impl Default for EventDestinationSyslogFilters {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSyslogFiltersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpHeaderProperty {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeSubscriptionRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliverBufferedEventDuration"
        )]
        pub deliver_buffered_event_duration: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
        #[serde(rename = "CommunityString")]
        CommunityString,
        #[serde(rename = "HMAC128_SHA224")]
        HMAC128SHA224,
        #[serde(rename = "HMAC192_SHA256")]
        HMAC192SHA256,
        #[serde(rename = "HMAC256_SHA384")]
        HMAC256SHA384,
        #[serde(rename = "HMAC384_SHA512")]
        HMAC384SHA512,
        #[serde(rename = "HMAC_MD5")]
        HMACMD5,
        #[serde(rename = "HMAC_SHA96")]
        HMACSHA96,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPEncryptionProtocols {
        #[default]
        #[serde(rename = "CBC_DES")]
        CBCDES,
        #[serde(rename = "CFB128_AES128")]
        CFB128AES128,
        #[serde(rename = "CFB128_AES192")]
        CFB128AES192,
        #[serde(rename = "CFB128_AES256")]
        CFB128AES256,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SNMPSettings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationKey")]
        pub authentication_key: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationKeySet"
        )]
        pub authentication_key_set: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationProtocol"
        )]
        pub authentication_protocol:
            Option<crate::event_destination::v1_14_1::SNMPSettingsAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::event_destination::v1_14_1::SNMPSettingsEncryptionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrapCommunity")]
        pub trap_community: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsAuthenticationProtocol {
        V011401(crate::event_destination::v1_14_1::SNMPAuthenticationProtocols),
        V000001(crate::event_destination::v1_14_1::SNMPSettingsAuthenticationProtocolN1),
    }
    impl Default for SNMPSettingsAuthenticationProtocol {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsEncryptionProtocol {
        V011401(crate::event_destination::v1_14_1::SNMPEncryptionProtocols),
        V000001(crate::event_destination::v1_14_1::SNMPSettingsEncryptionProtocolN1),
    }
    impl Default for SNMPSettingsEncryptionProtocol {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SubscriptionType {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
        #[serde(rename = "SNMPInform")]
        SNMPInform,
        #[serde(rename = "SNMPTrap")]
        SNMPTrap,
        #[serde(rename = "SSE")]
        SSE,
        #[serde(rename = "Syslog")]
        Syslog,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendSubscriptionRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFacility {
        #[default]
        #[serde(rename = "Auth")]
        Auth,
        #[serde(rename = "Authpriv")]
        Authpriv,
        #[serde(rename = "Console")]
        Console,
        #[serde(rename = "Cron")]
        Cron,
        #[serde(rename = "Daemon")]
        Daemon,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "Kern")]
        Kern,
        #[serde(rename = "LPR")]
        LPR,
        #[serde(rename = "Local0")]
        Local0,
        #[serde(rename = "Local1")]
        Local1,
        #[serde(rename = "Local2")]
        Local2,
        #[serde(rename = "Local3")]
        Local3,
        #[serde(rename = "Local4")]
        Local4,
        #[serde(rename = "Local5")]
        Local5,
        #[serde(rename = "Local6")]
        Local6,
        #[serde(rename = "Local7")]
        Local7,
        #[serde(rename = "Mail")]
        Mail,
        #[serde(rename = "NTP")]
        NTP,
        #[serde(rename = "News")]
        News,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "SolarisCron")]
        SolarisCron,
        #[serde(rename = "Syslog")]
        Syslog,
        #[serde(rename = "UUCP")]
        UUCP,
        #[serde(rename = "User")]
        User,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SyslogFilter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogFacilities")]
        pub log_facilities:
            Option<Vec<crate::event_destination::v1_14_1::SyslogFilterLogFacilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestSeverity")]
        pub lowest_severity: Option<crate::event_destination::v1_14_1::SyslogFilterLowestSeverity>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLogFacilities {
        V011401(crate::event_destination::v1_14_1::SyslogFacility),
        V000001(crate::event_destination::v1_14_1::SyslogFilterLogFacilitiesN1),
    }
    impl Default for SyslogFilterLogFacilities {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLogFacilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLowestSeverity {
        V011401(crate::event_destination::v1_14_1::SyslogSeverity),
        V000001(crate::event_destination::v1_14_1::SyslogFilterLowestSeverityN1),
    }
    impl Default for SyslogFilterLowestSeverity {
        fn default() -> Self {
            Self::V011401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLowestSeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogSeverity {
        #[default]
        #[serde(rename = "Alert")]
        Alert,
        #[serde(rename = "All")]
        All,
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "Debug")]
        Debug,
        #[serde(rename = "Emergency")]
        Emergency,
        #[serde(rename = "Error")]
        Error,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Notice")]
        Notice,
        #[serde(rename = "Warning")]
        Warning,
    }
}
pub mod v1_15_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventDestination.ResumeSubscription"
        )]
        pub event_destination_resume_subscription:
            Option<crate::event_destination::v1_15_1::ResumeSubscription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#EventDestination.SuspendSubscription"
        )]
        pub event_destination_suspend_subscription:
            Option<crate::event_destination::v1_15_1::SuspendSubscription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::event_destination::v1_15_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeliveryRetryPolicy {
        #[default]
        #[serde(rename = "RetryForever")]
        RetryForever,
        #[serde(rename = "RetryForeverWithBackoff")]
        RetryForeverWithBackoff,
        #[serde(rename = "SuspendRetries")]
        SuspendRetries,
        #[serde(rename = "TerminateAfterRetries")]
        TerminateAfterRetries,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventDestination {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::event_destination::v1_15_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BackupDestinations")]
        pub backup_destinations: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Context")]
        pub context: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliveryRetryPolicy"
        )]
        pub delivery_retry_policy:
            Option<crate::event_destination::v1_15_1::EventDestinationDeliveryRetryPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::event_destination::v1_15_1::EventDestinationDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Destination")]
        pub destination: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventFormatType")]
        pub event_format_type:
            Option<crate::event_destination::v1_15_1::EventDestinationEventFormatType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTypes")]
        pub event_types: Option<Vec<crate::event::EventType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcludeMessageIds")]
        pub exclude_message_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExcludeRegistryPrefixes"
        )]
        pub exclude_registry_prefixes: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HeartbeatIntervalMinutes"
        )]
        pub heartbeat_interval_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<crate::event_destination::v1_15_1::HttpHeaderProperty>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludeOriginOfCondition"
        )]
        pub include_origin_of_condition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageIds")]
        pub message_ids: Option<Vec<String>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMProtocol")]
        pub oem_protocol: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMSubscriptionType"
        )]
        pub oem_subscription_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginResources")]
        pub origin_resources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OriginResources@odata.count"
        )]
        pub origin_resources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Protocol")]
        pub protocol: Option<crate::event_destination::v1_15_1::EventDestinationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegistryPrefixes")]
        pub registry_prefixes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceTypes")]
        pub resource_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SendHeartbeat")]
        pub send_heartbeat: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severities")]
        pub severities: Option<Vec<crate::event_destination::v1_15_1::EventDestinationSeverities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::event_destination::v1_15_1::SNMPSettings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubordinateResources"
        )]
        pub subordinate_resources: Option<bool>,
        #[serde(rename = "SubscriptionType")]
        pub subscription_type: crate::event_destination::v1_15_1::EventDestinationSubscriptionType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyslogFilters")]
        pub syslog_filters:
            Option<Vec<crate::event_destination::v1_15_1::EventDestinationSyslogFilters>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationDeliveryRetryPolicy {
        V011501(crate::event_destination::v1_15_1::DeliveryRetryPolicy),
        V000001(crate::event_destination::v1_15_1::EventDestinationDeliveryRetryPolicyN1),
    }
    impl Default for EventDestinationDeliveryRetryPolicy {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationDeliveryRetryPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationDescription {
        V000001(crate::event_destination::v1_15_1::EventDestinationDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EventDestinationDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationEventFormatType {
        V000001(crate::event_destination::v1_15_1::EventDestinationEventFormatTypeN1),
        EventDestinationEventFormatType(crate::event_destination::EventFormatType),
    }
    impl Default for EventDestinationEventFormatType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationEventFormatTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationProtocol {
        #[default]
        #[serde(rename = "Kafka")]
        Kafka,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Redfish")]
        Redfish,
        #[serde(rename = "SMTP")]
        SMTP,
        #[serde(rename = "SNMPv1")]
        SNMPv1,
        #[serde(rename = "SNMPv2c")]
        SNMPv2c,
        #[serde(rename = "SNMPv3")]
        SNMPv3,
        #[serde(rename = "SyslogRELP")]
        SyslogRELP,
        #[serde(rename = "SyslogTCP")]
        SyslogTCP,
        #[serde(rename = "SyslogTLS")]
        SyslogTLS,
        #[serde(rename = "SyslogUDP")]
        SyslogUDP,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSeverities {
        V000001(crate::event_destination::v1_15_1::EventDestinationSeveritiesN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for EventDestinationSeverities {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSeveritiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSubscriptionType {
        V011501(crate::event_destination::v1_15_1::SubscriptionType),
        V000001(crate::event_destination::v1_15_1::EventDestinationSubscriptionTypeN1),
    }
    impl Default for EventDestinationSubscriptionType {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSubscriptionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EventDestinationSyslogFilters {
        V011501(crate::event_destination::v1_15_1::SyslogFilter),
        V000001(crate::event_destination::v1_15_1::EventDestinationSyslogFiltersN1),
    }
    impl Default for EventDestinationSyslogFilters {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventDestinationSyslogFiltersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpHeaderProperty {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeSubscriptionRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeliverBufferedEventDuration"
        )]
        pub deliver_buffered_event_duration: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
        #[serde(rename = "CommunityString")]
        CommunityString,
        #[serde(rename = "HMAC128_SHA224")]
        HMAC128SHA224,
        #[serde(rename = "HMAC192_SHA256")]
        HMAC192SHA256,
        #[serde(rename = "HMAC256_SHA384")]
        HMAC256SHA384,
        #[serde(rename = "HMAC384_SHA512")]
        HMAC384SHA512,
        #[serde(rename = "HMAC_MD5")]
        HMACMD5,
        #[serde(rename = "HMAC_SHA96")]
        HMACSHA96,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPEncryptionProtocols {
        #[default]
        #[serde(rename = "CBC_DES")]
        CBCDES,
        #[serde(rename = "CFB128_AES128")]
        CFB128AES128,
        #[serde(rename = "CFB128_AES192")]
        CFB128AES192,
        #[serde(rename = "CFB128_AES256")]
        CFB128AES256,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SNMPSettings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationKey")]
        pub authentication_key: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationKeySet"
        )]
        pub authentication_key_set: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationProtocol"
        )]
        pub authentication_protocol:
            Option<crate::event_destination::v1_15_1::SNMPSettingsAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::event_destination::v1_15_1::SNMPSettingsEncryptionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrapCommunity")]
        pub trap_community: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsAuthenticationProtocol {
        V011501(crate::event_destination::v1_15_1::SNMPAuthenticationProtocols),
        V000001(crate::event_destination::v1_15_1::SNMPSettingsAuthenticationProtocolN1),
    }
    impl Default for SNMPSettingsAuthenticationProtocol {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsEncryptionProtocol {
        V011501(crate::event_destination::v1_15_1::SNMPEncryptionProtocols),
        V000001(crate::event_destination::v1_15_1::SNMPSettingsEncryptionProtocolN1),
    }
    impl Default for SNMPSettingsEncryptionProtocol {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SubscriptionType {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishEvent")]
        RedfishEvent,
        #[serde(rename = "SNMPInform")]
        SNMPInform,
        #[serde(rename = "SNMPTrap")]
        SNMPTrap,
        #[serde(rename = "SSE")]
        SSE,
        #[serde(rename = "Syslog")]
        Syslog,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendSubscription {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendSubscriptionRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFacility {
        #[default]
        #[serde(rename = "Auth")]
        Auth,
        #[serde(rename = "Authpriv")]
        Authpriv,
        #[serde(rename = "Console")]
        Console,
        #[serde(rename = "Cron")]
        Cron,
        #[serde(rename = "Daemon")]
        Daemon,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "Kern")]
        Kern,
        #[serde(rename = "LPR")]
        LPR,
        #[serde(rename = "Local0")]
        Local0,
        #[serde(rename = "Local1")]
        Local1,
        #[serde(rename = "Local2")]
        Local2,
        #[serde(rename = "Local3")]
        Local3,
        #[serde(rename = "Local4")]
        Local4,
        #[serde(rename = "Local5")]
        Local5,
        #[serde(rename = "Local6")]
        Local6,
        #[serde(rename = "Local7")]
        Local7,
        #[serde(rename = "Mail")]
        Mail,
        #[serde(rename = "NTP")]
        NTP,
        #[serde(rename = "News")]
        News,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "SolarisCron")]
        SolarisCron,
        #[serde(rename = "Syslog")]
        Syslog,
        #[serde(rename = "UUCP")]
        UUCP,
        #[serde(rename = "User")]
        User,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SyslogFilter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogFacilities")]
        pub log_facilities:
            Option<Vec<crate::event_destination::v1_15_1::SyslogFilterLogFacilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestSeverity")]
        pub lowest_severity: Option<crate::event_destination::v1_15_1::SyslogFilterLowestSeverity>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLogFacilities {
        V011501(crate::event_destination::v1_15_1::SyslogFacility),
        V000001(crate::event_destination::v1_15_1::SyslogFilterLogFacilitiesN1),
    }
    impl Default for SyslogFilterLogFacilities {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLogFacilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLowestSeverity {
        V011501(crate::event_destination::v1_15_1::SyslogSeverity),
        V000001(crate::event_destination::v1_15_1::SyslogFilterLowestSeverityN1),
    }
    impl Default for SyslogFilterLowestSeverity {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLowestSeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogSeverity {
        #[default]
        #[serde(rename = "Alert")]
        Alert,
        #[serde(rename = "All")]
        All,
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "Debug")]
        Debug,
        #[serde(rename = "Emergency")]
        Emergency,
        #[serde(rename = "Error")]
        Error,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Notice")]
        Notice,
        #[serde(rename = "Warning")]
        Warning,
    }
}
