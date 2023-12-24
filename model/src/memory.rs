use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Memory {
    V011701(crate::memory::v1_17_1::Memory),
    V011601(crate::memory::v1_16_1::Memory),
    V011501(crate::memory::v1_15_1::Memory),
    V011401(crate::memory::v1_14_1::Memory),
    V011302(crate::memory::v1_13_2::Memory),
    V011201(crate::memory::v1_12_1::Memory),
    V011101(crate::memory::v1_11_1::Memory),
    V011002(crate::memory::v1_10_2::Memory),
    V010905(crate::memory::v1_9_5::Memory),
    V010805(crate::memory::v1_8_5::Memory),
    V010707(crate::memory::v1_7_7::Memory),
    V010608(crate::memory::v1_6_8::Memory),
    V010509(crate::memory::v1_5_9::Memory),
    V010410(crate::memory::v1_4_10::Memory),
    V010310(crate::memory::v1_3_10::Memory),
    V010210(crate::memory::v1_2_10::Memory),
    V010111(crate::memory::v1_1_11::Memory),
    V010012(crate::memory::v1_0_12::Memory),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_0_12::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_0_12::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_0_12::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_0_12::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_0_12::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_0_12::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_0_12::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_0_12::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_0_12::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_0_12::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_0_12::MemoryMedia>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_0_12::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_0_12::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_0_12::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_0_12::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_0_12::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_0_12::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_0_12::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_1_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_1_11::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_1_11::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_1_11::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_1_11::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_1_11::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_1_11::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_1_11::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_1_11::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_1_11::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_1_11::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_1_11::MemoryMedia>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_1_11::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_1_11::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_1_11::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_1_11::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_1_11::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_1_11::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_1_11::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_2_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_2_10::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_2_10::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_2_10::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_2_10::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_2_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_2_10::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_2_10::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_2_10::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_2_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_2_10::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_2_10::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_2_10::MemoryMedia>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_2_10::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_2_10::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_2_10::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_2_10::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_2_10::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_2_10::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_2_10::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_3_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_3_10::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_3_10::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_3_10::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_3_10::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_3_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_3_10::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_3_10::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_3_10::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_3_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_3_10::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_3_10::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_3_10::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_3_10::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_3_10::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_3_10::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_3_10::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_3_10::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_3_10::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_3_10::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_4_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_4_10::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_4_10::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_4_10::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_4_10::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_4_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_4_10::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_4_10::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_4_10::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_4_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_4_10::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_4_10::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_4_10::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_4_10::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_4_10::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_4_10::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_4_10::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_4_10::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_4_10::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_4_10::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_5_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_5_9::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_5_9::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_5_9::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_5_9::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_5_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_5_9::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_5_9::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_5_9::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_5_9::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_5_9::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_5_9::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_5_9::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_5_9::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_5_9::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_5_9::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_5_9::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_5_9::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_5_9::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_5_9::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_6_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_6_8::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_6_8::OverwriteUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_6_8::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_6_8::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_6_8::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_6_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_6_8::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_6_8::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_6_8::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_6_8::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_6_8::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_6_8::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_6_8::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_6_8::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_6_8::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_6_8::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_6_8::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_6_8::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_6_8::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_6_8::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_7_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_7_7::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_7_7::OverwriteUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_7_7::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_7_7::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_7_7::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_7_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_7_7::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_7_7::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_7_7::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_7_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_7_7::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_7_7::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_7_7::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_7_7::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_7_7::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_7_7::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_7_7::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_7_7::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_7_7::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_7_7::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_7_7::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_8_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_8_5::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_8_5::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_8_5::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_8_5::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_8_5::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_8_5::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_8_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_8_5::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_8_5::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_8_5::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_8_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_8_5::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_8_5::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_8_5::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_8_5::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_8_5::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_8_5::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_8_5::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_8_5::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_8_5::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_8_5::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_8_5::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_9_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_9_5::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_9_5::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_9_5::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_9_5::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_9_5::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_9_5::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_9_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_9_5::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_9_5::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_9_5::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_9_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_9_5::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_9_5::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_9_5::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_9_5::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_9_5::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_9_5::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_9_5::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_9_5::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_9_5::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_9_5::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_9_5::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_10_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_10_2::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_10_2::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_10_2::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_10_2::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_10_2::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_10_2::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_10_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_10_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_10_2::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_10_2::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_10_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_10_2::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_10_2::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_10_2::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_10_2::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_10_2::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_10_2::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_10_2::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_10_2::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_10_2::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_10_2::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_10_2::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_11_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_11_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_11_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_11_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_11_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_11_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_11_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_11_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_11_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_11_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_11_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_11_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_11_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_11_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_11_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_11_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_11_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_11_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_11_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_11_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_11_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_11_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_11_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_12_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_12_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_12_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_12_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_12_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_12_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_12_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_12_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_12_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_12_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_12_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_12_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_12_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_12_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_12_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_12_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_12_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_12_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_12_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_12_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_12_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_12_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_12_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_13_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_13_2::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_13_2::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_13_2::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_13_2::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_13_2::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_13_2::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_13_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_13_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_13_2::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_13_2::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_13_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_13_2::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_13_2::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_13_2::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_13_2::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_13_2::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_13_2::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_13_2::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_13_2::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_13_2::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_13_2::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_13_2::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_14_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_14_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_14_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_14_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_14_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_14_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_14_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_14_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_14_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_14_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_14_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_14_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_14_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_14_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_14_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_14_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_14_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_14_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_14_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_14_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_14_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_14_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_14_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_15_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_15_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_15_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_15_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_15_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_15_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_15_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_15_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Batteries@odata.count"
        )]
        pub batteries_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_15_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_15_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_15_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_15_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_15_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_15_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_15_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_15_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_15_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_15_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_15_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_15_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_15_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_15_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_15_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_16_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_16_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_16_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_16_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.ResetToDefaults"
        )]
        pub memory_reset_to_defaults: Option<crate::memory::v1_16_1::ResetToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_16_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_16_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_16_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_16_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Batteries@odata.count"
        )]
        pub batteries_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_16_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_16_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_16_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_16_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_16_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_16_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_16_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_16_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_16_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_16_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_16_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_16_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_16_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_16_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_16_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
