pub type VCATEntry = crate::vcat_entry::v1_0_3::VCATEntry;
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vcat_entry::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VCATEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vcat_entry::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RawEntryHex")]
        pub raw_entry_hex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCEntries")]
        pub vc_entries: Option<Vec<crate::vcat_entry::v1_0_2::VCATableEntry>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VCATableEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Threshold")]
        pub threshold: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCMask")]
        pub vc_mask: Option<String>,
    }
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vcat_entry::v1_0_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VCATEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vcat_entry::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::vcat_entry::v1_0_3::VCATEntryDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RawEntryHex")]
        pub raw_entry_hex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCEntries")]
        pub vc_entries: Option<Vec<crate::vcat_entry::v1_0_3::VCATableEntry>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VCATEntryDescription {
        V000001(crate::vcat_entry::v1_0_3::VCATEntryDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VCATEntryDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VCATEntryDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VCATableEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Threshold")]
        pub threshold: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCMask")]
        pub vc_mask: Option<String>,
    }
}
