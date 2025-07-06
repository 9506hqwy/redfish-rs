pub type UpdateServiceCapabilities =
    crate::update_service_capabilities::v1_0_0::UpdateServiceCapabilities;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service_capabilities::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateServiceCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service_capabilities::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableStaging")]
        pub allowable_staging: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableTargets")]
        pub allowable_targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<
            crate::update_service_capabilities::v1_0_0::UpdateServiceCapabilitiesDescription,
        >,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum UpdateServiceCapabilitiesDescription {
        V000001(crate::update_service_capabilities::v1_0_0::UpdateServiceCapabilitiesDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for UpdateServiceCapabilitiesDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UpdateServiceCapabilitiesDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
