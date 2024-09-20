pub type BootOption = crate::boot_option::v1_0_6::BootOption;
pub mod v1_0_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::boot_option::v1_0_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootOption {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::boot_option::v1_0_6::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Alias")]
        pub alias: Option<crate::boot_option::v1_0_6::BootOptionAlias>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptionEnabled")]
        pub boot_option_enabled: Option<bool>,
        #[serde(rename = "BootOptionReference")]
        pub boot_option_reference: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::boot_option::v1_0_6::BootOptionDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayName")]
        pub display_name: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePath")]
        pub uefi_device_path: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootOptionAlias {
        V000001(crate::boot_option::v1_0_6::BootOptionAliasN1),
        ComputerSystemBootSource(crate::computer_system::BootSource),
    }
    impl Default for BootOptionAlias {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOptionAliasN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootOptionDescription {
        V000001(crate::boot_option::v1_0_6::BootOptionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for BootOptionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOptionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
