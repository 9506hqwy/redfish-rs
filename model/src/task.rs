use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Task {
    V010701(crate::task::v1_7_1::Task),
    V010603(crate::task::v1_6_3::Task),
    V010503(crate::task::v1_5_3::Task),
    V010406(crate::task::v1_4_6::Task),
    V010307(crate::task::v1_3_7::Task),
    V010206(crate::task::v1_2_6::Task),
    V010107(crate::task::v1_1_7::Task),
    V010010(crate::task::v1_0_10::Task),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_0_10::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_1_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_1_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_1_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_1_7::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_2_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_2_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_2_6::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_2_6::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_3_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_3_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Payload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpOperation")]
        pub http_operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonBody")]
        pub json_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetUri")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_3_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::task::v1_3_7::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_3_7::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_4_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_4_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Payload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpOperation")]
        pub http_operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonBody")]
        pub json_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetUri")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_4_6::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::task::v1_4_6::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_4_6::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_5_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_5_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Payload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpOperation")]
        pub http_operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonBody")]
        pub json_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetUri")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_5_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::task::v1_5_3::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubTasks")]
        pub sub_tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_5_3::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_6_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_6_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Payload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpOperation")]
        pub http_operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonBody")]
        pub json_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetUri")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_6_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::task::v1_6_3::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubTasks")]
        pub sub_tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_6_3::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
pub mod v1_7_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task::v1_7_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedResources")]
        pub created_resources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CreatedResources@odata.count"
        )]
        pub created_resources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Payload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpHeaders")]
        pub http_headers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpOperation")]
        pub http_operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonBody")]
        pub json_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetUri")]
        pub target_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Task {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task::v1_7_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::task::v1_7_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Messages")]
        pub messages: Option<Vec<crate::message::Message>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::task::v1_7_1::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubTasks")]
        pub sub_tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskMonitor")]
        pub task_monitor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskState")]
        pub task_state: Option<crate::task::v1_7_1::TaskState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TaskStatus")]
        pub task_status: Option<crate::resource::Health>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Cancelling")]
        Cancelling,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
        #[serde(rename = "Killed")]
        Killed,
        #[serde(rename = "New")]
        New,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Running")]
        Running,
        #[serde(rename = "Service")]
        Service,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Stopping")]
        Stopping,
        #[serde(rename = "Suspended")]
        Suspended,
    }
}
