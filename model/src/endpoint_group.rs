use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AccessState {
    #[default]
    #[serde(rename = "NonOptimized")]
    NonOptimized,
    #[serde(rename = "Optimized")]
    Optimized,
    #[serde(rename = "Standby")]
    Standby,
    #[serde(rename = "Transitioning")]
    Transitioning,
    #[serde(rename = "Unavailable")]
    Unavailable,
}
pub type EndpointGroup = crate::endpoint_group::v1_3_3::EndpointGroup;
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint_group::v1_3_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EndpointGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint_group::v1_3_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupType")]
        pub group_type: Option<crate::endpoint_group::v1_3_2::GroupType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint_group::v1_3_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Preferred")]
        pub preferred: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroupIdentifier"
        )]
        pub target_endpoint_group_identifier: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GroupType {
        #[default]
        #[serde(rename = "Client")]
        Client,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Server")]
        Server,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint_group::v1_3_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EndpointGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint_group::v1_3_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupType")]
        pub group_type: Option<crate::endpoint_group::v1_3_3::GroupType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint_group::v1_3_3::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Preferred")]
        pub preferred: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroupIdentifier"
        )]
        pub target_endpoint_group_identifier: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GroupType {
        #[default]
        #[serde(rename = "Client")]
        Client,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Server")]
        Server,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
