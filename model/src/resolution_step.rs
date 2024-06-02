pub type ResolutionStep = crate::resolution_step::v1_0_1::ResolutionStep;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResolutionStep {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActionParameters")]
        pub action_parameters:
            Option<Vec<crate::resolution_step::v1_0_1::ResolutionStepActionParameters>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActionURI")]
        pub action_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(rename = "ResolutionType")]
        pub resolution_type: crate::resolution_step::v1_0_1::ResolutionStepResolutionType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetryCount")]
        pub retry_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RetryIntervalSeconds"
        )]
        pub retry_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetComponentURI")]
        pub target_component_uri: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResolutionStepActionParameters {
        V000001(crate::resolution_step::v1_0_1::ResolutionStepActionParametersN1),
        ActionInfoParameters(crate::action_info::v1_4_2::Parameters),
    }
    impl Default for ResolutionStepActionParameters {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResolutionStepActionParametersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResolutionStepResolutionType {
        V010001(crate::resolution_step::v1_0_1::ResolutionType),
        V000001(crate::resolution_step::v1_0_1::ResolutionStepResolutionTypeN1),
    }
    impl Default for ResolutionStepResolutionType {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResolutionStepResolutionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResolutionType {
        #[default]
        #[serde(rename = "CollectDiagnosticData")]
        CollectDiagnosticData,
        #[serde(rename = "ContactVendor")]
        ContactVendor,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "ReplaceComponent")]
        ReplaceComponent,
        #[serde(rename = "Reset")]
        Reset,
        #[serde(rename = "ResetToDefaults")]
        ResetToDefaults,
    }
}
