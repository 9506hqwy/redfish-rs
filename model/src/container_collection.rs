use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerCollection {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<crate::container_collection::ContainerCollectionDescription>,
    #[serde(rename = "Members")]
    pub members: Vec<crate::odata_v4::IdRef>,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "Members@odata.nextLink"
    )]
    pub members_odata_next_link: Option<String>,
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
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ContainerCollectionDescription {
    V000001(crate::container_collection::ContainerCollectionDescriptionN1),
    ResourceDescription(String),
}
impl Default for ContainerCollectionDescription {
    fn default() -> Self {
        Self::V000001(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ContainerCollectionDescriptionN1 {
    #[default]
    #[serde(rename = "null")]
    Null,
}
