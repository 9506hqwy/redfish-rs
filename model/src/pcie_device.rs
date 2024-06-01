use serde::{Deserialize, Serialize};
pub type PCIeDevice = crate::pcie_device::v1_12_0::PCIeDevice;
pub type PCIeErrors = crate::pcie_device::v1_12_0::PCIeErrors;
pub type PCIeInterface = crate::pcie_device::v1_12_0::PCIeInterface;
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
pub mod v1_12_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_12_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_12_0::CXLDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DynamicCapacity")]
        pub dynamic_capacity: Option<crate::pcie_device::v1_12_0::CXLDynamicCapacity>,
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
    pub struct CXLDynamicCapacity {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddCapacityPoliciesSupported"
        )]
        pub add_capacity_policies_supported:
            Option<Vec<crate::pcie_device::v1_12_0::CXLDynamicCapacityPolicies>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxDynamicCapacityRegions"
        )]
        pub max_dynamic_capacity_regions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxHosts")]
        pub max_hosts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryBlockSizesSupported"
        )]
        pub memory_block_sizes_supported:
            Option<Vec<crate::pcie_device::v1_12_0::CXLRegionBlockSizes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReleaseCapacityPoliciesSupported"
        )]
        pub release_capacity_policies_supported:
            Option<Vec<crate::pcie_device::v1_12_0::CXLDynamicCapacityPolicies>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SanitizationOnReleaseSupport"
        )]
        pub sanitization_on_release_support:
            Option<Vec<crate::pcie_device::v1_12_0::CXLRegionSanitization>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalDynamicCapacityMiB"
        )]
        pub total_dynamic_capacity_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDynamicCapacityPolicies {
        #[default]
        #[serde(rename = "Contiguous")]
        Contiguous,
        #[serde(rename = "Free")]
        Free,
        #[serde(rename = "Prescriptive")]
        Prescriptive,
        #[serde(rename = "TagBased")]
        TagBased,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLRegionBlockSizes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeMiB")]
        pub block_size_mib: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionNumber")]
        pub region_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLRegionSanitization {
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionNumber")]
        pub region_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SanitizationOnReleaseSupported"
        )]
        pub sanitization_on_release_supported: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Switch")]
        pub switch: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_12_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLDevice")]
        pub cxl_device: Option<crate::pcie_device::v1_12_0::CXLDevice>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevices")]
        pub cxl_logical_devices: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_12_0::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_12_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
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
        pub pcie_interface: Option<crate::pcie_device::v1_12_0::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<crate::pcie_device::v1_12_0::Slot>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LaneSplitting")]
        pub lane_splitting: Option<crate::pcie_device::v1_12_0::LaneSplittingType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_device::v1_12_0::SlotType>,
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
