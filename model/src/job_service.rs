pub type JobService = crate::job_service::v1_1_0::JobService;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#JobService.CancelAllJobs"
        )]
        pub job_service_cancel_all_jobs: Option<crate::job_service::v1_1_0::CancelAllJobs>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::job_service::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CancelAllJobs {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CancelAllJobsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct JobService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::job_service::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::job_service::v1_1_0::JobServiceDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobDocuments")]
        pub job_documents: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobExecutors")]
        pub job_executors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Jobs")]
        pub jobs: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceCapabilities"
        )]
        pub service_capabilities: Option<crate::job_service::v1_1_0::JobServiceCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidationPolicy")]
        pub validation_policy: Option<crate::job_service::v1_1_0::JobServiceValidationPolicy>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct JobServiceCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DocumentBasedJobs")]
        pub document_based_jobs: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxJobs")]
        pub max_jobs: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSteps")]
        pub max_steps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Scheduling")]
        pub scheduling: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserSpecifiedJobs")]
        pub user_specified_jobs: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum JobServiceDescription {
        V000001(crate::job_service::v1_1_0::JobServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for JobServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum JobServiceValidationPolicy {
        V010100(crate::job_service::v1_1_0::ValidationPolicy),
        V000001(crate::job_service::v1_1_0::JobServiceValidationPolicyN1),
    }
    impl Default for JobServiceValidationPolicy {
        fn default() -> Self {
            Self::V010100(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobServiceValidationPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ValidationPolicy {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Bypass")]
        Bypass,
        #[serde(rename = "Manual")]
        Manual,
    }
}
