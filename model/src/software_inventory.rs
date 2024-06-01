pub type AdditionalVersions = crate::software_inventory::v1_10_0::AdditionalVersions;
pub type MeasurementBlock = crate::software_inventory::v1_10_0::MeasurementBlock;
pub type SoftwareInventory = crate::software_inventory::v1_10_0::SoftwareInventory;
pub mod v1_9_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::software_inventory::v1_9_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AdditionalVersions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bootloader")]
        pub bootloader: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Kernel")]
        pub kernel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Microcode")]
        pub microcode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OSDistribution")]
        pub os_distribution: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MeasurementBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndex")]
        pub measurement_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSize")]
        pub measurement_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSpecification"
        )]
        pub measurement_specification: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SoftwareInventory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::software_inventory::v1_9_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalVersions")]
        pub additional_versions: Option<crate::software_inventory::v1_9_0::AdditionalVersions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowestSupportedVersion"
        )]
        pub lowest_supported_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<crate::software_inventory::MeasurementBlock>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReleaseDate")]
        pub release_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareId")]
        pub software_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePaths")]
        pub uefi_device_paths: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Updateable")]
        pub updateable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VersionScheme")]
        pub version_scheme: Option<crate::software_inventory::v1_9_0::VersionScheme>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VersionScheme {
        #[default]
        #[serde(rename = "DotIntegerNotation")]
        DotIntegerNotation,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SemVer")]
        SemVer,
    }
}
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::software_inventory::v1_10_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AdditionalVersions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bootloader")]
        pub bootloader: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Kernel")]
        pub kernel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Microcode")]
        pub microcode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OSDistribution")]
        pub os_distribution: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MeasurementBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndex")]
        pub measurement_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSize")]
        pub measurement_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSpecification"
        )]
        pub measurement_specification: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReleaseType {
        #[default]
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "Production")]
        Production,
        #[serde(rename = "Prototype")]
        Prototype,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SoftwareInventory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::software_inventory::v1_10_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalVersions")]
        pub additional_versions: Option<crate::software_inventory::v1_10_0::AdditionalVersions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalContext"
        )]
        pub associated_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowestSupportedVersion"
        )]
        pub lowest_supported_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<crate::software_inventory::MeasurementBlock>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReleaseDate")]
        pub release_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReleaseType")]
        pub release_type: Option<crate::software_inventory::v1_10_0::ReleaseType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareId")]
        pub software_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePaths")]
        pub uefi_device_paths: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Updateable")]
        pub updateable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VersionScheme")]
        pub version_scheme: Option<crate::software_inventory::v1_10_0::VersionScheme>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VersionScheme {
        #[default]
        #[serde(rename = "DotIntegerNotation")]
        DotIntegerNotation,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SemVer")]
        SemVer,
    }
}
