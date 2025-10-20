pub type AutomationInstrumentation =
    crate::automation_instrumentation::v1_0_0::AutomationInstrumentation;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::automation_instrumentation::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AutomationInstrumentation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::automation_instrumentation::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentAmps")]
        pub current_amps:
            Option<crate::automation_instrumentation::v1_0_0::AutomationInstrumentationCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::automation_instrumentation::v1_0_0::AutomationInstrumentationDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeControl")]
        pub node_control:
            Option<crate::automation_instrumentation::v1_0_0::AutomationInstrumentationNodeControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeState")]
        pub node_state: Option<crate::automation_node::NodeState>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PID")]
        pub pid: Option<crate::automation_instrumentation::v1_0_0::AutomationInstrumentationPID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TemperatureCelsius")]
        pub temperature_celsius: Option<
            crate::automation_instrumentation::v1_0_0::AutomationInstrumentationTemperatureCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltage")]
        pub voltage:
            Option<crate::automation_instrumentation::v1_0_0::AutomationInstrumentationVoltage>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationCurrentAmps {
        V000001(crate::automation_instrumentation::v1_0_0::AutomationInstrumentationCurrentAmpsN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_3::SensorCurrentExcerpt),
    }
    impl Default for AutomationInstrumentationCurrentAmps {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationDescription {
        V000001(crate::automation_instrumentation::v1_0_0::AutomationInstrumentationDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AutomationInstrumentationDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationNodeControl {
        V000001(crate::automation_instrumentation::v1_0_0::AutomationInstrumentationNodeControlN1),
        ControlControlNodeExcerpt(crate::control::v1_7_0::ControlNodeExcerpt),
    }
    impl Default for AutomationInstrumentationNodeControl {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationNodeControlN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationPID {
        V000001(crate::automation_instrumentation::v1_0_0::AutomationInstrumentationPIDN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for AutomationInstrumentationPID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationPIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationTemperatureCelsius {
        V000001 (crate :: automation_instrumentation :: v1_0_0 :: AutomationInstrumentationTemperatureCelsiusN1) , SensorSensorExcerpt (crate :: sensor :: v1_9_3 :: SensorExcerpt) }
    impl Default for AutomationInstrumentationTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationInstrumentationVoltage {
        V000001(crate::automation_instrumentation::v1_0_0::AutomationInstrumentationVoltageN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_3::SensorVoltageExcerpt),
    }
    impl Default for AutomationInstrumentationVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationInstrumentationVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
