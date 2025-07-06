pub type Manager = crate::manager::v1_22_0::Manager;
pub mod v1_19_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ForceFailover"
        )]
        pub manager_force_failover: Option<crate::manager::v1_19_1::ForceFailover>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ModifyRedundancySet"
        )]
        pub manager_modify_redundancy_set: Option<crate::manager::v1_19_1::ModifyRedundancySet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Manager.Reset")]
        pub manager_reset: Option<crate::manager::v1_19_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ResetToDefaults"
        )]
        pub manager_reset_to_defaults: Option<crate::manager::v1_19_1::ResetToDefaults>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager::v1_19_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CommandConnectTypesSupported {
        #[default]
        #[serde(rename = "IPMI")]
        IPMI,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "Telnet")]
        Telnet,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommandShell {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_19_1::CommandConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DaylightSavingTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndDateTime")]
        pub end_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMinutes")]
        pub offset_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartDateTime")]
        pub start_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeZoneName")]
        pub time_zone_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceFailover {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceFailoverRequestBody {
        #[serde(rename = "NewManager")]
        pub new_manager: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_19_1::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForChassis")]
        pub manager_for_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForChassis@odata.count"
        )]
        pub manager_for_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForManagers")]
        pub manager_for_managers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForManagers@odata.count"
        )]
        pub manager_for_managers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForServers")]
        pub manager_for_servers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForServers@odata.count"
        )]
        pub manager_for_servers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForSwitches")]
        pub manager_for_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForSwitches@odata.count"
        )]
        pub manager_for_switches_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerInChassis")]
        pub manager_in_chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SelectedNetworkPort"
        )]
        pub selected_network_port: Option<crate::manager::v1_19_1::LinksSelectedNetworkPort>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksSelectedNetworkPort {
        V000001(crate::manager::v1_19_1::LinksSelectedNetworkPortN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksSelectedNetworkPort {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksSelectedNetworkPortN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Manager {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager::v1_19_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalFirmwareVersions"
        )]
        pub additional_firmware_versions: Option<crate::software_inventory::AdditionalVersions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoDSTEnabled")]
        pub auto_dst_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommandShell")]
        pub command_shell: Option<crate::manager::v1_19_1::CommandShell>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DateTimeLocalOffset"
        )]
        pub date_time_local_offset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DaylightSavingTime")]
        pub daylight_saving_time: Option<crate::manager::v1_19_1::DaylightSavingTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DedicatedNetworkPorts"
        )]
        pub dedicated_network_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::manager::v1_19_1::ManagerDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::manager::v1_19_1::GraphicalConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaces")]
        pub host_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::manager::v1_19_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerDiagnosticData"
        )]
        pub manager_diagnostic_data: Option<crate::manager::v1_19_1::ManagerManagerDiagnosticData>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerType")]
        pub manager_type: Option<crate::manager::v1_19_1::ManagerType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkProtocol")]
        pub network_protocol: Option<crate::odata_v4::IdRef>,
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
        pub power_state: Option<crate::manager::v1_19_1::ManagerPowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteAccountService"
        )]
        pub remote_account_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteRedfishServiceUri"
        )]
        pub remote_redfish_service_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityPolicy")]
        pub security_policy: Option<crate::manager::v1_19_1::ManagerSecurityPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::manager::v1_19_1::SerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialInterfaces")]
        pub serial_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceEntryPointUUID"
        )]
        pub service_entry_point_uuid: Option<crate::manager::v1_19_1::ManagerServiceEntryPointUUID>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceIdentification"
        )]
        pub service_identification: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharedNetworkPorts")]
        pub shared_network_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeZoneName")]
        pub time_zone_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBPorts")]
        pub usb_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::manager::v1_19_1::ManagerUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerDescription {
        V000001(crate::manager::v1_19_1::ManagerDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ManagerDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerManagerDiagnosticData {
        V000001(crate::manager::v1_19_1::ManagerManagerDiagnosticDataN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for ManagerManagerDiagnosticData {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerManagerDiagnosticDataN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerPowerState {
        V000001(crate::manager::v1_19_1::ManagerPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for ManagerPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerSecurityPolicy {
        V000001(crate::manager::v1_19_1::ManagerSecurityPolicyN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for ManagerSecurityPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerSecurityPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerServiceEntryPointUUID {
        V000001(crate::manager::v1_19_1::ManagerServiceEntryPointUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ManagerServiceEntryPointUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerServiceEntryPointUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerType {
        #[default]
        #[serde(rename = "AuxiliaryController")]
        AuxiliaryController,
        #[serde(rename = "BMC")]
        BMC,
        #[serde(rename = "EnclosureManager")]
        EnclosureManager,
        #[serde(rename = "ManagementController")]
        ManagementController,
        #[serde(rename = "RackManager")]
        RackManager,
        #[serde(rename = "Service")]
        Service,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerUUID {
        V000001(crate::manager::v1_19_1::ManagerUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ManagerUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ModifyRedundancySet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ModifyRedundancySetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Add")]
        pub add: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Remove")]
        pub remove: Option<Vec<crate::odata_v4::IdRef>>,
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
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {
        #[serde(rename = "ResetType")]
        pub reset_type: crate::manager::v1_19_1::ResetToDefaultsType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetToDefaultsType {
        #[default]
        #[serde(rename = "PreserveNetwork")]
        PreserveNetwork,
        #[serde(rename = "PreserveNetworkAndUsers")]
        PreserveNetworkAndUsers,
        #[serde(rename = "ResetAll")]
        ResetAll,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SerialConnectTypesSupported {
        #[default]
        #[serde(rename = "IPMI")]
        IPMI,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "Telnet")]
        Telnet,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SerialConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_19_1::SerialConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
}
pub mod v1_22_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ForceFailover"
        )]
        pub manager_force_failover: Option<crate::manager::v1_22_0::ForceFailover>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ModifyRedundancySet"
        )]
        pub manager_modify_redundancy_set: Option<crate::manager::v1_22_0::ModifyRedundancySet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Manager.Reset")]
        pub manager_reset: Option<crate::manager::v1_22_0::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.ResetToDefaults"
        )]
        pub manager_reset_to_defaults: Option<crate::manager::v1_22_0::ResetToDefaults>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Manager.UpdateSecurityMode"
        )]
        pub manager_update_security_mode: Option<crate::manager::v1_22_0::UpdateSecurityMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager::v1_22_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CommandConnectTypesSupported {
        #[default]
        #[serde(rename = "IPMI")]
        IPMI,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "Telnet")]
        Telnet,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommandShell {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_22_0::CommandConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DateTimeSource {
        #[default]
        #[serde(rename = "Firmware")]
        Firmware,
        #[serde(rename = "Host")]
        Host,
        #[serde(rename = "NTP")]
        NTP,
        #[serde(rename = "PTP")]
        PTP,
        #[serde(rename = "RTC")]
        RTC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DaylightSavingTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndDateTime")]
        pub end_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMinutes")]
        pub offset_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StartDateTime")]
        pub start_date_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeZoneName")]
        pub time_zone_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceFailover {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ForceFailoverRequestBody {
        #[serde(rename = "NewManager")]
        pub new_manager: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum GraphicalConnectTypesSupported {
        #[default]
        #[serde(rename = "KVMIP")]
        KVMIP,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GraphicalConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_22_0::GraphicalConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveSoftwareImage"
        )]
        pub active_software_image: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForChassis")]
        pub manager_for_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForChassis@odata.count"
        )]
        pub manager_for_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForFabrics")]
        pub manager_for_fabrics: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForFabrics@odata.count"
        )]
        pub manager_for_fabrics_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForManagers")]
        pub manager_for_managers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForManagers@odata.count"
        )]
        pub manager_for_managers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForServers")]
        pub manager_for_servers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForServers@odata.count"
        )]
        pub manager_for_servers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerForSwitches")]
        pub manager_for_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerForSwitches@odata.count"
        )]
        pub manager_for_switches_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerInChassis")]
        pub manager_in_chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SelectedNetworkPort"
        )]
        pub selected_network_port: Option<crate::manager::v1_22_0::LinksSelectedNetworkPort>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareImages")]
        pub software_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SoftwareImages@odata.count"
        )]
        pub software_images_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksSelectedNetworkPort {
        V000001(crate::manager::v1_22_0::LinksSelectedNetworkPortN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksSelectedNetworkPort {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksSelectedNetworkPortN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Manager {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager::v1_22_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalFirmwareVersions"
        )]
        pub additional_firmware_versions: Option<crate::software_inventory::AdditionalVersions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoDSTEnabled")]
        pub auto_dst_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommandShell")]
        pub command_shell: Option<crate::manager::v1_22_0::CommandShell>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DateTimeLocalOffset"
        )]
        pub date_time_local_offset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTimeSource")]
        pub date_time_source: Option<crate::manager::v1_22_0::ManagerDateTimeSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DaylightSavingTime")]
        pub daylight_saving_time: Option<crate::manager::v1_22_0::DaylightSavingTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DedicatedNetworkPorts"
        )]
        pub dedicated_network_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::manager::v1_22_0::ManagerDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GraphicalConsole")]
        pub graphical_console: Option<crate::manager::v1_22_0::GraphicalConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaces")]
        pub host_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastResetTime")]
        pub last_reset_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::manager::v1_22_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerDiagnosticData"
        )]
        pub manager_diagnostic_data: Option<crate::manager::v1_22_0::ManagerManagerDiagnosticData>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagerType")]
        pub manager_type: Option<crate::manager::v1_22_0::ManagerType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkProtocol")]
        pub network_protocol: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMSecurityMode")]
        pub oem_security_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::manager::v1_22_0::ManagerPowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteAccountService"
        )]
        pub remote_account_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteRedfishServiceUri"
        )]
        pub remote_redfish_service_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityMode")]
        pub security_mode: Option<crate::manager::v1_22_0::SecurityModeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityPolicy")]
        pub security_policy: Option<crate::manager::v1_22_0::ManagerSecurityPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialConsole")]
        pub serial_console: Option<crate::manager::v1_22_0::SerialConsole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialInterfaces")]
        pub serial_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceEntryPointUUID"
        )]
        pub service_entry_point_uuid: Option<crate::manager::v1_22_0::ManagerServiceEntryPointUUID>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceIdentification"
        )]
        pub service_identification: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharedNetworkPorts")]
        pub shared_network_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeZoneName")]
        pub time_zone_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "USBPorts")]
        pub usb_ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::manager::v1_22_0::ManagerUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerDateTimeSource {
        V012200(crate::manager::v1_22_0::DateTimeSource),
        V000001(crate::manager::v1_22_0::ManagerDateTimeSourceN1),
    }
    impl Default for ManagerDateTimeSource {
        fn default() -> Self {
            Self::V012200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerDateTimeSourceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerDescription {
        V000001(crate::manager::v1_22_0::ManagerDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ManagerDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerManagerDiagnosticData {
        V000001(crate::manager::v1_22_0::ManagerManagerDiagnosticDataN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for ManagerManagerDiagnosticData {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerManagerDiagnosticDataN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerPowerState {
        V000001(crate::manager::v1_22_0::ManagerPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for ManagerPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerSecurityPolicy {
        V000001(crate::manager::v1_22_0::ManagerSecurityPolicyN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for ManagerSecurityPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerSecurityPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerServiceEntryPointUUID {
        V000001(crate::manager::v1_22_0::ManagerServiceEntryPointUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ManagerServiceEntryPointUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerServiceEntryPointUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerType {
        #[default]
        #[serde(rename = "AuxiliaryController")]
        AuxiliaryController,
        #[serde(rename = "BMC")]
        BMC,
        #[serde(rename = "EnclosureManager")]
        EnclosureManager,
        #[serde(rename = "FabricManager")]
        FabricManager,
        #[serde(rename = "ManagementController")]
        ManagementController,
        #[serde(rename = "RackManager")]
        RackManager,
        #[serde(rename = "Service")]
        Service,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerUUID {
        V000001(crate::manager::v1_22_0::ManagerUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ManagerUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ModifyRedundancySet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ModifyRedundancySetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Add")]
        pub add: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Remove")]
        pub remove: Option<Vec<crate::odata_v4::IdRef>>,
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
    pub struct ResetToDefaults {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetToDefaultsRequestBody {
        #[serde(rename = "ResetType")]
        pub reset_type: crate::manager::v1_22_0::ResetToDefaultsType,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResetToDefaultsType {
        #[default]
        #[serde(rename = "PreserveNetwork")]
        PreserveNetwork,
        #[serde(rename = "PreserveNetworkAndUsers")]
        PreserveNetworkAndUsers,
        #[serde(rename = "ResetAll")]
        ResetAll,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecurityModeTypes {
        #[default]
        #[serde(rename = "CNSA_1_0")]
        CNSAN1N0,
        #[serde(rename = "CNSA_2_0")]
        CNSAN2N0,
        #[serde(rename = "Default")]
        Default,
        #[serde(rename = "FIPS_140_2")]
        FIPSN140N2,
        #[serde(rename = "FIPS_140_3")]
        FIPSN140N3,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SuiteB")]
        SuiteB,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SerialConnectTypesSupported {
        #[default]
        #[serde(rename = "IPMI")]
        IPMI,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "Telnet")]
        Telnet,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SerialConsole {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectTypesSupported"
        )]
        pub connect_types_supported:
            Option<Vec<crate::manager::v1_22_0::SerialConnectTypesSupported>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxConcurrentSessions"
        )]
        pub max_concurrent_sessions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateSecurityMode {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateSecurityModeRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMSecurityMode")]
        pub oem_security_mode: Option<String>,
        #[serde(rename = "SecurityMode")]
        pub security_mode: crate::manager::v1_22_0::SecurityModeTypes,
    }
}
