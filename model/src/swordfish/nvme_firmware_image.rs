use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum NVMeDeviceType {
    #[default]
    #[serde(rename = "Drive")]
    Drive,
    #[serde(rename = "FabricAttachArray")]
    FabricAttachArray,
    #[serde(rename = "JBOF")]
    JBOF,
}
pub type NVMeFirmwareImage = crate::swordfish::nvme_firmware_image::v1_2_0::NVMeFirmwareImage;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::nvme_firmware_image::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeFirmwareImage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::nvme_firmware_image::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::swordfish::nvme_firmware_image::v1_2_0::NVMeFirmwareImageDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeDeviceType")]
        pub nvme_device_type:
            Option<crate::swordfish::nvme_firmware_image::v1_2_0::NVMeFirmwareImageNVMeDeviceType>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeFirmwareImageDescription {
        V000001(crate::swordfish::nvme_firmware_image::v1_2_0::NVMeFirmwareImageDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for NVMeFirmwareImageDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeFirmwareImageDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeFirmwareImageNVMeDeviceType {
        V000001(crate::swordfish::nvme_firmware_image::v1_2_0::NVMeFirmwareImageNVMeDeviceTypeN1),
        NVMeFirmwareImageNVMeDeviceType(crate::swordfish::nvme_firmware_image::NVMeDeviceType),
    }
    impl Default for NVMeFirmwareImageNVMeDeviceType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeFirmwareImageNVMeDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
