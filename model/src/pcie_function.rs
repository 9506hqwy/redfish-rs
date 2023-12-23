pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_function::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceClass {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "CommunicationController")]
        CommunicationController,
        #[serde(rename = "Coprocessor")]
        Coprocessor,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "DockingStation")]
        DockingStation,
        #[serde(rename = "EncryptionController")]
        EncryptionController,
        #[serde(rename = "GenericSystemPeripheral")]
        GenericSystemPeripheral,
        #[serde(rename = "InputDeviceController")]
        InputDeviceController,
        #[serde(rename = "IntelligentController")]
        IntelligentController,
        #[serde(rename = "MassStorageController")]
        MassStorageController,
        #[serde(rename = "MemoryController")]
        MemoryController,
        #[serde(rename = "MultimediaController")]
        MultimediaController,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "NonEssentialInstrumentation")]
        NonEssentialInstrumentation,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "ProcessingAccelerators")]
        ProcessingAccelerators,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "SatelliteCommunicationsController")]
        SatelliteCommunicationsController,
        #[serde(rename = "SerialBusController")]
        SerialBusController,
        #[serde(rename = "SignalProcessingController")]
        SignalProcessingController,
        #[serde(rename = "UnassignedClass")]
        UnassignedClass,
        #[serde(rename = "UnclassifiedDevice")]
        UnclassifiedDevice,
        #[serde(rename = "WirelessController")]
        WirelessController,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FunctionProtocol {
        #[default]
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "PCIe")]
        PCIe,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FunctionType {
        #[default]
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "Virtual")]
        Virtual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevice")]
        pub cxl_logical_device: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevice")]
        pub pcie_device: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processor")]
        pub processor: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageControllers")]
        pub storage_controllers: Option<Vec<crate::storage::StorageController>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StorageControllers@odata.count"
        )]
        pub storage_controllers_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeFunction {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_function::v1_5_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceClass")]
        pub device_class: Option<crate::pcie_function::v1_5_0::DeviceClass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionId")]
        pub function_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionProtocol")]
        pub function_protocol: Option<crate::pcie_function::v1_5_0::FunctionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionType")]
        pub function_type: Option<crate::pcie_function::v1_5_0::FunctionType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_function::v1_5_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RevisionId")]
        pub revision_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
