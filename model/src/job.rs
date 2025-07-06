pub type Job = crate::job::v1_3_0::Job;
pub type JobExcerpt = crate::job::v1_3_0::JobExcerpt;
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
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Cancel")]
        pub job_cancel: Option<crate::job::v1_3_0::Cancel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.ForceStart")]
        pub job_force_start: Option<crate::job::v1_3_0::ForceStart>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Invalidate")]
        pub job_invalidate: Option<crate::job::v1_3_0::Invalidate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Resubmit")]
        pub job_resubmit: Option<crate::job::v1_3_0::Resubmit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Resume")]
        pub job_resume: Option<crate::job::v1_3_0::Resume>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Suspend")]
        pub job_suspend: Option<crate::job::v1_3_0::Suspend>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Job.Validate")]
        pub job_validate: Option<crate::job::v1_3_0::Validate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::job::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Cancel {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CancelRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceStart {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceStartRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Invalidate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InvalidateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Job {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::job::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedBy")]
        pub created_by: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreationTime")]
        pub creation_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::job::v1_3_0::JobDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndTime")]
        pub end_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EstimatedCompletionTime"
        )]
        pub estimated_completion_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobPriority")]
        pub job_priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobState")]
        pub job_state: Option<crate::job::v1_3_0::JobState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobStatus")]
        pub job_status: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobType")]
        pub job_type: Option<crate::job::v1_3_0::JobType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::job::v1_3_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Parameters")]
        pub parameters: Option<crate::job::v1_3_0::Parameters>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Payload")]
        pub payload: Option<crate::job::v1_3_0::Payload>,
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
        V000001(crate::job::v1_3_0::JobDescriptionN1),
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
    pub struct JobExcerpt {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EstimatedCompletionTime"
        )]
        pub estimated_completion_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobState")]
        pub job_state: Option<crate::job::v1_3_0::JobState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentComplete")]
        pub percent_complete: Option<i64>,
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
        #[serde(rename = "Invalid")]
        Invalid,
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
        #[serde(rename = "Validating")]
        Validating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobType {
        #[default]
        #[serde(rename = "DocumentBased")]
        DocumentBased,
        #[serde(rename = "ServiceGenerated")]
        ServiceGenerated,
        #[serde(rename = "UserSpecified")]
        UserSpecified,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Executor")]
        pub executor: Option<crate::job::v1_3_0::LinksExecutor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobDocument")]
        pub job_document: Option<crate::job::v1_3_0::LinksJobDocument>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ParentJob")]
        pub parent_job: Option<crate::job::v1_3_0::LinksParentJob>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PreferredExecutors")]
        pub preferred_executors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PreferredExecutors@odata.count"
        )]
        pub preferred_executors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsidiaryJobs")]
        pub subsidiary_jobs: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SubsidiaryJobs@odata.count"
        )]
        pub subsidiary_jobs_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidatedExecutors")]
        pub validated_executors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValidatedExecutors@odata.count"
        )]
        pub validated_executors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksExecutor {
        V000001(crate::job::v1_3_0::LinksExecutorN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksExecutor {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksExecutorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksJobDocument {
        V000001(crate::job::v1_3_0::LinksJobDocumentN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksJobDocument {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksJobDocumentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksParentJob {
        V000001(crate::job::v1_3_0::LinksParentJobN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksParentJob {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksParentJobN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Parameters {}
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
    pub struct Resubmit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResubmitRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resume {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Suspend {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Validate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ValidateRequestBody {}
}
