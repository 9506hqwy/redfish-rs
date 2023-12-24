use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum BootSource {
    #[default]
    #[serde(rename = "BiosSetup")]
    BiosSetup,
    #[serde(rename = "Cd")]
    Cd,
    #[serde(rename = "Diags")]
    Diags,
    #[serde(rename = "Floppy")]
    Floppy,
    #[serde(rename = "Hdd")]
    Hdd,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Pxe")]
    Pxe,
    #[serde(rename = "Recovery")]
    Recovery,
    #[serde(rename = "RemoteDrive")]
    RemoteDrive,
    #[serde(rename = "SDCard")]
    SDCard,
    #[serde(rename = "UefiBootNext")]
    UefiBootNext,
    #[serde(rename = "UefiHttp")]
    UefiHttp,
    #[serde(rename = "UefiShell")]
    UefiShell,
    #[serde(rename = "UefiTarget")]
    UefiTarget,
    #[serde(rename = "Usb")]
    Usb,
    #[serde(rename = "Utilities")]
    Utilities,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComputerSystem {
    ComputerSystemV1N0N20ComputerSystem(crate::computer_system::v1_0_20::ComputerSystem),
    ComputerSystemV1N10N9ComputerSystem(crate::computer_system::v1_10_9::ComputerSystem),
    ComputerSystemV1N11N8ComputerSystem(crate::computer_system::v1_11_8::ComputerSystem),
    ComputerSystemV1N12N7ComputerSystem(crate::computer_system::v1_12_7::ComputerSystem),
    ComputerSystemV1N13N6ComputerSystem(crate::computer_system::v1_13_6::ComputerSystem),
    ComputerSystemV1N14N5ComputerSystem(crate::computer_system::v1_14_5::ComputerSystem),
    ComputerSystemV1N15N4ComputerSystem(crate::computer_system::v1_15_4::ComputerSystem),
    ComputerSystemV1N16N4ComputerSystem(crate::computer_system::v1_16_4::ComputerSystem),
    ComputerSystemV1N17N3ComputerSystem(crate::computer_system::v1_17_3::ComputerSystem),
    ComputerSystemV1N18N2ComputerSystem(crate::computer_system::v1_18_2::ComputerSystem),
    ComputerSystemV1N19N2ComputerSystem(crate::computer_system::v1_19_2::ComputerSystem),
    ComputerSystemV1N1N18ComputerSystem(crate::computer_system::v1_1_18::ComputerSystem),
    ComputerSystemV1N20N1ComputerSystem(crate::computer_system::v1_20_1::ComputerSystem),
    ComputerSystemV1N2N17ComputerSystem(crate::computer_system::v1_2_17::ComputerSystem),
    ComputerSystemV1N3N16ComputerSystem(crate::computer_system::v1_3_16::ComputerSystem),
    ComputerSystemV1N4N15ComputerSystem(crate::computer_system::v1_4_15::ComputerSystem),
    ComputerSystemV1N5N14ComputerSystem(crate::computer_system::v1_5_14::ComputerSystem),
    ComputerSystemV1N6N12ComputerSystem(crate::computer_system::v1_6_12::ComputerSystem),
    ComputerSystemV1N7N11ComputerSystem(crate::computer_system::v1_7_11::ComputerSystem),
    ComputerSystemV1N8N10ComputerSystem(crate::computer_system::v1_8_10::ComputerSystem),
    ComputerSystemV1N9N10ComputerSystem(crate::computer_system::v1_9_10::ComputerSystem),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_20 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_0_20::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_0_20::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_0_20::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_0_20::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_0_20::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_0_20::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_0_20::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_0_20::MemorySummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_0_20::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_0_20::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub enum SystemType {
        #[default]
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
}
pub mod v1_10_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_10_9::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_10_9::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_10_9::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_10_9::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_10_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_10_9::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_10_9::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_10_9::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_10_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_10_9::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_10_9::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_10_9::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_10_9::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_10_9::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_10_9::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_10_9::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_10_9::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_10_9::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_10_9::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_10_9::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_10_9::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_10_9::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_10_9::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_10_9::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_10_9::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_11_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_11_8::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_11_8::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_11_8::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_11_8::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_11_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_11_8::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_11_8::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_11_8::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_11_8::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_11_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_11_8::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_11_8::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_11_8::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_11_8::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_11_8::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_11_8::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_11_8::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_11_8::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_11_8::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_11_8::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_11_8::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_11_8::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_11_8::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_11_8::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_11_8::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_11_8::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_12_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_12_7::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_12_7::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_12_7::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_12_7::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_12_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_12_7::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_12_7::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_12_7::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_12_7::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_12_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_12_7::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_12_7::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_12_7::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_12_7::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_12_7::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_12_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_12_7::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_12_7::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_12_7::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_12_7::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_12_7::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_12_7::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_12_7::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_12_7::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_12_7::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_12_7::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_13_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_13_6::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_13_6::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_13_6::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_13_6::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_13_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_13_6::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_13_6::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_13_6::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_13_6::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_13_6::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_13_6::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_13_6::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_13_6::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_13_6::HostGraphicalConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_13_6::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_13_6::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_13_6::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_13_6::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_13_6::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_13_6::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_13_6::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_13_6::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_13_6::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_13_6::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_13_6::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_13_6::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_13_6::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_13_6::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_13_6::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_13_6::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_13_6::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_13_6::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_13_6::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_13_6::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_13_6::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_14_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_14_5::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_14_5::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_14_5::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_14_5::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_14_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_14_5::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_14_5::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_14_5::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_14_5::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_14_5::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_14_5::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_14_5::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_14_5::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_14_5::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_14_5::HostGraphicalConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_14_5::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_14_5::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_14_5::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_14_5::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_14_5::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_14_5::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_14_5::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_14_5::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_14_5::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_14_5::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_14_5::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_14_5::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_14_5::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_14_5::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_14_5::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_14_5::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_14_5::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_14_5::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_14_5::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_14_5::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_14_5::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_15_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_15_4::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_15_4::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_15_4::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_15_4::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_15_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_15_4::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_15_4::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_15_4::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_15_4::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_15_4::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_15_4::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_15_4::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_15_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_15_4::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_15_4::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_15_4::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_15_4::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_15_4::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_15_4::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_15_4::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_15_4::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_15_4::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_15_4::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_15_4::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_15_4::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_15_4::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_15_4::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_15_4::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_15_4::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_15_4::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_15_4::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_15_4::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_15_4::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_15_4::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_15_4::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_15_4::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_15_4::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_15_4::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_16_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_16_4::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_16_4::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_16_4::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_16_4::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_16_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_16_4::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_16_4::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_16_4::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_16_4::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_16_4::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_16_4::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_16_4::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_16_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_16_4::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_16_4::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_16_4::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_16_4::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_16_4::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_16_4::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_16_4::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_16_4::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_16_4::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_16_4::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_16_4::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_16_4::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_16_4::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_16_4::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_16_4::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_16_4::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_16_4::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_16_4::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_16_4::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_16_4::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_16_4::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_16_4::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IdlePowerSaver {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterDwellTimeSeconds"
        )]
        pub enter_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterUtilizationPercent"
        )]
        pub enter_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitDwellTimeSeconds"
        )]
        pub exit_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitUtilizationPercent"
        )]
        pub exit_utilization_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KMIPServer {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_16_4::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_16_4::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "DPU")]
        DPU,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_16_4::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_16_4::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_16_4::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_16_4::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_17_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_17_3::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_17_3::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_17_3::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_17_3::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_17_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_17_3::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_17_3::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_17_3::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_17_3::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_17_3::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_17_3::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_17_3::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_17_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_17_3::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_17_3::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_17_3::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_17_3::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_17_3::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_17_3::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_17_3::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_17_3::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_17_3::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_17_3::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_17_3::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_17_3::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_17_3::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_17_3::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_17_3::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_17_3::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_17_3::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_17_3::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_17_3::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_17_3::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_17_3::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_17_3::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IdlePowerSaver {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterDwellTimeSeconds"
        )]
        pub enter_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterUtilizationPercent"
        )]
        pub enter_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitDwellTimeSeconds"
        )]
        pub exit_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitUtilizationPercent"
        )]
        pub exit_utilization_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KMIPServer {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_17_3::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions"
        )]
        pub offloaded_network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions@odata.count"
        )]
        pub offloaded_network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_17_3::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "DPU")]
        DPU,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_17_3::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_17_3::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_17_3::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_17_3::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_18_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_18_2::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_18_2::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_18_2::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_18_2::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_18_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_18_2::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_18_2::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_18_2::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_18_2::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_18_2::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_18_2::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastBootTimeSeconds"
        )]
        pub last_boot_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_18_2::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Composition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseCases")]
        pub use_cases: Option<Vec<crate::computer_system::v1_18_2::CompositionUseCase>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionUseCase {
        #[default]
        #[serde(rename = "ExpandableSystem")]
        ExpandableSystem,
        #[serde(rename = "ResourceBlockCapable")]
        ResourceBlockCapable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_18_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_18_2::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_18_2::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Composition")]
        pub composition: Option<crate::computer_system::v1_18_2::Composition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_18_2::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_18_2::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_18_2::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_18_2::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_18_2::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_18_2::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_18_2::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_18_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManufacturingMode")]
        pub manufacturing_mode: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_18_2::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_18_2::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_18_2::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_18_2::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_18_2::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_18_2::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_18_2::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_18_2::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_18_2::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_18_2::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_18_2::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_18_2::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IdlePowerSaver {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterDwellTimeSeconds"
        )]
        pub enter_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterUtilizationPercent"
        )]
        pub enter_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitDwellTimeSeconds"
        )]
        pub exit_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitUtilizationPercent"
        )]
        pub exit_utilization_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KMIPServer {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_18_2::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions"
        )]
        pub offloaded_network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions@odata.count"
        )]
        pub offloaded_network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_18_2::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "DPU")]
        DPU,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_18_2::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_18_2::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_18_2::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_18_2::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_19_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_19_2::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_19_2::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_19_2::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_19_2::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_19_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_19_2::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_19_2::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_19_2::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_19_2::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_19_2::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_19_2::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastBootTimeSeconds"
        )]
        pub last_boot_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_19_2::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Composition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseCases")]
        pub use_cases: Option<Vec<crate::computer_system::v1_19_2::CompositionUseCase>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionUseCase {
        #[default]
        #[serde(rename = "ExpandableSystem")]
        ExpandableSystem,
        #[serde(rename = "ResourceBlockCapable")]
        ResourceBlockCapable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_19_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_19_2::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_19_2::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Composition")]
        pub composition: Option<crate::computer_system::v1_19_2::Composition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_19_2::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_19_2::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_19_2::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_19_2::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_19_2::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_19_2::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_19_2::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_19_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManufacturingMode")]
        pub manufacturing_mode: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_19_2::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_19_2::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_19_2::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_19_2::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_19_2::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_19_2::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_19_2::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_19_2::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_19_2::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_19_2::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_19_2::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_19_2::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IdlePowerSaver {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterDwellTimeSeconds"
        )]
        pub enter_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterUtilizationPercent"
        )]
        pub enter_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitDwellTimeSeconds"
        )]
        pub exit_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitUtilizationPercent"
        )]
        pub exit_utilization_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KMIPServer {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_19_2::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions"
        )]
        pub offloaded_network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions@odata.count"
        )]
        pub offloaded_network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedComponents")]
        pub trusted_components: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedComponents@odata.count"
        )]
        pub trusted_components_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_19_2::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "DPU")]
        DPU,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_19_2::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_19_2::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_19_2::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_19_2::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_1_18 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_1_18::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_1_18::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_1_18::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_1_18::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_1_18::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_1_18::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_1_18::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_1_18::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_1_18::MemorySummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_1_18::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_1_18::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_1_18::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_1_18::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub enum SystemType {
        #[default]
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_1_18::InterfaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_20_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_20_1::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_20_1::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_20_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_20_1::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_20_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutomaticRetryConfig {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "RetryAlways")]
        RetryAlways,
        #[serde(rename = "RetryAttempts")]
        RetryAttempts,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config: Option<crate::computer_system::v1_20_1::AutomaticRetryConfig>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_20_1::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_20_1::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_20_1::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingAutomaticRetryAttempts"
        )]
        pub remaining_automatic_retry_attempts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBootOnFault")]
        pub stop_boot_on_fault: Option<crate::computer_system::v1_20_1::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_20_1::TrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootProgress {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastBootTimeSeconds"
        )]
        pub last_boot_time_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastState")]
        pub last_state: Option<crate::computer_system::v1_20_1::BootProgressTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressTypes {
        #[default]
        #[serde(rename = "BusInitializationStarted")]
        BusInitializationStarted,
        #[serde(rename = "MemoryInitializationStarted")]
        MemoryInitializationStarted,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSBootStarted")]
        OSBootStarted,
        #[serde(rename = "OSRunning")]
        OSRunning,
        #[serde(rename = "PCIResourceConfigStarted")]
        PCIResourceConfigStarted,
        #[serde(rename = "PrimaryProcessorInitializationStarted")]
        PrimaryProcessorInitializationStarted,
        #[serde(rename = "SecondaryProcessorInitializationStarted")]
        SecondaryProcessorInitializationStarted,
        #[serde(rename = "SetupEntered")]
        SetupEntered,
        #[serde(rename = "SystemHardwareInitializationComplete")]
        SystemHardwareInitializationComplete,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Composition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseCases")]
        pub use_cases: Option<Vec<crate::computer_system::v1_20_1::CompositionUseCase>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionUseCase {
        #[default]
        #[serde(rename = "ExpandableSystem")]
        ExpandableSystem,
        #[serde(rename = "ResourceBlockCapable")]
        ResourceBlockCapable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_20_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_20_1::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_20_1::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Composition")]
        pub composition: Option<crate::computer_system::v1_20_1::Composition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_20_1::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_20_1::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_20_1::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_20_1::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_20_1::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_20_1::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_20_1::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_20_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManufacturingMode")]
        pub manufacturing_mode: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_20_1::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::computer_system::v1_20_1::PowerMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_20_1::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_20_1::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::computer_system::v1_20_1::HostSerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_20_1::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_20_1::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_20_1::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostGraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::computer_system::v1_20_1::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostSerialConsole {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::computer_system::v1_20_1::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_20_1::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_20_1::SerialConsoleProtocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "Appliance")]
        Appliance,
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "BareMetalServer")]
        BareMetalServer,
        #[serde(rename = "ContainerServer")]
        ContainerServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "VirtualMachineServer")]
        VirtualMachineServer,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IdlePowerSaver {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterDwellTimeSeconds"
        )]
        pub enter_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterUtilizationPercent"
        )]
        pub enter_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitDwellTimeSeconds"
        )]
        pub exit_dwell_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExitUtilizationPercent"
        )]
        pub exit_utilization_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KMIPCachePolicy {
        #[default]
        #[serde(rename = "AfterFirstUse")]
        AfterFirstUse,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KMIPServer {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheDuration")]
        pub cache_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CachePolicy")]
        pub cache_policy: Option<crate::computer_system::v1_20_1::KMIPCachePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_20_1::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions"
        )]
        pub offloaded_network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadedNetworkDeviceFunctions@odata.count"
        )]
        pub offloaded_network_device_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedComponents")]
        pub trusted_components: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedComponents@odata.count"
        )]
        pub trusted_components_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_20_1::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "MaximumPerformance")]
        MaximumPerformance,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OSControlled")]
        OSControlled,
        #[serde(rename = "PowerSaving")]
        PowerSaving,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThreadingEnabled")]
        pub threading_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SerialConsoleProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsoleEntryCommand"
        )]
        pub console_entry_command: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HotKeySequenceDisplay"
        )]
        pub hot_key_sequence_display: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SharedWithManagerCLI"
        )]
        pub shared_with_manager_cli: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBootOnFault {
        #[default]
        #[serde(rename = "AnyFault")]
        AnyFault,
        #[serde(rename = "Never")]
        Never,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "DPU")]
        DPU,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModuleRequiredToBoot {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Required")]
        Required,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_20_1::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_20_1::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMediaConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_20_1::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_20_1::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_2_17 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_2_17::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_2_17::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_2_17::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_2_17::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_2_17::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_2_17::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_2_17::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_2_17::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_2_17::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_2_17::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_2_17::MemorySummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_2_17::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_2_17::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_2_17::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_2_17::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub enum SystemType {
        #[default]
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_2_17::InterfaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_3_16 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_3_16::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_3_16::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_3_16::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_3_16::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_3_16::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_3_16::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_3_16::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_3_16::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_3_16::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_3_16::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_3_16::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_3_16::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_3_16::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_3_16::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_3_16::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub enum SystemType {
        #[default]
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_3_16::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_3_16::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_4_15 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_4_15::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_4_15::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_4_15::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_4_15::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_4_15::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_4_15::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_4_15::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_4_15::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_4_15::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_4_15::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_4_15::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_4_15::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_4_15::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_4_15::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_4_15::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_4_15::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_4_15::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_5_14 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_5_14::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_5_14::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_5_14::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_5_14::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_5_14::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_5_14::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_5_14::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_5_14::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_5_14::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_5_14::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_5_14::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_5_14::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_5_14::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_5_14::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_5_14::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_5_14::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_5_14::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_5_14::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_5_14::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_5_14::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_5_14::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_6_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_6_12::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_6_12::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_6_12::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_6_12::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_6_12::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_6_12::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_6_12::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_6_12::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_6_12::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_6_12::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_6_12::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_6_12::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_6_12::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_6_12::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_6_12::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_6_12::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_6_12::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_6_12::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_6_12::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_6_12::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_6_12::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_6_12::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_6_12::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_6_12::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_6_12::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_7_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_7_11::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_7_11::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_7_11::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_7_11::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_7_11::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_7_11::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_7_11::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_7_11::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_7_11::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_7_11::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_7_11::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_7_11::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_7_11::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_7_11::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_7_11::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_7_11::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_7_11::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_7_11::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_7_11::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_7_11::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_7_11::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_7_11::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_7_11::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_7_11::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_7_11::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_8_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_8_10::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_8_10::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_8_10::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_8_10::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_8_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_8_10::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_8_10::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_8_10::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_8_10::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_8_10::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_8_10::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_8_10::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_8_10::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_8_10::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_8_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_8_10::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_8_10::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_8_10::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_8_10::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_8_10::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_8_10::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_8_10::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_8_10::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_8_10::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_8_10::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
pub mod v1_9_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_9_10::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_9_10::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_9_10::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_9_10::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_9_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Boot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AliasBootOrder")]
        pub alias_boot_order: Option<Vec<crate::computer_system::BootSource>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootNext")]
        pub boot_next: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOptions")]
        pub boot_options: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootOrder")]
        pub boot_order: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootOrderPropertySelection"
        )]
        pub boot_order_property_selection: Option<crate::computer_system::v1_9_10::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_9_10::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_9_10::BootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target: Option<crate::computer_system::BootSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpBootUri")]
        pub http_boot_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootOrderTypes {
        #[default]
        #[serde(rename = "AliasBootOrder")]
        AliasBootOrder,
        #[serde(rename = "BootOrder")]
        BootOrder,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideEnabled {
        #[default]
        #[serde(rename = "Continuous")]
        Continuous,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Once")]
        Once,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootSourceOverrideMode {
        #[default]
        #[serde(rename = "Legacy")]
        Legacy,
        #[serde(rename = "UEFI")]
        UEFI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_9_10::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_9_10::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_9_10::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_9_10::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_9_10::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_9_10::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_9_10::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySummary")]
        pub memory_summary: Option<crate::computer_system::v1_9_10::MemorySummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<crate::computer_system::v1_9_10::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_9_10::ProcessorSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecureBoot")]
        pub secure_boot: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubModel")]
        pub sub_model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemType")]
        pub system_type: Option<crate::computer_system::v1_9_10::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_9_10::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostedServices {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostingRole {
        #[default]
        #[serde(rename = "ApplicationServer")]
        ApplicationServer,
        #[serde(rename = "StorageServer")]
        StorageServer,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceType {
        #[default]
        #[serde(rename = "TCM1_0")]
        TCM1N0,
        #[serde(rename = "TPM1_2")]
        TPM1N2,
        #[serde(rename = "TPM2_0")]
        TPM2N0,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InterfaceTypeSelection {
        #[default]
        #[serde(rename = "BiosSetting")]
        BiosSetting,
        #[serde(rename = "FirmwareUpdate")]
        FirmwareUpdate,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OemMethod")]
        OemMethod,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems"
        )]
        pub consuming_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingComputerSystems@odata.count"
        )]
        pub consuming_computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems"
        )]
        pub supplying_computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingComputerSystems@odata.count"
        )]
        pub supplying_computer_systems_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryMirroring {
        #[default]
        #[serde(rename = "DIMM")]
        DIMM,
        #[serde(rename = "Hybrid")]
        Hybrid,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemorySummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryMirroring")]
        pub memory_mirroring: Option<crate::computer_system::v1_9_10::MemoryMirroring>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemMemoryGiB"
        )]
        pub total_system_memory_gib: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalSystemPersistentMemoryGiB"
        )]
        pub total_system_persistent_memory_gib: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerRestorePolicyTypes {
        #[default]
        #[serde(rename = "AlwaysOff")]
        AlwaysOff,
        #[serde(rename = "AlwaysOn")]
        AlwaysOn,
        #[serde(rename = "LastState")]
        LastState,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProcessorSummary {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
        pub count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogicalProcessorCount"
        )]
        pub logical_processor_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveResourceBlockRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "ResourceBlock")]
        pub resource_block: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlockETag")]
        pub resource_block_etag: Option<String>,
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
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SystemType {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "PhysicallyPartitioned")]
        PhysicallyPartitioned,
        #[serde(rename = "Virtual")]
        Virtual,
        #[serde(rename = "VirtuallyPartitioned")]
        VirtuallyPartitioned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TrustedModules {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion2")]
        pub firmware_version2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceType")]
        pub interface_type: Option<crate::computer_system::v1_9_10::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_9_10::InterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimeoutActions {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PowerCycle")]
        PowerCycle,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "ResetSystem")]
        ResetSystem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct WatchdogTimer {
        #[serde(rename = "FunctionEnabled")]
        pub function_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: Option<crate::computer_system::v1_9_10::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_9_10::WatchdogWarningActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogWarningActions {
        #[default]
        #[serde(rename = "DiagnosticInterrupt")]
        DiagnosticInterrupt,
        #[serde(rename = "MessagingInterrupt")]
        MessagingInterrupt,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCI")]
        SCI,
        #[serde(rename = "SMI")]
        SMI,
    }
}
