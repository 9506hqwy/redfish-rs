pub type Manifest = crate::manifest::v1_1_2::Manifest;
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Expand {
        #[default]
        #[serde(rename = "All")]
        All,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Relevant")]
        Relevant,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Manifest {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Expand")]
        pub expand: Option<crate::manifest::v1_1_2::ManifestExpand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Stanzas")]
        pub stanzas: Option<Vec<crate::manifest::v1_1_2::ManifestStanzas>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManifestExpand {
        V010102(crate::manifest::v1_1_2::Expand),
        V000001(crate::manifest::v1_1_2::ManifestExpandN1),
    }
    impl Default for ManifestExpand {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManifestExpandN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManifestStanzas {
        V010102(crate::manifest::v1_1_2::Stanza),
        V000001(crate::manifest::v1_1_2::ManifestStanzasN1),
    }
    impl Default for ManifestStanzas {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManifestStanzasN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Request {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Response {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Stanza {
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMStanzaType")]
        pub oem_stanza_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Request")]
        pub request: Option<crate::manifest::v1_1_2::StanzaRequest>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Response")]
        pub response: Option<crate::manifest::v1_1_2::StanzaResponse>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaId")]
        pub stanza_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaType")]
        pub stanza_type: Option<crate::manifest::v1_1_2::StanzaStanzaType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StanzaRequest {
        V010102(crate::manifest::v1_1_2::Request),
        V000001(crate::manifest::v1_1_2::StanzaRequestN1),
    }
    impl Default for StanzaRequest {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StanzaRequestN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StanzaResponse {
        V010102(crate::manifest::v1_1_2::Response),
        V000001(crate::manifest::v1_1_2::StanzaResponseN1),
    }
    impl Default for StanzaResponse {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StanzaResponseN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StanzaStanzaType {
        V010102(crate::manifest::v1_1_2::StanzaType),
        V000001(crate::manifest::v1_1_2::StanzaStanzaTypeN1),
    }
    impl Default for StanzaStanzaType {
        fn default() -> Self {
            Self::V010102(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StanzaStanzaTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StanzaType {
        #[default]
        #[serde(rename = "ComposeResource")]
        ComposeResource,
        #[serde(rename = "ComposeSystem")]
        ComposeSystem,
        #[serde(rename = "DecomposeResource")]
        DecomposeResource,
        #[serde(rename = "DecomposeSystem")]
        DecomposeSystem,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RegisterResourceBlock")]
        RegisterResourceBlock,
    }
}
