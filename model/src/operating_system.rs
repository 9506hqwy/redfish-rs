pub type OperatingSystem = crate::operating_system::v1_0_2::OperatingSystem;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::operating_system::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContainerEngine {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementURIs")]
        pub management_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedImageTypes"
        )]
        pub supported_image_types: Option<Vec<crate::container_image::ImageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_1::ContainerEngineTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerEngineTypes {
        #[default]
        #[serde(rename = "CRIO")]
        CRIO,
        #[serde(rename = "Docker")]
        Docker,
        #[serde(rename = "containerd")]
        Containerd,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Kernel {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Machine")]
        pub machine: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Release")]
        pub release: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImage")]
        pub software_image: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OperatingSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::operating_system::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Applications")]
        pub applications: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerEngines")]
        pub container_engines: Option<Vec<crate::operating_system::v1_0_1::ContainerEngine>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerImages")]
        pub container_images: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Containers")]
        pub containers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Kernel")]
        pub kernel: Option<crate::operating_system::v1_0_1::Kernel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::operating_system::v1_0_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_1::OperatingSystemTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UptimeSeconds")]
        pub uptime_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualMachineEngines"
        )]
        pub virtual_machine_engines:
            Option<Vec<crate::operating_system::v1_0_1::VirtualMachineEngine>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemTypes {
        #[default]
        #[serde(rename = "AIX")]
        AIX,
        #[serde(rename = "BSD")]
        BSD,
        #[serde(rename = "HPUX")]
        HPUX,
        #[serde(rename = "Hypervisor")]
        Hypervisor,
        #[serde(rename = "IBMi")]
        IBMi,
        #[serde(rename = "Linux")]
        Linux,
        #[serde(rename = "Solaris")]
        Solaris,
        #[serde(rename = "Windows")]
        Windows,
        #[serde(rename = "macOS")]
        MacOS,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMachineEngine {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementURIs")]
        pub management_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedImageTypes"
        )]
        pub supported_image_types:
            Option<Vec<crate::operating_system::v1_0_1::VirtualMachineImageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_1::VirtualMachineEngineTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineEngineTypes {
        #[default]
        #[serde(rename = "HyperV")]
        HyperV,
        #[serde(rename = "KVM")]
        KVM,
        #[serde(rename = "PowerVM")]
        PowerVM,
        #[serde(rename = "QEMU")]
        QEMU,
        #[serde(rename = "VMwareESX")]
        VMwareESX,
        #[serde(rename = "VirtualBox")]
        VirtualBox,
        #[serde(rename = "Xen")]
        Xen,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineImageTypes {
        #[default]
        #[serde(rename = "OVA")]
        OVA,
        #[serde(rename = "OVF")]
        OVF,
        #[serde(rename = "QCOW")]
        QCOW,
        #[serde(rename = "QCOW2")]
        QCOW2,
        #[serde(rename = "Raw")]
        Raw,
        #[serde(rename = "VDI")]
        VDI,
        #[serde(rename = "VHD")]
        VHD,
        #[serde(rename = "VMDK")]
        VMDK,
    }
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::operating_system::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContainerEngine {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementURIs")]
        pub management_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedImageTypes"
        )]
        pub supported_image_types:
            Option<Vec<crate::operating_system::v1_0_2::ContainerEngineSupportedImageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_2::ContainerEngineType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerEngineSupportedImageTypes {
        V000001(crate::operating_system::v1_0_2::ContainerEngineSupportedImageTypesN1),
        ContainerImageImageTypes(crate::container_image::ImageTypes),
    }
    impl Default for ContainerEngineSupportedImageTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerEngineSupportedImageTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ContainerEngineType {
        V010002(crate::operating_system::v1_0_2::ContainerEngineTypes),
        V000001(crate::operating_system::v1_0_2::ContainerEngineTypeN1),
    }
    impl Default for ContainerEngineType {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerEngineTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ContainerEngineTypes {
        #[default]
        #[serde(rename = "CRIO")]
        CRIO,
        #[serde(rename = "Docker")]
        Docker,
        #[serde(rename = "containerd")]
        Containerd,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Kernel {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Machine")]
        pub machine: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Release")]
        pub release: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImage")]
        pub software_image: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OperatingSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::operating_system::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Applications")]
        pub applications: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerEngines")]
        pub container_engines:
            Option<Vec<crate::operating_system::v1_0_2::OperatingSystemContainerEngines>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerImages")]
        pub container_images: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Containers")]
        pub containers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::operating_system::v1_0_2::OperatingSystemDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Kernel")]
        pub kernel: Option<crate::operating_system::v1_0_2::OperatingSystemKernel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::operating_system::v1_0_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_2::OperatingSystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UptimeSeconds")]
        pub uptime_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualMachineEngines"
        )]
        pub virtual_machine_engines:
            Option<Vec<crate::operating_system::v1_0_2::OperatingSystemVirtualMachineEngines>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperatingSystemContainerEngines {
        V010002(crate::operating_system::v1_0_2::ContainerEngine),
        V000001(crate::operating_system::v1_0_2::OperatingSystemContainerEnginesN1),
    }
    impl Default for OperatingSystemContainerEngines {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemContainerEnginesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperatingSystemDescription {
        V000001(crate::operating_system::v1_0_2::OperatingSystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for OperatingSystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperatingSystemKernel {
        V010002(crate::operating_system::v1_0_2::Kernel),
        V000001(crate::operating_system::v1_0_2::OperatingSystemKernelN1),
    }
    impl Default for OperatingSystemKernel {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemKernelN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperatingSystemType {
        V010002(crate::operating_system::v1_0_2::OperatingSystemTypes),
        V000001(crate::operating_system::v1_0_2::OperatingSystemTypeN1),
    }
    impl Default for OperatingSystemType {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemTypes {
        #[default]
        #[serde(rename = "AIX")]
        AIX,
        #[serde(rename = "BSD")]
        BSD,
        #[serde(rename = "HPUX")]
        HPUX,
        #[serde(rename = "Hypervisor")]
        Hypervisor,
        #[serde(rename = "IBMi")]
        IBMi,
        #[serde(rename = "Linux")]
        Linux,
        #[serde(rename = "Solaris")]
        Solaris,
        #[serde(rename = "Windows")]
        Windows,
        #[serde(rename = "macOS")]
        MacOS,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OperatingSystemVirtualMachineEngines {
        V010002(crate::operating_system::v1_0_2::VirtualMachineEngine),
        V000001(crate::operating_system::v1_0_2::OperatingSystemVirtualMachineEnginesN1),
    }
    impl Default for OperatingSystemVirtualMachineEngines {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingSystemVirtualMachineEnginesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMachineEngine {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementURIs")]
        pub management_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedImageTypes"
        )]
        pub supported_image_types:
            Option<Vec<crate::operating_system::v1_0_2::VirtualMachineEngineSupportedImageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_2::VirtualMachineEngineType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VirtualMachineEngineSupportedImageTypes {
        V010002(crate::operating_system::v1_0_2::VirtualMachineImageTypes),
        V000001(crate::operating_system::v1_0_2::VirtualMachineEngineSupportedImageTypesN1),
    }
    impl Default for VirtualMachineEngineSupportedImageTypes {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineEngineSupportedImageTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VirtualMachineEngineType {
        V010002(crate::operating_system::v1_0_2::VirtualMachineEngineTypes),
        V000001(crate::operating_system::v1_0_2::VirtualMachineEngineTypeN1),
    }
    impl Default for VirtualMachineEngineType {
        fn default() -> Self {
            Self::V010002(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineEngineTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineEngineTypes {
        #[default]
        #[serde(rename = "HyperV")]
        HyperV,
        #[serde(rename = "KVM")]
        KVM,
        #[serde(rename = "PowerVM")]
        PowerVM,
        #[serde(rename = "QEMU")]
        QEMU,
        #[serde(rename = "VMwareESX")]
        VMwareESX,
        #[serde(rename = "VirtualBox")]
        VirtualBox,
        #[serde(rename = "Xen")]
        Xen,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualMachineImageTypes {
        #[default]
        #[serde(rename = "OVA")]
        OVA,
        #[serde(rename = "OVF")]
        OVF,
        #[serde(rename = "QCOW")]
        QCOW,
        #[serde(rename = "QCOW2")]
        QCOW2,
        #[serde(rename = "Raw")]
        Raw,
        #[serde(rename = "VDI")]
        VDI,
        #[serde(rename = "VHD")]
        VHD,
        #[serde(rename = "VMDK")]
        VMDK,
    }
}
