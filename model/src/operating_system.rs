pub type OperatingSystem = crate::operating_system::v1_0_0::OperatingSystem;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::operating_system::v1_0_0::OemActions>,
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
        pub r#type: Option<crate::operating_system::v1_0_0::ContainerEngineTypes>,
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
        pub actions: Option<crate::operating_system::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Applications")]
        pub applications: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerEngines")]
        pub container_engines: Option<Vec<crate::operating_system::v1_0_0::ContainerEngine>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainerImages")]
        pub container_images: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Containers")]
        pub containers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Kernel")]
        pub kernel: Option<crate::operating_system::v1_0_0::Kernel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::operating_system::v1_0_0::Links>,
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
        pub r#type: Option<crate::operating_system::v1_0_0::OperatingSystemTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UptimeSeconds")]
        pub uptime_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualMachineEngines"
        )]
        pub virtual_machine_engines:
            Option<Vec<crate::operating_system::v1_0_0::VirtualMachineEngine>>,
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
            Option<Vec<crate::operating_system::v1_0_0::VirtualMachineImageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::operating_system::v1_0_0::VirtualMachineEngineTypes>,
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
