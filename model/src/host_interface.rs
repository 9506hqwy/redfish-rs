use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HostInterface {
    V010300(crate::host_interface::v1_3_0::HostInterface),
    V010202(crate::host_interface::v1_2_2::HostInterface),
    V010105(crate::host_interface::v1_1_5::HostInterface),
    V010005(crate::host_interface::v1_0_5::HostInterface),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMode {
        #[default]
        #[serde(rename = "AuthNone")]
        AuthNone,
        #[serde(rename = "BasicAuth")]
        BasicAuth,
        #[serde(rename = "OemAuth")]
        OemAuth,
        #[serde(rename = "RedfishSessionAuth")]
        RedfishSessionAuth,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostInterface {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModes"
        )]
        pub authentication_modes: Option<Vec<crate::host_interface::v1_0_5::AuthenticationMode>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternallyAccessible"
        )]
        pub externally_accessible: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareAuthEnabled"
        )]
        pub firmware_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRoleId")]
        pub firmware_auth_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostEthernetInterfaces"
        )]
        pub host_ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaceType")]
        pub host_interface_type: Option<crate::host_interface::v1_0_5::HostInterfaceType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthEnabled")]
        pub kernel_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRoleId")]
        pub kernel_auth_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::host_interface::v1_0_5::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerEthernetInterface"
        )]
        pub manager_ethernet_interface: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceType {
        #[default]
        #[serde(rename = "NetworkHostInterface")]
        NetworkHostInterface,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRole")]
        pub firmware_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRole")]
        pub kernel_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
}
pub mod v1_1_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::host_interface::v1_1_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMode {
        #[default]
        #[serde(rename = "AuthNone")]
        AuthNone,
        #[serde(rename = "BasicAuth")]
        BasicAuth,
        #[serde(rename = "OemAuth")]
        OemAuth,
        #[serde(rename = "RedfishSessionAuth")]
        RedfishSessionAuth,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::host_interface::v1_1_5::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModes"
        )]
        pub authentication_modes: Option<Vec<crate::host_interface::v1_1_5::AuthenticationMode>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternallyAccessible"
        )]
        pub externally_accessible: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareAuthEnabled"
        )]
        pub firmware_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRoleId")]
        pub firmware_auth_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostEthernetInterfaces"
        )]
        pub host_ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaceType")]
        pub host_interface_type: Option<crate::host_interface::v1_1_5::HostInterfaceType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthEnabled")]
        pub kernel_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRoleId")]
        pub kernel_auth_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::host_interface::v1_1_5::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerEthernetInterface"
        )]
        pub manager_ethernet_interface: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceType {
        #[default]
        #[serde(rename = "NetworkHostInterface")]
        NetworkHostInterface,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRole")]
        pub firmware_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRole")]
        pub kernel_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::host_interface::v1_2_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMode {
        #[default]
        #[serde(rename = "AuthNone")]
        AuthNone,
        #[serde(rename = "BasicAuth")]
        BasicAuth,
        #[serde(rename = "OemAuth")]
        OemAuth,
        #[serde(rename = "RedfishSessionAuth")]
        RedfishSessionAuth,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::host_interface::v1_2_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthNoneRoleId")]
        pub auth_none_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModes"
        )]
        pub authentication_modes: Option<Vec<crate::host_interface::v1_2_2::AuthenticationMode>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternallyAccessible"
        )]
        pub externally_accessible: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareAuthEnabled"
        )]
        pub firmware_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRoleId")]
        pub firmware_auth_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostEthernetInterfaces"
        )]
        pub host_ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaceType")]
        pub host_interface_type: Option<crate::host_interface::v1_2_2::HostInterfaceType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthEnabled")]
        pub kernel_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRoleId")]
        pub kernel_auth_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::host_interface::v1_2_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerEthernetInterface"
        )]
        pub manager_ethernet_interface: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceType {
        #[default]
        #[serde(rename = "NetworkHostInterface")]
        NetworkHostInterface,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthNoneRole")]
        pub auth_none_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRole")]
        pub firmware_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRole")]
        pub kernel_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::host_interface::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMode {
        #[default]
        #[serde(rename = "AuthNone")]
        AuthNone,
        #[serde(rename = "BasicAuth")]
        BasicAuth,
        #[serde(rename = "OemAuth")]
        OemAuth,
        #[serde(rename = "RedfishSessionAuth")]
        RedfishSessionAuth,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CredentialBootstrapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnableAfterReset")]
        pub enable_after_reset: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoleId")]
        pub role_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HostInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::host_interface::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthNoneRoleId")]
        pub auth_none_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModes"
        )]
        pub authentication_modes: Option<Vec<crate::host_interface::v1_3_0::AuthenticationMode>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CredentialBootstrapping"
        )]
        pub credential_bootstrapping:
            Option<crate::host_interface::v1_3_0::CredentialBootstrapping>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternallyAccessible"
        )]
        pub externally_accessible: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareAuthEnabled"
        )]
        pub firmware_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRoleId")]
        pub firmware_auth_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostEthernetInterfaces"
        )]
        pub host_ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterfaceType")]
        pub host_interface_type: Option<crate::host_interface::v1_3_0::HostInterfaceType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthEnabled")]
        pub kernel_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRoleId")]
        pub kernel_auth_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::host_interface::v1_3_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerEthernetInterface"
        )]
        pub manager_ethernet_interface: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceType {
        #[default]
        #[serde(rename = "NetworkHostInterface")]
        NetworkHostInterface,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthNoneRole")]
        pub auth_none_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CredentialBootstrappingRole"
        )]
        pub credential_bootstrapping_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareAuthRole")]
        pub firmware_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRole")]
        pub kernel_auth_role: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
