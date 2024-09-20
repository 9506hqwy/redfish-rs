pub type RegisteredClient = crate::registered_client::v1_1_2::RegisteredClient;
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::registered_client::v1_1_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ClientType {
        #[default]
        #[serde(rename = "Configure")]
        Configure,
        #[serde(rename = "Monitor")]
        Monitor,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagedResource {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludesSubordinates"
        )]
        pub includes_subordinates: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedResourceURI")]
        pub managed_resource_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PreferExclusive")]
        pub prefer_exclusive: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegisteredClient {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::registered_client::v1_1_2::Actions>,
        #[serde(rename = "ClientType")]
        pub client_type: crate::registered_client::v1_1_2::ClientType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientURI")]
        pub client_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedDate")]
        pub created_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::registered_client::v1_1_2::RegisteredClientDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpirationDate")]
        pub expiration_date: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedResources")]
        pub managed_resources:
            Option<Vec<crate::registered_client::v1_1_2::RegisteredClientManagedResources>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubContext")]
        pub sub_context: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RegisteredClientDescription {
        V000001(crate::registered_client::v1_1_2::RegisteredClientDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for RegisteredClientDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RegisteredClientDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RegisteredClientManagedResources {
        V010102(crate::registered_client::v1_1_2::ManagedResource),
        V000001(crate::registered_client::v1_1_2::RegisteredClientManagedResourcesN1),
    }
    impl Default for RegisteredClientManagedResources {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RegisteredClientManagedResourcesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
