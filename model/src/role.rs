pub type Role = crate::role::v1_3_1::Role;
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::role::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Role {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::role::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlternateRoleId")]
        pub alternate_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssignedPrivileges")]
        pub assigned_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsPredefined")]
        pub is_predefined: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemPrivileges")]
        pub oem_privileges: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Restricted")]
        pub restricted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoleId")]
        pub role_id: Option<String>,
    }
}
