pub type Session = crate::session::v1_7_2::Session;
pub mod v1_7_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::session::v1_7_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutboundConnection")]
        pub outbound_connection: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Session {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::session::v1_7_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientOriginIPAddress"
        )]
        pub client_origin_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedTime")]
        pub created_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::session::v1_7_2::SessionDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::session::v1_7_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemSessionType")]
        pub oem_session_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionType")]
        pub session_type: Option<crate::session::v1_7_2::SessionSessionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SessionDescription {
        V000001(crate::session::v1_7_2::SessionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SessionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SessionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SessionSessionType {
        V010702(crate::session::v1_7_2::SessionTypes),
        V000001(crate::session::v1_7_2::SessionSessionTypeN1),
    }
    impl Default for SessionSessionType {
        fn default() -> Self {
            Self::V010702(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SessionSessionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SessionTypes {
        #[default]
        #[serde(rename = "HostConsole")]
        HostConsole,
        #[serde(rename = "IPMI")]
        IPMI,
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "ManagerConsole")]
        ManagerConsole,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OutboundConnection")]
        OutboundConnection,
        #[serde(rename = "Redfish")]
        Redfish,
        #[serde(rename = "VirtualMedia")]
        VirtualMedia,
        #[serde(rename = "WebUI")]
        WebUI,
    }
}
