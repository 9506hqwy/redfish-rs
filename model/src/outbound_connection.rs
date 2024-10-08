pub type OutboundConnection = crate::outbound_connection::v1_0_2::OutboundConnection;
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outbound_connection::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationType {
        #[default]
        #[serde(rename = "JWT")]
        JWT,
        #[serde(rename = "MTLS")]
        MTLS,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPHeaderProperty {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Session")]
        pub session: Option<crate::outbound_connection::v1_0_2::LinksSession>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksSession {
        V000001(crate::outbound_connection::v1_0_2::LinksSessionN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksSession {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksSessionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutboundConnection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outbound_connection::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication:
            Option<crate::outbound_connection::v1_0_2::OutboundConnectionAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionEnabled")]
        pub connection_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::outbound_connection::v1_0_2::OutboundConnectionDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointURI")]
        pub endpoint_uri: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outbound_connection::v1_0_2::Links>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PreUpgradeHTTPHeaders"
        )]
        pub pre_upgrade_http_headers:
            Option<crate::outbound_connection::v1_0_2::HTTPHeaderProperty>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetryPolicy")]
        pub retry_policy: Option<crate::outbound_connection::v1_0_2::RetryPolicyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WebSocketPingIntervalMinutes"
        )]
        pub web_socket_ping_interval_minutes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutboundConnectionAuthentication {
        V010002(crate::outbound_connection::v1_0_2::AuthenticationType),
        V000001(crate::outbound_connection::v1_0_2::OutboundConnectionAuthenticationN1),
    }
    impl Default for OutboundConnectionAuthentication {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutboundConnectionAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutboundConnectionDescription {
        V000001(crate::outbound_connection::v1_0_2::OutboundConnectionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for OutboundConnectionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutboundConnectionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutboundConnectionRetryPolicyType {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RetryCount")]
        RetryCount,
        #[serde(rename = "RetryForever")]
        RetryForever,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RetryPolicyType {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionRetryPolicy"
        )]
        pub connection_retry_policy:
            Option<crate::outbound_connection::v1_0_2::RetryPolicyTypeConnectionRetryPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetryCount")]
        pub retry_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RetryIntervalMinutes"
        )]
        pub retry_interval_minutes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RetryPolicyTypeConnectionRetryPolicy {
        V010002(crate::outbound_connection::v1_0_2::OutboundConnectionRetryPolicyType),
        V000001(crate::outbound_connection::v1_0_2::RetryPolicyTypeConnectionRetryPolicyN1),
    }
    impl Default for RetryPolicyTypeConnectionRetryPolicy {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RetryPolicyTypeConnectionRetryPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
