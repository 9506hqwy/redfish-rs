pub mod v1_18_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::processor::v1_18_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Processor.Reset")]
        pub processor_reset: Option<crate::processor::v1_18_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Processor.ResetToDefaults"
        )]
        pub processor_reset_to_defaults: Option<crate::processor::v1_18_0::ResetToDefaults>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseSpeedPriorityState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EthernetInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLanes")]
        pub max_lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedMbps")]
        pub max_speed_mbps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FPGA {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExternalInterfaces")]
        pub external_interfaces: Option<Vec<crate::processor::v1_18_0::ProcessorInterface>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareId")]
        pub firmware_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareManufacturer"
        )]
        pub firmware_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FpgaType")]
        pub fpga_type: Option<crate::processor::v1_18_0::FpgaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterface")]
        pub host_interface: Option<crate::processor::v1_18_0::ProcessorInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeVirtualFunctions"
        )]
        pub pcie_virtual_functions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProgrammableFromHost"
        )]
        pub programmable_from_host: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReconfigurationSlots"
        )]
        pub reconfiguration_slots: Option<Vec<crate::processor::v1_18_0::FpgaReconfigurationSlot>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FpgaReconfigurationSlot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccelerationFunction"
        )]
        pub acceleration_function: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProgrammableFromHost"
        )]
        pub programmable_from_host: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotId")]
        pub slot_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FpgaType {
        #[default]
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Integrated")]
        Integrated,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InstructionSet {
        #[default]
        #[serde(rename = "ARM-A32")]
        ARMA32,
        #[serde(rename = "ARM-A64")]
        ARMA64,
        #[serde(rename = "IA-64")]
        IAN64,
        #[serde(rename = "MIPS32")]
        MIPS32,
        #[serde(rename = "MIPS64")]
        MIPS64,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerISA")]
        PowerISA,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "x86-64")]
        X86N64,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedProcessors"
        )]
        pub connected_processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedProcessors@odata.count"
        )]
        pub connected_processors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricAdapters@odata.count"
        )]
        pub fabric_adapters_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicsController")]
        pub graphics_controller: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ECCModeEnabled")]
        pub ecc_mode_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalCacheSizeMiB")]
        pub total_cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalMemorySizeMiB")]
        pub total_memory_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Processor {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccelerationFunctions"
        )]
        pub acceleration_functions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::processor::v1_18_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalFirmwareVersions"
        )]
        pub additional_firmware_versions: Option<crate::software_inventory::AdditionalVersions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AppliedOperatingConfig"
        )]
        pub applied_operating_config: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseSpeedMHz")]
        pub base_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseSpeedPriorityState"
        )]
        pub base_speed_priority_state: Option<crate::processor::v1_18_0::BaseSpeedPriorityState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Family")]
        pub family: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FPGA")]
        pub fpga: Option<crate::processor::v1_18_0::FPGA>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HighSpeedCoreIDs")]
        pub high_speed_core_ids: Option<Vec<i64>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InstructionSet")]
        pub instruction_set: Option<crate::processor::v1_18_0::InstructionSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::processor::v1_18_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedMHz")]
        pub max_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPWatts")]
        pub max_tdp_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::processor::v1_18_0::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSpeedMHz")]
        pub min_speed_mhz: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingConfigs")]
        pub operating_configs: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProcessorArchitecture"
        )]
        pub processor_architecture: Option<crate::processor::v1_18_0::ProcessorArchitecture>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorId")]
        pub processor_id: Option<crate::processor::v1_18_0::ProcessorId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorIndex")]
        pub processor_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorMemory")]
        pub processor_memory: Option<Vec<crate::processor::v1_18_0::ProcessorMemory>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorType")]
        pub processor_type: Option<crate::processor::v1_18_0::ProcessorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replaceable")]
        pub replaceable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedLimitMHz")]
        pub speed_limit_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedLocked")]
        pub speed_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubProcessors")]
        pub sub_processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemInterface")]
        pub system_interface: Option<crate::processor::v1_18_0::ProcessorInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TDPWatts")]
        pub tdp_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThrottleCauses")]
        pub throttle_causes: Option<Vec<crate::processor::v1_18_0::ThrottleCause>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Throttled")]
        pub throttled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalCores")]
        pub total_cores: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalEnabledCores")]
        pub total_enabled_cores: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TotalThreads")]
        pub total_threads: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TurboState")]
        pub turbo_state: Option<crate::processor::v1_18_0::TurboState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ProcessorArchitecture {
        #[default]
        #[serde(rename = "ARM")]
        ARM,
        #[serde(rename = "IA-64")]
        IAN64,
        #[serde(rename = "MIPS")]
        MIPS,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "x86")]
        X86,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EffectiveFamily")]
        pub effective_family: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EffectiveModel")]
        pub effective_model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentificationRegisters"
        )]
        pub identification_registers: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MicrocodeInfo")]
        pub microcode_info: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtectedIdentificationNumber"
        )]
        pub protected_identification_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Step")]
        pub step: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::processor::v1_18_0::EthernetInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::processor::v1_18_0::SystemInterfaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIe")]
        pub pcie: Option<crate::pcie_device::PCIeInterface>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorMemory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntegratedMemory")]
        pub integrated_memory: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::processor::v1_18_0::ProcessorMemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedMHz")]
        pub speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ProcessorMemoryType {
        #[default]
        #[serde(rename = "Cache")]
        Cache,
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "Flash")]
        Flash,
        #[serde(rename = "GDDR")]
        GDDR,
        #[serde(rename = "GDDR2")]
        GDDR2,
        #[serde(rename = "GDDR3")]
        GDDR3,
        #[serde(rename = "GDDR4")]
        GDDR4,
        #[serde(rename = "GDDR5")]
        GDDR5,
        #[serde(rename = "GDDR5X")]
        GDDR5X,
        #[serde(rename = "GDDR6")]
        GDDR6,
        #[serde(rename = "HBM1")]
        HBM1,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM2E")]
        HBM2E,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "L1Cache")]
        L1Cache,
        #[serde(rename = "L2Cache")]
        L2Cache,
        #[serde(rename = "L3Cache")]
        L3Cache,
        #[serde(rename = "L4Cache")]
        L4Cache,
        #[serde(rename = "L5Cache")]
        L5Cache,
        #[serde(rename = "L6Cache")]
        L6Cache,
        #[serde(rename = "L7Cache")]
        L7Cache,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SDRAM")]
        SDRAM,
        #[serde(rename = "SGRAM")]
        SGRAM,
        #[serde(rename = "SRAM")]
        SRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ProcessorType {
        #[default]
        #[serde(rename = "Accelerator")]
        Accelerator,
        #[serde(rename = "CPU")]
        CPU,
        #[serde(rename = "Core")]
        Core,
        #[serde(rename = "DSP")]
        DSP,
        #[serde(rename = "FPGA")]
        FPGA,
        #[serde(rename = "GPU")]
        GPU,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Thread")]
        Thread,
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
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemInterfaceType {
        #[default]
        #[serde(rename = "AMBA")]
        AMBA,
        #[serde(rename = "CCIX")]
        CCIX,
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PCIe")]
        PCIe,
        #[serde(rename = "QPI")]
        QPI,
        #[serde(rename = "UPI")]
        UPI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThrottleCause {
        #[default]
        #[serde(rename = "ClockLimit")]
        ClockLimit,
        #[serde(rename = "ManagementDetectedFault")]
        ManagementDetectedFault,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerLimit")]
        PowerLimit,
        #[serde(rename = "ThermalLimit")]
        ThermalLimit,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TurboState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
    }
}
