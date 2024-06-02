pub type Container = crate::container::v1_0_1::Container;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Container.Reset")]
        pub container_reset: Option<crate::container::v1_0_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::container::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Container {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::container::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Limits")]
        pub limits: Option<crate::container::v1_0_0::Limits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::container::v1_0_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MountPoints")]
        pub mount_points: Option<Vec<crate::container::v1_0_0::MountPoint>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Limits {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPUCount")]
        pub cpu_count: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryBytes")]
        pub memory_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerImage")]
        pub container_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MountPoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Destination")]
        pub destination: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Source")]
        pub source: Option<String>,
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
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Container.Reset")]
        pub container_reset: Option<crate::container::v1_0_1::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::container::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Container {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::container::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::container::v1_0_1::ContainerDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Limits")]
        pub limits: Option<crate::container::v1_0_1::Limits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::container::v1_0_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MountPoints")]
        pub mount_points: Option<Vec<crate::container::v1_0_1::ContainerMountPoints>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartTime")]
        pub start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerDescription {
        V000001(crate::container::v1_0_1::ContainerDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ContainerDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerMountPoints {
        V010001(crate::container::v1_0_1::MountPoint),
        V000001(crate::container::v1_0_1::ContainerMountPointsN1),
    }
    impl Default for ContainerMountPoints {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerMountPointsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Limits {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPUCount")]
        pub cpu_count: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryBytes")]
        pub memory_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerImage")]
        pub container_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MountPoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Destination")]
        pub destination: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Source")]
        pub source: Option<String>,
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
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
}
