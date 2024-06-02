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
pub type ComputerSystem = crate::computer_system::v1_22_1::ComputerSystem;
pub mod v1_22_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_22_0::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Decommission"
        )]
        pub computer_system_decommission: Option<crate::computer_system::v1_22_0::Decommission>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_22_0::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_22_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_22_0::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_22_0::OemActions>,
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
        pub automatic_retry_config: Option<crate::computer_system::v1_22_0::AutomaticRetryConfig>,
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
        pub boot_order_property_selection: Option<crate::computer_system::v1_22_0::BootOrderTypes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_22_0::BootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_22_0::BootSourceOverrideMode>,
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
        pub stop_boot_on_fault: Option<crate::computer_system::v1_22_0::StopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_22_0::TrustedModuleRequiredToBoot>,
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
        pub last_state: Option<crate::computer_system::v1_22_0::BootProgressTypes>,
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
        pub use_cases: Option<Vec<crate::computer_system::v1_22_0::CompositionUseCase>>,
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
        pub actions: Option<crate::computer_system::v1_22_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_22_0::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_22_0::BootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Composition")]
        pub composition: Option<crate::computer_system::v1_22_0::Composition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_22_0::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_22_0::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_22_0::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_22_0::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_22_0::IdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_22_0::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_22_0::KeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_22_0::Links>,
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
        pub memory_summary: Option<crate::computer_system::v1_22_0::MemorySummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSystem")]
        pub operating_system: Option<crate::odata_v4::IdRef>,
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
        pub power_mode: Option<crate::computer_system::v1_22_0::PowerMode>,
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
        pub power_restore_policy: Option<crate::computer_system::v1_22_0::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_22_0::ProcessorSummary>,
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
        pub serial_console: Option<crate::computer_system::v1_22_0::HostSerialConsole>,
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
        pub system_type: Option<crate::computer_system::v1_22_0::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_22_0::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_22_0::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Decommission {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DecommissionRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "DecommissionTypes")]
        pub decommission_types: Vec<crate::computer_system::v1_22_0::DecommissionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDecommissionTypes"
        )]
        pub oem_decommission_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequireSecureErase")]
        pub require_secure_erase: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DecommissionType {
        #[default]
        #[serde(rename = "All")]
        All,
        #[serde(rename = "BIOSConfig")]
        BIOSConfig,
        #[serde(rename = "Logs")]
        Logs,
        #[serde(rename = "ManagerConfig")]
        ManagerConfig,
        #[serde(rename = "NetworkConfig")]
        NetworkConfig,
        #[serde(rename = "StorageConfig")]
        StorageConfig,
        #[serde(rename = "UserData")]
        UserData,
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
            Option<Vec<crate::computer_system::v1_22_0::GraphicalConnectTypesSupported>>,
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
        pub ipmi: Option<crate::computer_system::v1_22_0::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_22_0::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_22_0::SerialConsoleProtocol>,
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
        pub cache_policy: Option<crate::computer_system::v1_22_0::KMIPCachePolicy>,
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
        pub kmip_servers: Option<Vec<crate::computer_system::v1_22_0::KMIPServer>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingComputerSystem"
        )]
        pub hosting_computer_system: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMachines")]
        pub virtual_machines: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualMachines@odata.count"
        )]
        pub virtual_machines_odata_count: Option<i64>,
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
        pub memory_mirroring: Option<crate::computer_system::v1_22_0::MemoryMirroring>,
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
        #[serde(rename = "EfficiencyFavorPerformance")]
        EfficiencyFavorPerformance,
        #[serde(rename = "EfficiencyFavorPower")]
        EfficiencyFavorPower,
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
        pub interface_type: Option<crate::computer_system::v1_22_0::InterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_22_0::InterfaceTypeSelection>,
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
        pub timeout_action: Option<crate::computer_system::v1_22_0::WatchdogTimeoutActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_22_0::WatchdogWarningActions>,
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
pub mod v1_22_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.AddResourceBlock"
        )]
        pub computer_system_add_resource_block:
            Option<crate::computer_system::v1_22_1::AddResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Decommission"
        )]
        pub computer_system_decommission: Option<crate::computer_system::v1_22_1::Decommission>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.RemoveResourceBlock"
        )]
        pub computer_system_remove_resource_block:
            Option<crate::computer_system::v1_22_1::RemoveResourceBlock>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.Reset"
        )]
        pub computer_system_reset: Option<crate::computer_system::v1_22_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComputerSystem.SetDefaultBootOrder"
        )]
        pub computer_system_set_default_boot_order:
            Option<crate::computer_system::v1_22_1::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::computer_system::v1_22_1::OemActions>,
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
        pub alias_boot_order: Option<Vec<crate::computer_system::v1_22_1::BootAliasBootOrder>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryAttempts"
        )]
        pub automatic_retry_attempts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomaticRetryConfig"
        )]
        pub automatic_retry_config:
            Option<crate::computer_system::v1_22_1::BootAutomaticRetryConfig>,
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
        pub boot_order_property_selection:
            Option<crate::computer_system::v1_22_1::BootBootOrderPropertySelection>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideEnabled"
        )]
        pub boot_source_override_enabled:
            Option<crate::computer_system::v1_22_1::BootBootSourceOverrideEnabled>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideMode"
        )]
        pub boot_source_override_mode:
            Option<crate::computer_system::v1_22_1::BootBootSourceOverrideMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BootSourceOverrideTarget"
        )]
        pub boot_source_override_target:
            Option<crate::computer_system::v1_22_1::BootBootSourceOverrideTarget>,
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
        pub stop_boot_on_fault: Option<crate::computer_system::v1_22_1::BootStopBootOnFault>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedModuleRequiredToBoot"
        )]
        pub trusted_module_required_to_boot:
            Option<crate::computer_system::v1_22_1::BootTrustedModuleRequiredToBoot>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UefiTargetBootSourceOverride"
        )]
        pub uefi_target_boot_source_override: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootAliasBootOrder {
        V000001(crate::computer_system::v1_22_1::BootAliasBootOrderN1),
        ComputerSystemBootSource(crate::computer_system::BootSource),
    }
    impl Default for BootAliasBootOrder {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootAliasBootOrderN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootAutomaticRetryConfig {
        V012201(crate::computer_system::v1_22_1::AutomaticRetryConfig),
        V000001(crate::computer_system::v1_22_1::BootAutomaticRetryConfigN1),
    }
    impl Default for BootAutomaticRetryConfig {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootAutomaticRetryConfigN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootBootOrderPropertySelection {
        V012201(crate::computer_system::v1_22_1::BootOrderTypes),
        V000001(crate::computer_system::v1_22_1::BootBootOrderPropertySelectionN1),
    }
    impl Default for BootBootOrderPropertySelection {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootBootOrderPropertySelectionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootBootSourceOverrideEnabled {
        V012201(crate::computer_system::v1_22_1::BootSourceOverrideEnabled),
        V000001(crate::computer_system::v1_22_1::BootBootSourceOverrideEnabledN1),
    }
    impl Default for BootBootSourceOverrideEnabled {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootBootSourceOverrideEnabledN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootBootSourceOverrideMode {
        V012201(crate::computer_system::v1_22_1::BootSourceOverrideMode),
        V000001(crate::computer_system::v1_22_1::BootBootSourceOverrideModeN1),
    }
    impl Default for BootBootSourceOverrideMode {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootBootSourceOverrideModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootBootSourceOverrideTarget {
        V000001(crate::computer_system::v1_22_1::BootBootSourceOverrideTargetN1),
        ComputerSystemBootSource(crate::computer_system::BootSource),
    }
    impl Default for BootBootSourceOverrideTarget {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootBootSourceOverrideTargetN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub last_state: Option<crate::computer_system::v1_22_1::BootProgressLastState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastStateTime")]
        pub last_state_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLastState")]
        pub oem_last_state: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootProgressLastState {
        V012201(crate::computer_system::v1_22_1::BootProgressTypes),
        V000001(crate::computer_system::v1_22_1::BootProgressLastStateN1),
    }
    impl Default for BootProgressLastState {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootProgressLastStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootStopBootOnFault {
        V012201(crate::computer_system::v1_22_1::StopBootOnFault),
        V000001(crate::computer_system::v1_22_1::BootStopBootOnFaultN1),
    }
    impl Default for BootStopBootOnFault {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootStopBootOnFaultN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum BootTrustedModuleRequiredToBoot {
        V012201(crate::computer_system::v1_22_1::TrustedModuleRequiredToBoot),
        V000001(crate::computer_system::v1_22_1::BootTrustedModuleRequiredToBootN1),
    }
    impl Default for BootTrustedModuleRequiredToBoot {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootTrustedModuleRequiredToBootN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Composition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseCases")]
        pub use_cases: Option<Vec<crate::computer_system::v1_22_1::CompositionUseCases>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionUseCase {
        #[default]
        #[serde(rename = "ExpandableSystem")]
        ExpandableSystem,
        #[serde(rename = "ResourceBlockCapable")]
        ResourceBlockCapable,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CompositionUseCases {
        V012201(crate::computer_system::v1_22_1::CompositionUseCase),
        V000001(crate::computer_system::v1_22_1::CompositionUseCasesN1),
    }
    impl Default for CompositionUseCases {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionUseCasesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComputerSystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::computer_system::v1_22_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Bios")]
        pub bios: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BiosVersion")]
        pub bios_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Boot")]
        pub boot: Option<crate::computer_system::v1_22_1::Boot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootProgress")]
        pub boot_progress: Option<crate::computer_system::v1_22_1::ComputerSystemBootProgress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Composition")]
        pub composition: Option<crate::computer_system::v1_22_1::ComputerSystemComposition>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::computer_system::v1_22_1::ComputerSystemDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::computer_system::v1_22_1::HostGraphicalConsole>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GraphicsControllers"
        )]
        pub graphics_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostWatchdogTimer")]
        pub host_watchdog_timer: Option<crate::computer_system::v1_22_1::WatchdogTimer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostedServices")]
        pub hosted_services: Option<crate::computer_system::v1_22_1::HostedServices>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostingRoles")]
        pub hosting_roles: Option<Vec<crate::computer_system::v1_22_1::HostingRole>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IdlePowerSaver")]
        pub idle_power_saver: Option<crate::computer_system::v1_22_1::ComputerSystemIdlePowerSaver>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::computer_system::v1_22_1::ComputerSystemIndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyManagement")]
        pub key_management: Option<crate::computer_system::v1_22_1::ComputerSystemKeyManagement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::computer_system::v1_22_1::Links>,
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
        pub memory_summary: Option<crate::computer_system::v1_22_1::MemorySummary>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSystem")]
        pub operating_system: Option<crate::odata_v4::IdRef>,
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
        pub power_mode: Option<crate::computer_system::v1_22_1::ComputerSystemPowerMode>,
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
        pub power_restore_policy: Option<crate::computer_system::v1_22_1::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::computer_system::v1_22_1::ComputerSystemPowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProcessorSummary")]
        pub processor_summary: Option<crate::computer_system::v1_22_1::ProcessorSummary>,
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
        pub serial_console: Option<crate::computer_system::v1_22_1::HostSerialConsole>,
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
        pub system_type: Option<crate::computer_system::v1_22_1::SystemType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedModules")]
        pub trusted_modules: Option<Vec<crate::computer_system::v1_22_1::TrustedModules>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBControllers")]
        pub usb_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::computer_system::v1_22_1::ComputerSystemUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMediaConfig")]
        pub virtual_media_config: Option<crate::computer_system::v1_22_1::VirtualMediaConfig>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemBootProgress {
        V012201(crate::computer_system::v1_22_1::BootProgress),
        V000001(crate::computer_system::v1_22_1::ComputerSystemBootProgressN1),
    }
    impl Default for ComputerSystemBootProgress {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemBootProgressN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemComposition {
        V012201(crate::computer_system::v1_22_1::Composition),
        V000001(crate::computer_system::v1_22_1::ComputerSystemCompositionN1),
    }
    impl Default for ComputerSystemComposition {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemCompositionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemDescription {
        V000001(crate::computer_system::v1_22_1::ComputerSystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ComputerSystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemIdlePowerSaver {
        V012201(crate::computer_system::v1_22_1::IdlePowerSaver),
        V000001(crate::computer_system::v1_22_1::ComputerSystemIdlePowerSaverN1),
    }
    impl Default for ComputerSystemIdlePowerSaver {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemIdlePowerSaverN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemIndicatorLED {
        V012201(crate::computer_system::v1_22_1::IndicatorLED),
        V000001(crate::computer_system::v1_22_1::ComputerSystemIndicatorLEDN1),
    }
    impl Default for ComputerSystemIndicatorLED {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemIndicatorLEDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemKeyManagement {
        V012201(crate::computer_system::v1_22_1::KeyManagement),
        V000001(crate::computer_system::v1_22_1::ComputerSystemKeyManagementN1),
    }
    impl Default for ComputerSystemKeyManagement {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemKeyManagementN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemPowerMode {
        V012201(crate::computer_system::v1_22_1::PowerMode),
        V000001(crate::computer_system::v1_22_1::ComputerSystemPowerModeN1),
    }
    impl Default for ComputerSystemPowerMode {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemPowerModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemPowerState {
        V000001(crate::computer_system::v1_22_1::ComputerSystemPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for ComputerSystemPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComputerSystemUUID {
        V000001(crate::computer_system::v1_22_1::ComputerSystemUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ComputerSystemUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComputerSystemUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Decommission {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DecommissionRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystemETag")]
        pub computer_system_etag: Option<String>,
        #[serde(rename = "DecommissionTypes")]
        pub decommission_types: Vec<crate::computer_system::v1_22_1::DecommissionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDecommissionTypes"
        )]
        pub oem_decommission_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequireSecureErase")]
        pub require_secure_erase: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DecommissionType {
        #[default]
        #[serde(rename = "All")]
        All,
        #[serde(rename = "BIOSConfig")]
        BIOSConfig,
        #[serde(rename = "Logs")]
        Logs,
        #[serde(rename = "ManagerConfig")]
        ManagerConfig,
        #[serde(rename = "NetworkConfig")]
        NetworkConfig,
        #[serde(rename = "StorageConfig")]
        StorageConfig,
        #[serde(rename = "UserData")]
        UserData,
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
            Option<Vec<crate::computer_system::v1_22_1::GraphicalConnectTypesSupported>>,
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
        pub ipmi: Option<crate::computer_system::v1_22_1::SerialConsoleProtocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::computer_system::v1_22_1::SerialConsoleProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::computer_system::v1_22_1::SerialConsoleProtocol>,
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
        pub cache_policy: Option<crate::computer_system::v1_22_1::KMIPServerCachePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KMIPServerCachePolicy {
        V012201(crate::computer_system::v1_22_1::KMIPCachePolicy),
        V000001(crate::computer_system::v1_22_1::KMIPServerCachePolicyN1),
    }
    impl Default for KMIPServerCachePolicy {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KMIPServerCachePolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyManagement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPCertificates")]
        pub kmip_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KMIPServers")]
        pub kmip_servers: Option<Vec<crate::computer_system::v1_22_1::KeyManagementKMIPServers>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KeyManagementKMIPServers {
        V012201(crate::computer_system::v1_22_1::KMIPServer),
        V000001(crate::computer_system::v1_22_1::KeyManagementKMIPServersN1),
    }
    impl Default for KeyManagementKMIPServers {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyManagementKMIPServersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostingComputerSystem"
        )]
        pub hosting_computer_system:
            Option<crate::computer_system::v1_22_1::LinksHostingComputerSystem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMachines")]
        pub virtual_machines: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualMachines@odata.count"
        )]
        pub virtual_machines_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksHostingComputerSystem {
        V000001(crate::computer_system::v1_22_1::LinksHostingComputerSystemN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksHostingComputerSystem {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksHostingComputerSystemN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub memory_mirroring: Option<crate::computer_system::v1_22_1::MemorySummaryMemoryMirroring>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemorySummaryMemoryMirroring {
        V012201(crate::computer_system::v1_22_1::MemoryMirroring),
        V000001(crate::computer_system::v1_22_1::MemorySummaryMemoryMirroringN1),
    }
    impl Default for MemorySummaryMemoryMirroring {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemorySummaryMemoryMirroringN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerMode {
        #[default]
        #[serde(rename = "BalancedPerformance")]
        BalancedPerformance,
        #[serde(rename = "EfficiencyFavorPerformance")]
        EfficiencyFavorPerformance,
        #[serde(rename = "EfficiencyFavorPower")]
        EfficiencyFavorPower,
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
        pub interface_type: Option<crate::computer_system::v1_22_1::TrustedModulesInterfaceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterfaceTypeSelection"
        )]
        pub interface_type_selection:
            Option<crate::computer_system::v1_22_1::TrustedModulesInterfaceTypeSelection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TrustedModulesInterfaceType {
        V012201(crate::computer_system::v1_22_1::InterfaceType),
        V000001(crate::computer_system::v1_22_1::TrustedModulesInterfaceTypeN1),
    }
    impl Default for TrustedModulesInterfaceType {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModulesInterfaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TrustedModulesInterfaceTypeSelection {
        V012201(crate::computer_system::v1_22_1::InterfaceTypeSelection),
        V000001(crate::computer_system::v1_22_1::TrustedModulesInterfaceTypeSelectionN1),
    }
    impl Default for TrustedModulesInterfaceTypeSelection {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TrustedModulesInterfaceTypeSelectionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub timeout_action: crate::computer_system::v1_22_1::WatchdogTimerTimeoutAction,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WarningAction")]
        pub warning_action: Option<crate::computer_system::v1_22_1::WatchdogTimerWarningAction>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum WatchdogTimerTimeoutAction {
        V012201(crate::computer_system::v1_22_1::WatchdogTimeoutActions),
        V000001(crate::computer_system::v1_22_1::WatchdogTimerTimeoutActionN1),
    }
    impl Default for WatchdogTimerTimeoutAction {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimerTimeoutActionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum WatchdogTimerWarningAction {
        V012201(crate::computer_system::v1_22_1::WatchdogWarningActions),
        V000001(crate::computer_system::v1_22_1::WatchdogTimerWarningActionN1),
    }
    impl Default for WatchdogTimerWarningAction {
        fn default() -> Self {
            Self::V012201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WatchdogTimerWarningActionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
