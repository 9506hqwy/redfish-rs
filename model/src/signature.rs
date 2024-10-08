use serde::{Deserialize, Serialize};
pub type Signature = crate::signature::v1_0_3::Signature;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum SignatureTypeRegistry {
    #[default]
    #[serde(rename = "UEFI")]
    UEFI,
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::signature::v1_0_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Signature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::signature::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::signature::v1_0_3::SignatureDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignatureString")]
        pub signature_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignatureType")]
        pub signature_type: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SignatureTypeRegistry"
        )]
        pub signature_type_registry:
            Option<crate::signature::v1_0_3::SignatureSignatureTypeRegistry>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiSignatureOwner")]
        pub uefi_signature_owner: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SignatureDescription {
        V000001(crate::signature::v1_0_3::SignatureDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SignatureDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SignatureDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SignatureSignatureTypeRegistry {
        V000001(crate::signature::v1_0_3::SignatureSignatureTypeRegistryN1),
        SignatureSignatureTypeRegistry(crate::signature::SignatureTypeRegistry),
    }
    impl Default for SignatureSignatureTypeRegistry {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SignatureSignatureTypeRegistryN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
