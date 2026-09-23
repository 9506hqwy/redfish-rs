pub type PolicyService = crate::policy_service::v1_0_0::PolicyService;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::policy_service::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMode {
        #[default]
        #[serde(rename = "AlertOnly")]
        AlertOnly,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PolicyService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::policy_service::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActionsSupported")]
        pub actions_supported: Option<Vec<crate::policy::ResponseAction>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConditionTypesSupported"
        )]
        pub condition_types_supported: Option<Vec<crate::policy::ConditionType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::policy_service::v1_0_0::PolicyServiceDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::policy_service::v1_0_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingMode")]
        pub operating_mode: Option<crate::policy_service::v1_0_0::PolicyServiceOperatingMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Policies")]
        pub policies: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceTypesSupported"
        )]
        pub resource_types_supported: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PolicyServiceDescription {
        V000001(crate::policy_service::v1_0_0::PolicyServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PolicyServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PolicyServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PolicyServiceOperatingMode {
        V010000(crate::policy_service::v1_0_0::OperatingMode),
        V000001(crate::policy_service::v1_0_0::PolicyServiceOperatingModeN1),
    }
    impl Default for PolicyServiceOperatingMode {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PolicyServiceOperatingModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
