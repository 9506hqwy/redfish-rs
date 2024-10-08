pub type TaskService = crate::task_service::v1_2_1::TaskService;
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::task_service::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OverWritePolicy {
        #[default]
        #[serde(rename = "Manual")]
        Manual,
        #[serde(rename = "Oldest")]
        Oldest,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TaskService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::task_service::v1_2_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompletedTaskOverWritePolicy"
        )]
        pub completed_task_over_write_policy: Option<crate::task_service::v1_2_1::OverWritePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::task_service::v1_2_1::TaskServiceDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LifeCycleEventOnTaskStateChange"
        )]
        pub life_cycle_event_on_task_state_change: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TaskAutoDeleteTimeoutMinutes"
        )]
        pub task_auto_delete_timeout_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TaskServiceDescription {
        V000001(crate::task_service::v1_2_1::TaskServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for TaskServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TaskServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
