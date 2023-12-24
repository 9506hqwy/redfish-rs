use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PCIeDevice {
    PCIeDeviceV1N0N7PCIeDevice(crate::pcie_device::v1_0_7::PCIeDevice),
    PCIeDeviceV1N10N1PCIeDevice(crate::pcie_device::v1_10_1::PCIeDevice),
    PCIeDeviceV1N11N1PCIeDevice(crate::pcie_device::v1_11_1::PCIeDevice),
    PCIeDeviceV1N1N5PCIeDevice(crate::pcie_device::v1_1_5::PCIeDevice),
    PCIeDeviceV1N2N5PCIeDevice(crate::pcie_device::v1_2_5::PCIeDevice),
    PCIeDeviceV1N3N4PCIeDevice(crate::pcie_device::v1_3_4::PCIeDevice),
    PCIeDeviceV1N4N2PCIeDevice(crate::pcie_device::v1_4_2::PCIeDevice),
    PCIeDeviceV1N5N2PCIeDevice(crate::pcie_device::v1_5_2::PCIeDevice),
    PCIeDeviceV1N6N2PCIeDevice(crate::pcie_device::v1_6_2::PCIeDevice),
    PCIeDeviceV1N7N1PCIeDevice(crate::pcie_device::v1_7_1::PCIeDevice),
    PCIeDeviceV1N8N1PCIeDevice(crate::pcie_device::v1_8_1::PCIeDevice),
    PCIeDeviceV1N9N1PCIeDevice(crate::pcie_device::v1_9_1::PCIeDevice),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PCIeErrors {
    PCIeDeviceV1N10N1PCIeErrors(crate::pcie_device::v1_10_1::PCIeErrors),
    PCIeDeviceV1N11N1PCIeErrors(crate::pcie_device::v1_11_1::PCIeErrors),
    PCIeDeviceV1N8N1PCIeErrors(crate::pcie_device::v1_8_1::PCIeErrors),
    PCIeDeviceV1N9N1PCIeErrors(crate::pcie_device::v1_9_1::PCIeErrors),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PCIeInterface {
    PCIeDeviceV1N10N1PCIeInterface(crate::pcie_device::v1_10_1::PCIeInterface),
    PCIeDeviceV1N11N1PCIeInterface(crate::pcie_device::v1_11_1::PCIeInterface),
    PCIeDeviceV1N3N4PCIeInterface(crate::pcie_device::v1_3_4::PCIeInterface),
    PCIeDeviceV1N4N2PCIeInterface(crate::pcie_device::v1_4_2::PCIeInterface),
    PCIeDeviceV1N5N2PCIeInterface(crate::pcie_device::v1_5_2::PCIeInterface),
    PCIeDeviceV1N6N2PCIeInterface(crate::pcie_device::v1_6_2::PCIeInterface),
    PCIeDeviceV1N7N1PCIeInterface(crate::pcie_device::v1_7_1::PCIeInterface),
    PCIeDeviceV1N8N1PCIeInterface(crate::pcie_device::v1_8_1::PCIeInterface),
    PCIeDeviceV1N9N1PCIeInterface(crate::pcie_device::v1_9_1::PCIeInterface),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PCIeTypes {
    #[default]
    #[serde(rename = "Gen1")]
    Gen1,
    #[serde(rename = "Gen2")]
    Gen2,
    #[serde(rename = "Gen3")]
    Gen3,
    #[serde(rename = "Gen4")]
    Gen4,
    #[serde(rename = "Gen5")]
    Gen5,
}
pub mod v1_0_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_0_7::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_0_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_10_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_10_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Retimer")]
        Retimer,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LaneSplittingType {
        #[default]
        #[serde(rename = "Bifurcated")]
        Bifurcated,
        #[serde(rename = "Bridged")]
        Bridged,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Switch")]
        pub switch: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_10_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_10_1::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_10_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_10_1::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<crate::pcie_device::v1_10_1::Slot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeErrors {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableErrorCount"
        )]
        pub correctable_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FatalErrorCount")]
        pub fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "L0ToRecoveryCount")]
        pub l0_to_recovery_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKReceivedCount")]
        pub nak_received_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKSentCount")]
        pub nak_sent_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonFatalErrorCount")]
        pub non_fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplayCount")]
        pub replay_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplayRolloverCount"
        )]
        pub replay_rollover_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Slot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LaneSplitting")]
        pub lane_splitting: Option<crate::pcie_device::v1_10_1::LaneSplittingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_device::v1_10_1::SlotType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotType {
        #[default]
        #[serde(rename = "FullLength")]
        FullLength,
        #[serde(rename = "HalfLength")]
        HalfLength,
        #[serde(rename = "LowProfile")]
        LowProfile,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "Mini")]
        Mini,
        #[serde(rename = "OCP3Large")]
        OCP3Large,
        #[serde(rename = "OCP3Small")]
        OCP3Small,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "U2")]
        U2,
    }
}
pub mod v1_11_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_11_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_11_1::CXLDeviceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressPortCongestionSupport"
        )]
        pub egress_port_congestion_support: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxNumberLogicalDevices"
        )]
        pub max_number_logical_devices: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThroughputReductionSupport"
        )]
        pub throughput_reduction_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDeviceType {
        #[default]
        #[serde(rename = "Type1")]
        Type1,
        #[serde(rename = "Type2")]
        Type2,
        #[serde(rename = "Type3")]
        Type3,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Retimer")]
        Retimer,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LaneSplittingType {
        #[default]
        #[serde(rename = "Bifurcated")]
        Bifurcated,
        #[serde(rename = "Bridged")]
        Bridged,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Switch")]
        pub switch: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_11_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLDevice")]
        pub cxl_device: Option<crate::pcie_device::v1_11_1::CXLDevice>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevices")]
        pub cxl_logical_devices: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_11_1::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_11_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_11_1::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<crate::pcie_device::v1_11_1::Slot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StagedVersion")]
        pub staged_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeErrors {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableErrorCount"
        )]
        pub correctable_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FatalErrorCount")]
        pub fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "L0ToRecoveryCount")]
        pub l0_to_recovery_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKReceivedCount")]
        pub nak_received_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKSentCount")]
        pub nak_sent_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonFatalErrorCount")]
        pub non_fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplayCount")]
        pub replay_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplayRolloverCount"
        )]
        pub replay_rollover_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Slot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LaneSplitting")]
        pub lane_splitting: Option<crate::pcie_device::v1_11_1::LaneSplittingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_device::v1_11_1::SlotType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotType {
        #[default]
        #[serde(rename = "FullLength")]
        FullLength,
        #[serde(rename = "HalfLength")]
        HalfLength,
        #[serde(rename = "LowProfile")]
        LowProfile,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "Mini")]
        Mini,
        #[serde(rename = "OCP3Large")]
        OCP3Large,
        #[serde(rename = "OCP3Small")]
        OCP3Small,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "U2")]
        U2,
    }
}
pub mod v1_1_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_1_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_1_5::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_1_5::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_1_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_2_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_2_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_2_5::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_2_5::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_2_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_3_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_3_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_3_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_3_4::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_3_4::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_3_4::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_4_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_4_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_4_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_4_2::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_4_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_4_2::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_5_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_5_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_5_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_5_2::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_5_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_5_2::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_6_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_6_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_6_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_6_2::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_6_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_6_2::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_7_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_7_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_7_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_7_1::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_7_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_7_1::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_8_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_8_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_8_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_8_1::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_8_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_8_1::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeErrors {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableErrorCount"
        )]
        pub correctable_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FatalErrorCount")]
        pub fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "L0ToRecoveryCount")]
        pub l0_to_recovery_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKReceivedCount")]
        pub nak_received_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKSentCount")]
        pub nak_sent_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonFatalErrorCount")]
        pub non_fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplayCount")]
        pub replay_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplayRolloverCount"
        )]
        pub replay_rollover_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
}
pub mod v1_9_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_9_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DeviceType {
        #[default]
        #[serde(rename = "MultiFunction")]
        MultiFunction,
        #[serde(rename = "Simulated")]
        Simulated,
        #[serde(rename = "SingleFunction")]
        SingleFunction,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LaneSplittingType {
        #[default]
        #[serde(rename = "Bifurcated")]
        Bifurcated,
        #[serde(rename = "Bridged")]
        Bridged,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_9_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_9_1::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_9_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::v1_9_1::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<crate::pcie_device::v1_9_1::Slot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeErrors {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableErrorCount"
        )]
        pub correctable_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FatalErrorCount")]
        pub fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "L0ToRecoveryCount")]
        pub l0_to_recovery_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKReceivedCount")]
        pub nak_received_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NAKSentCount")]
        pub nak_sent_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonFatalErrorCount")]
        pub non_fatal_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplayCount")]
        pub replay_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplayRolloverCount"
        )]
        pub replay_rollover_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Slot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LaneSplitting")]
        pub lane_splitting: Option<crate::pcie_device::v1_9_1::LaneSplittingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_device::v1_9_1::SlotType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotType {
        #[default]
        #[serde(rename = "FullLength")]
        FullLength,
        #[serde(rename = "HalfLength")]
        HalfLength,
        #[serde(rename = "LowProfile")]
        LowProfile,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "Mini")]
        Mini,
        #[serde(rename = "OCP3Large")]
        OCP3Large,
        #[serde(rename = "OCP3Small")]
        OCP3Small,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "U2")]
        U2,
    }
}
