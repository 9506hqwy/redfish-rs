pub type ServiceConditions = crate::service_conditions::v1_0_1::ServiceConditions;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::service_conditions::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceConditions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::service_conditions::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Conditions")]
        pub conditions: Option<Vec<crate::service_conditions::v1_0_1::ServiceConditionsConditions>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::service_conditions::v1_0_1::ServiceConditionsDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthRollup")]
        pub health_rollup: Option<crate::resource::Health>,
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
    pub enum ServiceConditionsConditions {
        V000001(crate::service_conditions::v1_0_1::ServiceConditionsConditionsN1),
        ResourceCondition(crate::resource::Condition),
    }
    impl Default for ServiceConditionsConditions {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ServiceConditionsConditionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ServiceConditionsDescription {
        V000001(crate::service_conditions::v1_0_1::ServiceConditionsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ServiceConditionsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ServiceConditionsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
