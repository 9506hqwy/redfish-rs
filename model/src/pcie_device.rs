use serde::{Deserialize, Serialize};
pub type PCIeDevice = crate::pcie_device::v1_16_0::PCIeDevice;
pub type PCIeErrors = crate::pcie_device::v1_16_0::PCIeErrors;
pub type PCIeInterface = crate::pcie_device::v1_16_0::PCIeInterface;
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
    #[serde(rename = "Gen6")]
    Gen6,
}
pub mod v1_16_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_device::v1_16_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_16_0::CXLDeviceDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DynamicCapacity")]
        pub dynamic_capacity: Option<crate::pcie_device::v1_16_0::CXLDeviceDynamicCapacity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressPortCongestionSupport"
        )]
        pub egress_port_congestion_support: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxNumberLogicalDevices"
        )]
        pub max_number_logical_devices: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionEnabled"
        )]
        pub temporary_throughput_reduction_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionSupported"
        )]
        pub temporary_throughput_reduction_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThroughputReductionSupport"
        )]
        pub throughput_reduction_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
        pub timestamp: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDeviceDeviceType {
        V011600(crate::pcie_device::v1_16_0::CXLDeviceType),
        V000001(crate::pcie_device::v1_16_0::CXLDeviceDeviceTypeN1),
    }
    impl Default for CXLDeviceDeviceType {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDeviceDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDeviceDynamicCapacity {
        V011600(crate::pcie_device::v1_16_0::CXLDynamicCapacity),
        V000001(crate::pcie_device::v1_16_0::CXLDeviceDynamicCapacityN1),
    }
    impl Default for CXLDeviceDynamicCapacity {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDeviceDynamicCapacityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub add_capacity_policies_supported: Option<
            Vec<crate::pcie_device::v1_16_0::CXLDynamicCapacityAddCapacityPoliciesSupported>,
        >,
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
            Option<Vec<crate::pcie_device::v1_16_0::CXLDynamicCapacityMemoryBlockSizesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReleaseCapacityPoliciesSupported"
        )]
        pub release_capacity_policies_supported: Option<
            Vec<crate::pcie_device::v1_16_0::CXLDynamicCapacityReleaseCapacityPoliciesSupported>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SanitizationOnReleaseSupport"
        )]
        pub sanitization_on_release_support: Option<
            Vec<crate::pcie_device::v1_16_0::CXLDynamicCapacitySanitizationOnReleaseSupport>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalDynamicCapacityMiB"
        )]
        pub total_dynamic_capacity_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDynamicCapacityAddCapacityPoliciesSupported {
        V011600(crate::pcie_device::v1_16_0::CXLDynamicCapacityPolicies),
        V000001(crate::pcie_device::v1_16_0::CXLDynamicCapacityAddCapacityPoliciesSupportedN1),
    }
    impl Default for CXLDynamicCapacityAddCapacityPoliciesSupported {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDynamicCapacityAddCapacityPoliciesSupportedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDynamicCapacityMemoryBlockSizesSupported {
        V011600(crate::pcie_device::v1_16_0::CXLRegionBlockSizes),
        V000001(crate::pcie_device::v1_16_0::CXLDynamicCapacityMemoryBlockSizesSupportedN1),
    }
    impl Default for CXLDynamicCapacityMemoryBlockSizesSupported {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDynamicCapacityMemoryBlockSizesSupportedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDynamicCapacityReleaseCapacityPoliciesSupported {
        V011600(crate::pcie_device::v1_16_0::CXLDynamicCapacityPolicies),
        V000001(crate::pcie_device::v1_16_0::CXLDynamicCapacityReleaseCapacityPoliciesSupportedN1),
    }
    impl Default for CXLDynamicCapacityReleaseCapacityPoliciesSupported {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDynamicCapacityReleaseCapacityPoliciesSupportedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLDynamicCapacitySanitizationOnReleaseSupport {
        V011600(crate::pcie_device::v1_16_0::CXLRegionSanitization),
        V000001(crate::pcie_device::v1_16_0::CXLDynamicCapacitySanitizationOnReleaseSupportN1),
    }
    impl Default for CXLDynamicCapacitySanitizationOnReleaseSupport {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLDynamicCapacitySanitizationOnReleaseSupportN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub switch: Option<crate::pcie_device::v1_16_0::LinksSwitch>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksSwitch {
        V000001(crate::pcie_device::v1_16_0::LinksSwitchN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksSwitch {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksSwitchN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_device::v1_16_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLDevice")]
        pub cxl_device: Option<crate::pcie_device::v1_16_0::PCIeDeviceCXLDevice>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevices")]
        pub cxl_logical_devices: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::pcie_device::v1_16_0::PCIeDeviceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::pcie_device::v1_16_0::DeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_device::v1_16_0::Links>,
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
        pub pcie_interface: Option<crate::pcie_device::v1_16_0::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadyToRemove")]
        pub ready_to_remove: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<crate::pcie_device::v1_16_0::PCIeDeviceSlot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StagedVersion")]
        pub staged_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::pcie_device::v1_16_0::PCIeDeviceUUID>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeDeviceCXLDevice {
        V011600(crate::pcie_device::v1_16_0::CXLDevice),
        V000001(crate::pcie_device::v1_16_0::PCIeDeviceCXLDeviceN1),
    }
    impl Default for PCIeDeviceCXLDevice {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeDeviceCXLDeviceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeDeviceDescription {
        V000001(crate::pcie_device::v1_16_0::PCIeDeviceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PCIeDeviceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeDeviceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeDeviceSlot {
        V011600(crate::pcie_device::v1_16_0::Slot),
        V000001(crate::pcie_device::v1_16_0::PCIeDeviceSlotN1),
    }
    impl Default for PCIeDeviceSlot {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeDeviceSlotN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeDeviceUUID {
        V000001(crate::pcie_device::v1_16_0::PCIeDeviceUUIDN1),
        ResourceUUID(String),
    }
    impl Default for PCIeDeviceUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeDeviceUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeErrors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BadDLLPCount")]
        pub bad_dllp_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BadTLPCount")]
        pub bad_tlp_count: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnsupportedRequestCount"
        )]
        pub unsupported_request_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LanesInUse")]
        pub lanes_in_use: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPCIeType")]
        pub max_pcie_type: Option<crate::pcie_device::v1_16_0::PCIeInterfaceMaxPCIeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::v1_16_0::PCIeInterfacePCIeType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeInterfaceMaxPCIeType {
        V000001(crate::pcie_device::v1_16_0::PCIeInterfaceMaxPCIeTypeN1),
        PCIeDevicePCIeTypes(crate::pcie_device::PCIeTypes),
    }
    impl Default for PCIeInterfaceMaxPCIeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeInterfaceMaxPCIeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeInterfacePCIeType {
        V000001(crate::pcie_device::v1_16_0::PCIeInterfacePCIeTypeN1),
        PCIeDevicePCIeTypes(crate::pcie_device::PCIeTypes),
    }
    impl Default for PCIeInterfacePCIeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeInterfacePCIeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Slot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LaneSplitting")]
        pub lane_splitting: Option<crate::pcie_device::v1_16_0::SlotLaneSplitting>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::v1_16_0::SlotPCIeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_device::v1_16_0::SlotSlotType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SlotLaneSplitting {
        V011600(crate::pcie_device::v1_16_0::LaneSplittingType),
        V000001(crate::pcie_device::v1_16_0::SlotLaneSplittingN1),
    }
    impl Default for SlotLaneSplitting {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotLaneSplittingN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SlotPCIeType {
        V000001(crate::pcie_device::v1_16_0::SlotPCIeTypeN1),
        PCIeDevicePCIeTypes(crate::pcie_device::PCIeTypes),
    }
    impl Default for SlotPCIeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotPCIeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SlotSlotType {
        V011600(crate::pcie_device::v1_16_0::SlotType),
        V000001(crate::pcie_device::v1_16_0::SlotSlotTypeN1),
    }
    impl Default for SlotSlotType {
        fn default() -> Self {
            Self::V011600(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotSlotTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