pub mod v1_17_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisableMasterPassphrase"
        )]
        pub memory_disable_master_passphrase:
            Option<crate::memory::v1_17_1::DisableMasterPassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.DisablePassphrase"
        )]
        pub memory_disable_passphrase: Option<crate::memory::v1_17_1::DisablePassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.FreezeSecurityState"
        )]
        pub memory_freeze_security_state: Option<crate::memory::v1_17_1::FreezeSecurityState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.InjectPersistentPoison"
        )]
        pub memory_inject_persistent_poison: Option<crate::memory::v1_17_1::InjectPersistentPoison>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.OverwriteUnit"
        )]
        pub memory_overwrite_unit: Option<crate::memory::v1_17_1::OverwriteUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.Reset")]
        pub memory_reset: Option<crate::memory::v1_17_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.ResetToDefaults"
        )]
        pub memory_reset_to_defaults: Option<crate::memory::v1_17_1::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.ScanMedia")]
        pub memory_scan_media: Option<crate::memory::v1_17_1::ScanMedia>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SecureEraseUnit"
        )]
        pub memory_secure_erase_unit: Option<crate::memory::v1_17_1::SecureEraseUnit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetMasterPassphrase"
        )]
        pub memory_set_master_passphrase: Option<crate::memory::v1_17_1::SetMasterPassphrase>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Memory.SetPassphrase"
        )]
        pub memory_set_passphrase: Option<crate::memory::v1_17_1::SetPassphrase>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Memory.UnlockUnit")]
        pub memory_unlock_unit: Option<crate::memory::v1_17_1::UnlockUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory::v1_17_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BaseModuleType {
        #[default]
        #[serde(rename = "Die")]
        Die,
        #[serde(rename = "LRDIMM")]
        LRDIMM,
        #[serde(rename = "Mini_RDIMM")]
        MiniRDIMM,
        #[serde(rename = "Mini_UDIMM")]
        MiniUDIMM,
        #[serde(rename = "RDIMM")]
        RDIMM,
        #[serde(rename = "SO_DIMM")]
        SODIMM,
        #[serde(rename = "SO_DIMM_16b")]
        SODIMM16b,
        #[serde(rename = "SO_DIMM_32b")]
        SODIMM32b,
        #[serde(rename = "SO_RDIMM_72b")]
        SORDIMM72b,
        #[serde(rename = "SO_UDIMM_72b")]
        SOUDIMM72b,
        #[serde(rename = "UDIMM")]
        UDIMM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LabelStorageSizeBytes"
        )]
        pub label_storage_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StagedNonVolatileSizeMiB"
        )]
        pub staged_non_volatile_size_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StagedVolatileSizeMiB"
        )]
        pub staged_volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisableMasterPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisableMasterPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DisablePassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ErrorCorrection {
        #[default]
        #[serde(rename = "AddressParity")]
        AddressParity,
        #[serde(rename = "MultiBitECC")]
        MultiBitECC,
        #[serde(rename = "NoECC")]
        NoECC,
        #[serde(rename = "SingleBitECC")]
        SingleBitECC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FreezeSecurityState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FreezeSecurityStateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InjectPersistentPoison {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InjectPersistentPoisonRequestBody {
        #[serde(rename = "PhysicalAddress")]
        pub physical_address: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Batteries@odata.count"
        )]
        pub batteries_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMediaSources")]
        pub memory_media_sources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryMediaSources@odata.count"
        )]
        pub memory_media_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Memory {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory::v1_17_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationAlignmentMiB"
        )]
        pub allocation_alignment_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocationIncrementMiB"
        )]
        pub allocation_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowedSpeedsMHz")]
        pub allowed_speeds_mhz: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseModuleType")]
        pub base_module_type: Option<crate::memory::v1_17_1::BaseModuleType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BusWidthBits")]
        pub bus_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSizeMiB")]
        pub cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityMiB")]
        pub capacity_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::memory::v1_17_1::CXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataWidthBits")]
        pub data_width_bits: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceID")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceLocator")]
        pub device_locator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ErrorCorrection")]
        pub error_correction: Option<crate::memory::v1_17_1::ErrorCorrection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareApiVersion")]
        pub firmware_api_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareRevision")]
        pub firmware_revision: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionClasses")]
        pub function_classes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory::v1_17_1::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsRankSpareEnabled")]
        pub is_rank_spare_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IsSpareDeviceEnabled"
        )]
        pub is_spare_device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory::v1_17_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalSizeMiB")]
        pub logical_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDeviceType")]
        pub memory_device_type: Option<crate::memory::v1_17_1::MemoryDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLocation")]
        pub memory_location: Option<crate::memory::v1_17_1::MemoryLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMedia")]
        pub memory_media: Option<Vec<crate::memory::v1_17_1::MemoryMedia>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerManufacturerID"
        )]
        pub memory_subsystem_controller_manufacturer_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySubsystemControllerProductID"
        )]
        pub memory_subsystem_controller_product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryType")]
        pub memory_type: Option<crate::memory::v1_17_1::MemoryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModuleManufacturerID"
        )]
        pub module_manufacturer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ModuleProductID")]
        pub module_product_id: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonVolatileSizeLimitMiB"
        )]
        pub non_volatile_size_limit_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileSizeMiB")]
        pub non_volatile_size_mib: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingMemoryModes"
        )]
        pub operating_memory_modes: Option<Vec<crate::memory::v1_17_1::OperatingMemoryModes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMhz")]
        pub operating_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OperatingSpeedRangeMHz"
        )]
        pub operating_speed_range_mhz: Option<crate::control::ControlRangeExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionNumberLimit"
        )]
        pub persistent_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeLimitMiB"
        )]
        pub persistent_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentRegionSizeMaxMiB"
        )]
        pub persistent_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoisonListMaxMediaErrorRecords"
        )]
        pub poison_list_max_media_error_records: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerManagementPolicy"
        )]
        pub power_management_policy: Option<crate::memory::v1_17_1::PowerManagementPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RankCount")]
        pub rank_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Regions")]
        pub regions: Option<Vec<crate::memory::v1_17_1::RegionSet>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityCapabilities"
        )]
        pub security_capabilities: Option<crate::memory::v1_17_1::SecurityCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityState")]
        pub security_state: Option<crate::memory::v1_17_1::SecurityStates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<crate::memory::v1_17_1::SecurityStateInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareDeviceCount")]
        pub spare_device_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemDeviceID")]
        pub subsystem_device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorID")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorID")]
        pub vendor_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionNumberLimit"
        )]
        pub volatile_region_number_limit: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeLimitMiB"
        )]
        pub volatile_region_size_limit_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileRegionSizeMaxMiB"
        )]
        pub volatile_region_size_max_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VolatileSizeLimitMiB"
        )]
        pub volatile_size_limit_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolatileSizeMiB")]
        pub volatile_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryClassification {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "ByteAccessiblePersistent")]
        ByteAccessiblePersistent,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryDeviceType {
        #[default]
        #[serde(rename = "DDR")]
        DDR,
        #[serde(rename = "DDR2")]
        DDR2,
        #[serde(rename = "DDR2_SDRAM")]
        DDR2SDRAM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM")]
        DDR2SDRAMFBDIMM,
        #[serde(rename = "DDR2_SDRAM_FB_DIMM_PROBE")]
        DDR2SDRAMFBDIMMPROBE,
        #[serde(rename = "DDR3")]
        DDR3,
        #[serde(rename = "DDR3_SDRAM")]
        DDR3SDRAM,
        #[serde(rename = "DDR4")]
        DDR4,
        #[serde(rename = "DDR4E_SDRAM")]
        DDR4ESDRAM,
        #[serde(rename = "DDR4_SDRAM")]
        DDR4SDRAM,
        #[serde(rename = "DDR5")]
        DDR5,
        #[serde(rename = "DDR_SDRAM")]
        DDRSDRAM,
        #[serde(rename = "DDR_SGRAM")]
        DDRSGRAM,
        #[serde(rename = "EDO")]
        EDO,
        #[serde(rename = "FastPageMode")]
        FastPageMode,
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
        #[serde(rename = "HBM")]
        HBM,
        #[serde(rename = "HBM2")]
        HBM2,
        #[serde(rename = "HBM2E")]
        HBM2E,
        #[serde(rename = "HBM3")]
        HBM3,
        #[serde(rename = "LPDDR3_SDRAM")]
        LPDDR3SDRAM,
        #[serde(rename = "LPDDR4_SDRAM")]
        LPDDR4SDRAM,
        #[serde(rename = "Logical")]
        Logical,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PipelinedNibble")]
        PipelinedNibble,
        #[serde(rename = "ROM")]
        ROM,
        #[serde(rename = "SDRAM")]
        SDRAM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryLocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Channel")]
        pub channel: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryController")]
        pub memory_controller: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slot")]
        pub slot: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Socket")]
        pub socket: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMedia {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "Intel3DXPoint")]
        Intel3DXPoint,
        #[serde(rename = "NAND")]
        NAND,
        #[serde(rename = "Proprietary")]
        Proprietary,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryType {
        #[default]
        #[serde(rename = "DRAM")]
        DRAM,
        #[serde(rename = "IntelOptane")]
        IntelOptane,
        #[serde(rename = "NVDIMM_F")]
        NVDIMMF,
        #[serde(rename = "NVDIMM_N")]
        NVDIMMN,
        #[serde(rename = "NVDIMM_P")]
        NVDIMMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperatingMemoryModes {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OverwriteUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerManagementPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AveragePowerBudgetMilliWatts"
        )]
        pub average_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxTDPMilliWatts")]
        pub max_tdp_milli_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeakPowerBudgetMilliWatts"
        )]
        pub peak_power_budget_milli_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolicyEnabled")]
        pub policy_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RegionSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MasterPassphraseEnabled"
        )]
        pub master_passphrase_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryClassification"
        )]
        pub memory_classification: Option<crate::memory::v1_17_1::MemoryClassification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseEnabled")]
        pub passphrase_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseState")]
        pub passphrase_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
    pub struct ScanMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ScanMediaRequestBody {
        #[serde(rename = "Length")]
        pub length: i64,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NoEventLog")]
        pub no_event_log: Option<bool>,
        #[serde(rename = "PhysicalAddress")]
        pub physical_address: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecureEraseUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLockCapable"
        )]
        pub configuration_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLockCapable")]
        pub data_lock_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPassphraseCount")]
        pub max_passphrase_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassphraseCapable")]
        pub passphrase_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PassphraseLockLimit"
        )]
        pub passphrase_lock_limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityStates")]
        pub security_states: Option<Vec<crate::memory::v1_17_1::SecurityStates>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityStateInfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MasterPassphraseAttemptCountReached"
        )]
        pub master_passphrase_attempt_count_reached: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserPassphraseAttemptCountReached"
        )]
        pub user_passphrase_attempt_count_reached: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityStates {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Frozen")]
        Frozen,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "Passphraselimit")]
        Passphraselimit,
        #[serde(rename = "Unlocked")]
        Unlocked,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetMasterPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetMasterPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphrase {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetPassphraseRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UnlockUnitRequestBody {
        #[serde(rename = "Passphrase")]
        pub passphrase: String,
        #[serde(rename = "RegionId")]
        pub region_id: String,
    }
}
