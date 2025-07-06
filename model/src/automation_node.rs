use serde::{Deserialize, Serialize};
pub type AutomationNode = crate::automation_node::v1_0_0::AutomationNode;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum NodeState {
    #[default]
    #[serde(rename = "ConditionStop")]
    ConditionStop,
    #[serde(rename = "Done")]
    Done,
    #[serde(rename = "ErrorStop")]
    ErrorStop,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Waiting")]
    Waiting,
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AutomationNode.Reset"
        )]
        pub automation_node_reset: Option<crate::automation_node::v1_0_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AutomationNode.SendTrigger"
        )]
        pub automation_node_send_trigger: Option<crate::automation_node::v1_0_0::SendTrigger>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AutomationNode.Start"
        )]
        pub automation_node_start: Option<crate::automation_node::v1_0_0::Start>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AutomationNode.Stop"
        )]
        pub automation_node_stop: Option<crate::automation_node::v1_0_0::Stop>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AutomationNode.Wait"
        )]
        pub automation_node_wait: Option<crate::automation_node::v1_0_0::Wait>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::automation_node::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AutomationNode {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::automation_node::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::automation_node::v1_0_0::AutomationNodeDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Instrumentation")]
        pub instrumentation: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::automation_node::v1_0_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MotionAxis")]
        pub motion_axis: Option<crate::automation_node::v1_0_0::AutomationNodeMotionAxis>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MotionProfile")]
        pub motion_profile: Option<crate::automation_node::v1_0_0::AutomationNodeMotionProfile>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeState")]
        pub node_state: Option<crate::automation_node::NodeState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeType")]
        pub node_type: Option<crate::automation_node::v1_0_0::NodeType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationNodeDescription {
        V000001(crate::automation_node::v1_0_0::AutomationNodeDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AutomationNodeDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationNodeDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationNodeMotionAxis {
        V010000(crate::automation_node::v1_0_0::MotionAxisType),
        V000001(crate::automation_node::v1_0_0::AutomationNodeMotionAxisN1),
    }
    impl Default for AutomationNodeMotionAxis {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationNodeMotionAxisN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AutomationNodeMotionProfile {
        V010000(crate::automation_node::v1_0_0::MotionProfileType),
        V000001(crate::automation_node::v1_0_0::AutomationNodeMotionProfileN1),
    }
    impl Default for AutomationNodeMotionProfile {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomationNodeMotionProfileN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomationNodeGroup"
        )]
        pub automation_node_group: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomationNodeGroup@odata.count"
        )]
        pub automation_node_group_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputControl")]
        pub output_control: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PidFeedbackSensor")]
        pub pid_feedback_sensor: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PositionSensor")]
        pub position_sensor: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VelocitySensor")]
        pub velocity_sensor: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MotionAxisType {
        #[default]
        #[serde(rename = "ThreeAxis")]
        ThreeAxis,
        #[serde(rename = "TwoAxis")]
        TwoAxis,
        #[serde(rename = "X")]
        X,
        #[serde(rename = "Y")]
        Y,
        #[serde(rename = "Z")]
        Z,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MotionProfileType {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "SCurve")]
        SCurve,
        #[serde(rename = "Trapezoidal")]
        Trapezoidal,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NodeType {
        #[default]
        #[serde(rename = "MotionPosition")]
        MotionPosition,
        #[serde(rename = "MotionPositionGroup")]
        MotionPositionGroup,
        #[serde(rename = "MotionVelocity")]
        MotionVelocity,
        #[serde(rename = "PID")]
        PID,
        #[serde(rename = "Simple")]
        Simple,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SendTrigger {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SendTriggerRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Start {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Stop {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StopRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wait {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WaitRequestBody {}
}
