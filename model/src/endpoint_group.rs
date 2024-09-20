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
pub type EndpointGroup = crate::endpoint_group::v1_3_4::EndpointGroup;
pub mod v1_3_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint_group::v1_3_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EndpointGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::v1_3_4::EndpointGroupAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint_group::v1_3_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::endpoint_group::v1_3_4::EndpointGroupDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupType")]
        pub group_type: Option<crate::endpoint_group::v1_3_4::EndpointGroupGroupType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint_group::v1_3_4::Links>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EndpointGroupAccessState {
        V000001(crate::endpoint_group::v1_3_4::EndpointGroupAccessStateN1),
        EndpointGroupAccessState(crate::endpoint_group::AccessState),
    }
    impl Default for EndpointGroupAccessState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EndpointGroupAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EndpointGroupDescription {
        V000001(crate::endpoint_group::v1_3_4::EndpointGroupDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for EndpointGroupDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EndpointGroupDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EndpointGroupGroupType {
        V010304(crate::endpoint_group::v1_3_4::GroupType),
        V000001(crate::endpoint_group::v1_3_4::EndpointGroupGroupTypeN1),
    }
    impl Default for EndpointGroupGroupType {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EndpointGroupGroupTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
