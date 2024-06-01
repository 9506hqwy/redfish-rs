pub type OutboundConnection = crate::outbound_connection::v1_0_1::OutboundConnection;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outbound_connection::v1_0_1::OemActions>,
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
        pub session: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutboundConnection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outbound_connection::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::outbound_connection::v1_0_1::AuthenticationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionEnabled")]
        pub connection_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointURI")]
        pub endpoint_uri: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outbound_connection::v1_0_1::Links>,
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
            Option<crate::outbound_connection::v1_0_1::HTTPHeaderProperty>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetryPolicy")]
        pub retry_policy: Option<crate::outbound_connection::v1_0_1::RetryPolicyType>,
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
            Option<crate::outbound_connection::v1_0_1::OutboundConnectionRetryPolicyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetryCount")]
        pub retry_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RetryIntervalMinutes"
        )]
        pub retry_interval_minutes: Option<i64>,
    }
}
