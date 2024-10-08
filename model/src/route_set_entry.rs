pub type RouteSetEntry = crate::route_set_entry::v1_0_2::RouteSetEntry;
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::route_set_entry::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RouteSetEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::route_set_entry::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::route_set_entry::v1_0_2::RouteSetEntryDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EgressIdentifier")]
        pub egress_identifier: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HopCount")]
        pub hop_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Valid")]
        pub valid: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCAction")]
        pub vc_action: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RouteSetEntryDescription {
        V000001(crate::route_set_entry::v1_0_2::RouteSetEntryDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for RouteSetEntryDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RouteSetEntryDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
