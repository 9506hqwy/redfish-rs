pub type Job = crate::job::v1_2_4::Job;
pub mod v1_2_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::job::v1_2_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Job {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::job::v1_2_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedBy")]
        pub created_by: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::job::v1_2_4::JobDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobState")]
        pub job_state: Option<crate::job::v1_2_4::JobState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobStatus")]
        pub job_status: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::job::v1_2_4::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxExecutionTime")]
        pub max_execution_time: Option<String>,
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
        pub payload: Option<crate::job::v1_2_4::Payload>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Schedule")]
        pub schedule: Option<crate::schedule::Schedule>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StepOrder")]
        pub step_order: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Steps")]
        pub steps: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum JobDescription {
        V000001(crate::job::v1_2_4::JobDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for JobDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobState {
        #[default]
        #[serde(rename = "Cancelled")]
        Cancelled,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Continue")]
        Continue,
        #[serde(rename = "Exception")]
        Exception,
        #[serde(rename = "Interrupted")]
        Interrupted,
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
        #[serde(rename = "UserIntervention")]
        UserIntervention,
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
}
