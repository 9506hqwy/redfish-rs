pub type SecureBoot = crate::secure_boot::v1_2_0::SecureBoot;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::secure_boot::v1_2_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#SecureBoot.ResetKeys"
        )]
        pub secure_boot_reset_keys: Option<crate::secure_boot::v1_2_0::ResetKeys>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetKeys {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetKeysRequestBody {
        #[serde(rename = "ResetKeysType")]
        pub reset_keys_type: crate::secure_boot::v1_2_0::ResetKeysType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetKeysType {
        #[default]
        #[serde(rename = "DeleteAllKeys")]
        DeleteAllKeys,
        #[serde(rename = "DeletePK")]
        DeletePK,
        #[serde(rename = "ResetAllKeysToDefault")]
        ResetAllKeysToDefault,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureBoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::secure_boot::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::secure_boot::v1_2_0::SecureBootDescription>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureBootCurrentBoot"
        )]
        pub secure_boot_current_boot:
            Option<crate::secure_boot::v1_2_0::SecureBootSecureBootCurrentBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureBootDatabases"
        )]
        pub secure_boot_databases: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureBootDesiredMode"
        )]
        pub secure_boot_desired_mode:
            Option<crate::secure_boot::v1_2_0::SecureBootSecureBootDesiredMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBootEnable")]
        pub secure_boot_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBootMode")]
        pub secure_boot_mode: Option<crate::secure_boot::v1_2_0::SecureBootSecureBootMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootCurrentBootType {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SecureBootDescription {
        V000001(crate::secure_boot::v1_2_0::SecureBootDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SecureBootDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootModeType {
        #[default]
        #[serde(rename = "AuditMode")]
        AuditMode,
        #[serde(rename = "DeployedMode")]
        DeployedMode,
        #[serde(rename = "SetupMode")]
        SetupMode,
        #[serde(rename = "UserMode")]
        UserMode,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SecureBootSecureBootCurrentBoot {
        V010200(crate::secure_boot::v1_2_0::SecureBootCurrentBootType),
        V000001(crate::secure_boot::v1_2_0::SecureBootSecureBootCurrentBootN1),
    }
    impl Default for SecureBootSecureBootCurrentBoot {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootSecureBootCurrentBootN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SecureBootSecureBootDesiredMode {
        V010200(crate::secure_boot::v1_2_0::SecureBootModeType),
        V000001(crate::secure_boot::v1_2_0::SecureBootSecureBootDesiredModeN1),
    }
    impl Default for SecureBootSecureBootDesiredMode {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootSecureBootDesiredModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SecureBootSecureBootMode {
        V010200(crate::secure_boot::v1_2_0::SecureBootModeType),
        V000001(crate::secure_boot::v1_2_0::SecureBootSecureBootModeN1),
    }
    impl Default for SecureBootSecureBootMode {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootSecureBootModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
