use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum JsonSchemaFile {
    V010104(crate::json_schema_file::v1_1_4::JsonSchemaFile),
    V010008(crate::json_schema_file::v1_0_8::JsonSchemaFile),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct JsonSchemaFile {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Languages")]
        pub languages: Vec<String>,
        #[serde(rename = "Location")]
        pub location: Vec<crate::json_schema_file::v1_0_8::Location>,
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
        #[serde(rename = "Schema")]
        pub schema: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArchiveFile")]
        pub archive_file: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArchiveUri")]
        pub archive_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Language")]
        pub language: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PublicationUri")]
        pub publication_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Uri")]
        pub uri: Option<String>,
    }
}
pub mod v1_1_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::json_schema_file::v1_1_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct JsonSchemaFile {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::json_schema_file::v1_1_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Languages")]
        pub languages: Vec<String>,
        #[serde(rename = "Location")]
        pub location: Vec<crate::json_schema_file::v1_1_4::Location>,
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
        #[serde(rename = "Schema")]
        pub schema: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArchiveFile")]
        pub archive_file: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArchiveUri")]
        pub archive_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Language")]
        pub language: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PublicationUri")]
        pub publication_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Uri")]
        pub uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
