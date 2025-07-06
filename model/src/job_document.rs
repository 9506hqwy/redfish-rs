pub type JobDocument = crate::job_document::v1_0_0::JobDocument;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#JobDocument.SubmitJob"
        )]
        pub job_document_submit_job: Option<crate::job_document::v1_0_0::SubmitJob>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::job_document::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataType {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "Number")]
        Number,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct JobDocument {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::job_document::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreationTime")]
        pub creation_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::job_document::v1_0_0::JobDocumentDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DocumentData")]
        pub document_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DocumentDataHash")]
        pub document_data_hash: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DocumentDataURI")]
        pub document_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DocumentType")]
        pub document_type: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::job_document::v1_0_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ParameterMetadata")]
        pub parameter_metadata: Option<Vec<crate::job_document::v1_0_0::ParameterMetadata>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum JobDocumentDescription {
        V000001(crate::job_document::v1_0_0::JobDocumentDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for JobDocumentDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum JobDocumentDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedExecutors")]
        pub supported_executors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedExecutors@odata.count"
        )]
        pub supported_executors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Parameter {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ParameterMetadata {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableNumbers")]
        pub allowable_numbers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowablePattern")]
        pub allowable_pattern: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowableValueDescriptions"
        )]
        pub allowable_value_descriptions: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableValues")]
        pub allowable_values: Option<Vec<String>>,
        #[serde(rename = "DataType")]
        pub data_type: crate::job_document::v1_0_0::DataType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumValue")]
        pub maximum_value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumValue")]
        pub minimum_value: Option<f64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Required")]
        pub required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValueHint")]
        pub value_hint: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitJob {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SubmitJobRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HidePayload")]
        pub hide_payload: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobCreator")]
        pub job_creator: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Parameters")]
        pub parameters: crate::job_document::v1_0_0::Parameter,
        #[serde(rename = "PreferredExecutors")]
        pub preferred_executors: Vec<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
    }
}
