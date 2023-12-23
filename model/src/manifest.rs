use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Manifest {
    ManifestV1N0N0Manifest(crate::manifest::v1_0_0::Manifest),
    ManifestV1N1N0Manifest(crate::manifest::v1_1_0::Manifest),
}
pub mod v1_0_0 {
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
        pub expand: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Stanzas")]
        pub stanzas: Option<Vec<crate::manifest::v1_0_0::Stanza>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
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
        pub request: Option<crate::manifest::v1_0_0::Request>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Response")]
        pub response: Option<crate::manifest::v1_0_0::Response>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaId")]
        pub stanza_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaType")]
        pub stanza_type: Option<String>,
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
    }
}
pub mod v1_1_0 {
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
        pub expand: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Stanzas")]
        pub stanzas: Option<Vec<crate::manifest::v1_1_0::Stanza>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
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
        pub request: Option<crate::manifest::v1_1_0::Request>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Response")]
        pub response: Option<crate::manifest::v1_1_0::Response>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaId")]
        pub stanza_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StanzaType")]
        pub stanza_type: Option<String>,
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
