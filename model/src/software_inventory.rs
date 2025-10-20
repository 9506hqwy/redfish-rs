pub type AdditionalVersions = crate::software_inventory::v1_13_0::AdditionalVersions;
pub type MeasurementBlock = crate::software_inventory::v1_13_0::MeasurementBlock;
pub type SoftwareInventory = crate::software_inventory::v1_13_0::SoftwareInventory;
pub mod v1_12_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::software_inventory::v1_12_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#SoftwareInventory.Activate"
        )]
        pub software_inventory_activate: Option<crate::software_inventory::v1_12_0::Activate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Activate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ActivateRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<crate::odata_v4::IdRef>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AdditionalVersions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootParameters")]
        pub boot_parameters: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bootloader")]
        pub bootloader: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FactoryConfiguration"
        )]
        pub factory_configuration: Option<String>,
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
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveTargets")]
        pub active_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveTargets@odata.count"
        )]
        pub active_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StagedTargets")]
        pub staged_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StagedTargets@odata.count"
        )]
        pub staged_targets_odata_count: Option<i64>,
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
        pub actions: Option<crate::software_inventory::v1_12_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
        pub active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalVersions")]
        pub additional_versions: Option<crate::software_inventory::v1_12_0::AdditionalVersions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalContext"
        )]
        pub associated_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::software_inventory::v1_12_0::SoftwareInventoryDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::software_inventory::v1_12_0::Links>,
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
        pub release_type: Option<crate::software_inventory::v1_12_0::SoftwareInventoryReleaseType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResetRequiredOnUpdate"
        )]
        pub reset_required_on_update: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareId")]
        pub software_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Staged")]
        pub staged: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePaths")]
        pub uefi_device_paths: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Updateable")]
        pub updateable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VersionScheme")]
        pub version_scheme:
            Option<crate::software_inventory::v1_12_0::SoftwareInventoryVersionScheme>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryDescription {
        V000001(crate::software_inventory::v1_12_0::SoftwareInventoryDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SoftwareInventoryDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryReleaseType {
        V011200(crate::software_inventory::v1_12_0::ReleaseType),
        V000001(crate::software_inventory::v1_12_0::SoftwareInventoryReleaseTypeN1),
    }
    impl Default for SoftwareInventoryReleaseType {
        fn default() -> Self {
            Self::V011200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryReleaseTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryVersionScheme {
        V011200(crate::software_inventory::v1_12_0::VersionScheme),
        V000001(crate::software_inventory::v1_12_0::SoftwareInventoryVersionSchemeN1),
    }
    impl Default for SoftwareInventoryVersionScheme {
        fn default() -> Self {
            Self::V011200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryVersionSchemeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
pub mod v1_13_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::software_inventory::v1_13_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#SoftwareInventory.Activate"
        )]
        pub software_inventory_activate: Option<crate::software_inventory::v1_13_0::Activate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Activate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ActivateRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<crate::odata_v4::IdRef>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AdditionalVersions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootParameters")]
        pub boot_parameters: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bootloader")]
        pub bootloader: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FactoryConfiguration"
        )]
        pub factory_configuration: Option<String>,
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
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveTargets")]
        pub active_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveTargets@odata.count"
        )]
        pub active_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StagedTargets")]
        pub staged_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StagedTargets@odata.count"
        )]
        pub staged_targets_odata_count: Option<i64>,
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
        pub actions: Option<crate::software_inventory::v1_13_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
        pub active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalVersions")]
        pub additional_versions: Option<crate::software_inventory::v1_13_0::AdditionalVersions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalContext"
        )]
        pub associated_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::software_inventory::v1_13_0::SoftwareInventoryDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImageDataURI")]
        pub image_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::software_inventory::v1_13_0::Links>,
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
        pub release_type: Option<crate::software_inventory::v1_13_0::SoftwareInventoryReleaseType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResetRequiredOnUpdate"
        )]
        pub reset_required_on_update: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeBytes")]
        pub size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareId")]
        pub software_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Staged")]
        pub staged: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePaths")]
        pub uefi_device_paths: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Updateable")]
        pub updateable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VersionScheme")]
        pub version_scheme:
            Option<crate::software_inventory::v1_13_0::SoftwareInventoryVersionScheme>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryDescription {
        V000001(crate::software_inventory::v1_13_0::SoftwareInventoryDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SoftwareInventoryDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryReleaseType {
        V011300(crate::software_inventory::v1_13_0::ReleaseType),
        V000001(crate::software_inventory::v1_13_0::SoftwareInventoryReleaseTypeN1),
    }
    impl Default for SoftwareInventoryReleaseType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryReleaseTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SoftwareInventoryVersionScheme {
        V011300(crate::software_inventory::v1_13_0::VersionScheme),
        V000001(crate::software_inventory::v1_13_0::SoftwareInventoryVersionSchemeN1),
    }
    impl Default for SoftwareInventoryVersionScheme {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SoftwareInventoryVersionSchemeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
