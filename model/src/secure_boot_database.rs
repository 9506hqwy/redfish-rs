pub type SecureBootDatabase = crate::secure_boot_database::v1_0_3::SecureBootDatabase;
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::secure_boot_database::v1_0_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#SecureBootDatabase.ResetKeys"
        )]
        pub secure_boot_database_reset_keys: Option<crate::secure_boot_database::v1_0_3::ResetKeys>,
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
        pub reset_keys_type: crate::secure_boot_database::v1_0_3::ResetKeysType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetKeysType {
        #[default]
        #[serde(rename = "DeleteAllKeys")]
        DeleteAllKeys,
        #[serde(rename = "ResetAllKeysToDefault")]
        ResetAllKeysToDefault,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureBootDatabase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::secure_boot_database::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DatabaseId")]
        pub database_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::secure_boot_database::v1_0_3::SecureBootDatabaseDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Signatures")]
        pub signatures: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SecureBootDatabaseDescription {
        V000001(crate::secure_boot_database::v1_0_3::SecureBootDatabaseDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SecureBootDatabaseDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureBootDatabaseDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
