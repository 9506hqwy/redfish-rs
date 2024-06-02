use serde::{Deserialize, Serialize};
pub type ContainerImage = crate::container_image::v1_0_1::ContainerImage;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ImageTypes {
    #[default]
    #[serde(rename = "DockerV1")]
    DockerV1,
    #[serde(rename = "DockerV2")]
    DockerV2,
    #[serde(rename = "OCI")]
    OCI,
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::container_image::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContainerImage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::container_image::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreateTime")]
        pub create_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::container_image::v1_0_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProgrammaticId")]
        pub programmatic_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeBytes")]
        pub size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::container_image::ImageTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Containers")]
        pub containers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Containers@odata.count"
        )]
        pub containers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImage")]
        pub software_image: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::container_image::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContainerImage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::container_image::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreateTime")]
        pub create_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::container_image::v1_0_1::ContainerImageDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::container_image::v1_0_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProgrammaticId")]
        pub programmatic_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeBytes")]
        pub size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::container_image::v1_0_1::ContainerImageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerImageDescription {
        V000001(crate::container_image::v1_0_1::ContainerImageDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ContainerImageDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerImageDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerImageType {
        V000001(crate::container_image::v1_0_1::ContainerImageTypeN1),
        ContainerImageImageTypes(crate::container_image::ImageTypes),
    }
    impl Default for ContainerImageType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerImageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Containers")]
        pub containers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Containers@odata.count"
        )]
        pub containers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImage")]
        pub software_image: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
