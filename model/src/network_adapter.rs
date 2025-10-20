pub type NetworkAdapter = crate::network_adapter::v1_13_0::NetworkAdapter;
pub mod v1_12_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#NetworkAdapter.Reset"
        )]
        pub network_adapter_reset: Option<crate::network_adapter::v1_12_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#NetworkAdapter.ResetSettingsToDefault"
        )]
        pub network_adapter_reset_settings_to_default:
            Option<crate::network_adapter::v1_12_1::ResetSettingsToDefault>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_adapter::v1_12_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControllerCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataCenterBridging")]
        pub data_center_bridging: Option<crate::network_adapter::v1_12_1::DataCenterBridging>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctionCount"
        )]
        pub network_device_function_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPortCount")]
        pub network_port_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NPAR")]
        pub npar: Option<crate::network_adapter::v1_12_1::NicPartitioning>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NPIV")]
        pub npiv: Option<crate::network_adapter::v1_12_1::NPIV>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualizationOffload"
        )]
        pub virtualization_offload: Option<crate::network_adapter::v1_12_1::VirtualizationOffload>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControllerLinks {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPorts")]
        pub network_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkPorts@odata.count"
        )]
        pub network_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Controllers {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ControllerCapabilities"
        )]
        pub controller_capabilities:
            Option<crate::network_adapter::v1_12_1::ControllerCapabilities>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwarePackageVersion"
        )]
        pub firmware_package_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::network_adapter::v1_12_1::ControllerLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataCenterBridging {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capable")]
        pub capable: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NPIV {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxDeviceLogins")]
        pub max_device_logins: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPortLogins")]
        pub max_port_logins: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_adapter::v1_12_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<Vec<crate::network_adapter::v1_12_1::Controllers>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::network_adapter::v1_12_1::NetworkAdapterDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::network_adapter::v1_12_1::NetworkAdapterMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPorts")]
        pub network_ports: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NetworkAdapterDescription {
        V000001(crate::network_adapter::v1_12_1::NetworkAdapterDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for NetworkAdapterDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkAdapterDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NetworkAdapterMetrics {
        V000001(crate::network_adapter::v1_12_1::NetworkAdapterMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for NetworkAdapterMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkAdapterMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NicPartitioning {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NparCapable")]
        pub npar_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NparEnabled")]
        pub npar_enabled: Option<bool>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetSettingsToDefault {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetSettingsToDefaultRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SRIOV {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOVEnabled")]
        pub sriov_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOVVEPACapable")]
        pub sriovvepa_capable: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualFunction {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceMaxCount")]
        pub device_max_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAssignmentGroupSize"
        )]
        pub min_assignment_group_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkPortMaxCount"
        )]
        pub network_port_max_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualizationOffload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOV")]
        pub sriov: Option<crate::network_adapter::v1_12_1::SRIOV>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualFunction")]
        pub virtual_function: Option<crate::network_adapter::v1_12_1::VirtualFunction>,
    }
}
pub mod v1_13_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#NetworkAdapter.Reset"
        )]
        pub network_adapter_reset: Option<crate::network_adapter::v1_13_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#NetworkAdapter.ResetSettingsToDefault"
        )]
        pub network_adapter_reset_settings_to_default:
            Option<crate::network_adapter::v1_13_0::ResetSettingsToDefault>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_adapter::v1_13_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControllerCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataCenterBridging")]
        pub data_center_bridging: Option<crate::network_adapter::v1_13_0::DataCenterBridging>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctionCount"
        )]
        pub network_device_function_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPortCount")]
        pub network_port_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NPAR")]
        pub npar: Option<crate::network_adapter::v1_13_0::NicPartitioning>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NPIV")]
        pub npiv: Option<crate::network_adapter::v1_13_0::NPIV>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualizationOffload"
        )]
        pub virtualization_offload: Option<crate::network_adapter::v1_13_0::VirtualizationOffload>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ControllerLinks {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPorts")]
        pub network_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkPorts@odata.count"
        )]
        pub network_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Controllers {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ControllerCapabilities"
        )]
        pub controller_capabilities:
            Option<crate::network_adapter::v1_13_0::ControllerCapabilities>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwarePackageVersion"
        )]
        pub firmware_package_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::network_adapter::v1_13_0::ControllerLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataCenterBridging {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Capable")]
        pub capable: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NPIV {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxDeviceLogins")]
        pub max_device_logins: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPortLogins")]
        pub max_port_logins: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_adapter::v1_13_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<Vec<crate::network_adapter::v1_13_0::Controllers>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::network_adapter::v1_13_0::NetworkAdapterDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::network_adapter::v1_13_0::NetworkAdapterMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkPorts")]
        pub network_ports: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortSplitting")]
        pub port_splitting: Option<crate::network_adapter::v1_13_0::NetworkAdapterPortSplitting>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NetworkAdapterDescription {
        V000001(crate::network_adapter::v1_13_0::NetworkAdapterDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for NetworkAdapterDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkAdapterDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NetworkAdapterMetrics {
        V000001(crate::network_adapter::v1_13_0::NetworkAdapterMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for NetworkAdapterMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkAdapterMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NetworkAdapterPortSplitting {
        V011300(crate::network_adapter::v1_13_0::PortSplitting),
        V000001(crate::network_adapter::v1_13_0::NetworkAdapterPortSplittingN1),
    }
    impl Default for NetworkAdapterPortSplitting {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkAdapterPortSplittingN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NicPartitioning {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NparCapable")]
        pub npar_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NparEnabled")]
        pub npar_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortSplitting {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentConfiguration"
        )]
        pub current_configuration:
            Option<Vec<crate::network_adapter::v1_13_0::PortSplittingSubconfiguration>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumSubports")]
        pub maximum_subports: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaximumSubportsPerPort"
        )]
        pub maximum_subports_per_port: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedConfigurations"
        )]
        pub supported_configurations:
            Option<Vec<crate::network_adapter::v1_13_0::PortSplittingSubconfigurationList>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortSplittingSubconfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndingPhysicalPort")]
        pub ending_physical_port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirstSubportId")]
        pub first_subport_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedGbps")]
        pub link_speed_gbps: Option<Vec<i64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StartingPhysicalPort"
        )]
        pub starting_physical_port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubportsPerPort")]
        pub subports_per_port: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortSplittingSubconfigurationList {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subconfigurations")]
        pub subconfigurations:
            Option<Vec<crate::network_adapter::v1_13_0::PortSplittingSubconfiguration>>,
    }
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetSettingsToDefault {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetSettingsToDefaultRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SRIOV {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOVEnabled")]
        pub sriov_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOVVEPACapable")]
        pub sriovvepa_capable: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualFunction {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceMaxCount")]
        pub device_max_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAssignmentGroupSize"
        )]
        pub min_assignment_group_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkPortMaxCount"
        )]
        pub network_port_max_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualizationOffload {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SRIOV")]
        pub sriov: Option<crate::network_adapter::v1_13_0::SRIOV>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualFunction")]
        pub virtual_function: Option<crate::network_adapter::v1_13_0::VirtualFunction>,
    }
}
