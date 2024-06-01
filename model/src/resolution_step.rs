pub type ResolutionStep = crate::resolution_step::v1_0_0::ResolutionStep;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResolutionStep {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActionParameters")]
        pub action_parameters: Option<Vec<crate::action_info::Parameters>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActionURI")]
        pub action_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(rename = "ResolutionType")]
        pub resolution_type: Option<crate::resolution_step::v1_0_0::ResolutionType>,
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
