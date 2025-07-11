pub type HostInterface = crate::host_interface::v1_3_3::HostInterface;
pub mod v1_3_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::host_interface::v1_3_3::OemActions>,
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
        pub actions: Option<crate::host_interface::v1_3_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthNoneRoleId")]
        pub auth_none_role_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModes"
        )]
        pub authentication_modes: Option<Vec<crate::host_interface::v1_3_3::AuthenticationMode>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CredentialBootstrapping"
        )]
        pub credential_bootstrapping:
            Option<crate::host_interface::v1_3_3::CredentialBootstrapping>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::host_interface::v1_3_3::HostInterfaceDescription>,
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
        pub host_interface_type:
            Option<crate::host_interface::v1_3_3::HostInterfaceHostInterfaceType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthEnabled")]
        pub kernel_auth_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KernelAuthRoleId")]
        pub kernel_auth_role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::host_interface::v1_3_3::Links>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum HostInterfaceDescription {
        V000001(crate::host_interface::v1_3_3::HostInterfaceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for HostInterfaceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum HostInterfaceHostInterfaceType {
        V010303(crate::host_interface::v1_3_3::HostInterfaceType),
        V000001(crate::host_interface::v1_3_3::HostInterfaceHostInterfaceTypeN1),
    }
    impl Default for HostInterfaceHostInterfaceType {
        fn default() -> Self {
            Self::V010303(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum HostInterfaceHostInterfaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
