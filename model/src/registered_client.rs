pub type RegisteredClient = crate::registered_client::v1_1_1::RegisteredClient;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::registered_client::v1_1_0::OemActions>,
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
        pub actions: Option<crate::registered_client::v1_1_0::Actions>,
        #[serde(rename = "ClientType")]
        pub client_type: crate::registered_client::v1_1_0::ClientType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientURI")]
        pub client_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedDate")]
        pub created_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpirationDate")]
        pub expiration_date: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedResources")]
        pub managed_resources: Option<Vec<crate::registered_client::v1_1_0::ManagedResource>>,
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
}
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::registered_client::v1_1_1::OemActions>,
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
        pub actions: Option<crate::registered_client::v1_1_1::Actions>,
        #[serde(rename = "ClientType")]
        pub client_type: crate::registered_client::v1_1_1::ClientType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientURI")]
        pub client_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Context")]
        pub context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedDate")]
        pub created_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpirationDate")]
        pub expiration_date: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedResources")]
        pub managed_resources: Option<Vec<crate::registered_client::v1_1_1::ManagedResource>>,
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
}
