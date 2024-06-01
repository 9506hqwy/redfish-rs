pub type NetworkDeviceFunction = crate::network_device_function::v1_9_1::NetworkDeviceFunction;
pub mod v1_9_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_device_function::v1_9_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "FibreChannelOverEthernet")]
        FibreChannelOverEthernet,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "PXE")]
        PXE,
        #[serde(rename = "iSCSI")]
        ISCSI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootTargets {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootPriority")]
        pub boot_priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUNID")]
        pub lunid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWPN")]
        pub wwpn: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataDirection {
        #[default]
        #[serde(rename = "Egress")]
        Egress,
        #[serde(rename = "Ingress")]
        Ingress,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Ethernet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MACAddress")]
        pub mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSize")]
        pub mtu_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSizeMaximum")]
        pub mtu_size_maximum: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PermanentMACAddress"
        )]
        pub permanent_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLAN")]
        pub vlan: Option<crate::vlan_network_interface::VLAN>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANs")]
        pub vlans: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowFIPVLANDiscovery"
        )]
        pub allow_fip_vlan_discovery: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootTargets")]
        pub boot_targets: Option<Vec<crate::network_device_function::v1_9_0::BootTargets>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCoEActiveVLANId")]
        pub fc_oe_active_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCoELocalVLANId")]
        pub fc_oe_local_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannelId")]
        pub fibre_channel_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentWWNN")]
        pub permanent_wwnn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentWWPN")]
        pub permanent_wwpn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWNSource")]
        pub wwn_source: Option<crate::network_device_function::v1_9_0::WWNSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWNN")]
        pub wwnn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWPN")]
        pub wwpn: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPBoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootMediaURI")]
        pub boot_media_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPAddressType {
        #[default]
        #[serde(rename = "IPv4")]
        IPv4,
        #[serde(rename = "IPv6")]
        IPv6,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ISCSIBoot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::network_device_function::v1_9_0::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPSecret")]
        pub chap_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUsername")]
        pub chap_username: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorDefaultGateway"
        )]
        pub initiator_default_gateway: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorIPAddress")]
        pub initiator_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorName")]
        pub initiator_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorNetmask")]
        pub initiator_netmask: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPAddressType")]
        pub ip_address_type: Option<crate::network_device_function::v1_9_0::IPAddressType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMaskDNSViaDHCP")]
        pub ip_mask_dns_via_dhcp: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MutualCHAPSecret")]
        pub mutual_chap_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MutualCHAPUsername")]
        pub mutual_chap_username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryDNS")]
        pub primary_dns: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryLUN")]
        pub primary_lun: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrimaryTargetIPAddress"
        )]
        pub primary_target_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryTargetName")]
        pub primary_target_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrimaryTargetTCPPort"
        )]
        pub primary_target_tcp_port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryVLANEnable")]
        pub primary_vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryVLANId")]
        pub primary_vlan_id: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouterAdvertisementEnabled"
        )]
        pub router_advertisement_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryDNS")]
        pub secondary_dns: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryLUN")]
        pub secondary_lun: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetIPAddress"
        )]
        pub secondary_target_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetName"
        )]
        pub secondary_target_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetTCPPort"
        )]
        pub secondary_target_tcp_port: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryVLANEnable"
        )]
        pub secondary_vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryVLANId")]
        pub secondary_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetInfoViaDHCP")]
        pub target_info_via_dhcp: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InfiniBand {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSize")]
        pub mtu_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeGUID")]
        pub node_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentNodeGUID")]
        pub permanent_node_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentPortGUID")]
        pub permanent_port_guid: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PermanentSystemGUID"
        )]
        pub permanent_system_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortGUID")]
        pub port_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedMTUSizes")]
        pub supported_mtu_sizes: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemGUID")]
        pub system_guid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Limit {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BurstBytesPerSecond"
        )]
        pub burst_bytes_per_second: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BurstPacketsPerSecond"
        )]
        pub burst_packets_per_second: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Direction")]
        pub direction: Option<crate::network_device_function::v1_9_0::DataDirection>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SustainedBytesPerSecond"
        )]
        pub sustained_bytes_per_second: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SustainedPacketsPerSecond"
        )]
        pub sustained_packets_per_second: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterface")]
        pub ethernet_interface: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffloadProcessors")]
        pub offload_processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadProcessors@odata.count"
        )]
        pub offload_processors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffloadSystem")]
        pub offload_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunction")]
        pub pcie_function: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalNetworkPortAssignment"
        )]
        pub physical_network_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalPortAssignment"
        )]
        pub physical_port_assignment: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkDeviceFunction {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_device_function::v1_9_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowDeny")]
        pub allow_deny: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalNetworkPorts"
        )]
        pub assignable_physical_network_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalNetworkPorts@odata.count"
        )]
        pub assignable_physical_network_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalPorts"
        )]
        pub assignable_physical_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalPorts@odata.count"
        )]
        pub assignable_physical_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootMode")]
        pub boot_mode: Option<crate::network_device_function::v1_9_0::BootMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceEnabled")]
        pub device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::network_device_function::v1_9_0::Ethernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::network_device_function::v1_9_0::FibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTPBoot")]
        pub http_boot: Option<crate::network_device_function::v1_9_0::HTTPBoot>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::network_device_function::v1_9_0::InfiniBand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "iSCSIBoot")]
        pub iscsi_boot: Option<crate::network_device_function::v1_9_0::ISCSIBoot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Limits")]
        pub limits: Option<Vec<crate::network_device_function::v1_9_0::Limit>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::network_device_function::v1_9_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxVirtualFunctions"
        )]
        pub max_virtual_functions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncCapabilities"
        )]
        pub net_dev_func_capabilities:
            Option<Vec<crate::network_device_function::v1_9_0::NetworkDeviceTechnology>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetDevFuncType")]
        pub net_dev_func_type:
            Option<crate::network_device_function::v1_9_0::NetworkDeviceTechnology>,
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
            rename = "PhysicalNetworkPortAssignment"
        )]
        pub physical_network_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalPortAssignment"
        )]
        pub physical_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAVIEnabled")]
        pub savi_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualFunctionsEnabled"
        )]
        pub virtual_functions_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkDeviceTechnology {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "FibreChannelOverEthernet")]
        FibreChannelOverEthernet,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
        #[serde(rename = "iSCSI")]
        ISCSI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WWNSource {
        #[default]
        #[serde(rename = "ConfiguredLocally")]
        ConfiguredLocally,
        #[serde(rename = "ProvidedByFabric")]
        ProvidedByFabric,
    }
}
pub mod v1_9_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_device_function::v1_9_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BootMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "FibreChannelOverEthernet")]
        FibreChannelOverEthernet,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "PXE")]
        PXE,
        #[serde(rename = "iSCSI")]
        ISCSI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BootTargets {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootPriority")]
        pub boot_priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUNID")]
        pub lunid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWPN")]
        pub wwpn: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataDirection {
        #[default]
        #[serde(rename = "Egress")]
        Egress,
        #[serde(rename = "Ingress")]
        Ingress,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Ethernet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MACAddress")]
        pub mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSize")]
        pub mtu_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSizeMaximum")]
        pub mtu_size_maximum: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PermanentMACAddress"
        )]
        pub permanent_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLAN")]
        pub vlan: Option<crate::vlan_network_interface::VLAN>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANs")]
        pub vlans: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowFIPVLANDiscovery"
        )]
        pub allow_fip_vlan_discovery: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootTargets")]
        pub boot_targets: Option<Vec<crate::network_device_function::v1_9_1::BootTargets>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCoEActiveVLANId")]
        pub fc_oe_active_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCoELocalVLANId")]
        pub fc_oe_local_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannelId")]
        pub fibre_channel_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentWWNN")]
        pub permanent_wwnn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentWWPN")]
        pub permanent_wwpn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWNSource")]
        pub wwn_source: Option<crate::network_device_function::v1_9_1::WWNSource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWNN")]
        pub wwnn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WWPN")]
        pub wwpn: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPBoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootMediaURI")]
        pub boot_media_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPAddressType {
        #[default]
        #[serde(rename = "IPv4")]
        IPv4,
        #[serde(rename = "IPv6")]
        IPv6,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ISCSIBoot {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::network_device_function::v1_9_1::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPSecret")]
        pub chap_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUsername")]
        pub chap_username: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorDefaultGateway"
        )]
        pub initiator_default_gateway: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorIPAddress")]
        pub initiator_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorName")]
        pub initiator_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorNetmask")]
        pub initiator_netmask: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPAddressType")]
        pub ip_address_type: Option<crate::network_device_function::v1_9_1::IPAddressType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMaskDNSViaDHCP")]
        pub ip_mask_dns_via_dhcp: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MutualCHAPSecret")]
        pub mutual_chap_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MutualCHAPUsername")]
        pub mutual_chap_username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryDNS")]
        pub primary_dns: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryLUN")]
        pub primary_lun: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrimaryTargetIPAddress"
        )]
        pub primary_target_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryTargetName")]
        pub primary_target_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrimaryTargetTCPPort"
        )]
        pub primary_target_tcp_port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryVLANEnable")]
        pub primary_vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryVLANId")]
        pub primary_vlan_id: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouterAdvertisementEnabled"
        )]
        pub router_advertisement_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryDNS")]
        pub secondary_dns: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryLUN")]
        pub secondary_lun: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetIPAddress"
        )]
        pub secondary_target_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetName"
        )]
        pub secondary_target_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryTargetTCPPort"
        )]
        pub secondary_target_tcp_port: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryVLANEnable"
        )]
        pub secondary_vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryVLANId")]
        pub secondary_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetInfoViaDHCP")]
        pub target_info_via_dhcp: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InfiniBand {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSize")]
        pub mtu_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NodeGUID")]
        pub node_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentNodeGUID")]
        pub permanent_node_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PermanentPortGUID")]
        pub permanent_port_guid: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PermanentSystemGUID"
        )]
        pub permanent_system_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortGUID")]
        pub port_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedMTUSizes")]
        pub supported_mtu_sizes: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemGUID")]
        pub system_guid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Limit {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BurstBytesPerSecond"
        )]
        pub burst_bytes_per_second: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BurstPacketsPerSecond"
        )]
        pub burst_packets_per_second: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Direction")]
        pub direction: Option<crate::network_device_function::v1_9_1::DataDirection>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SustainedBytesPerSecond"
        )]
        pub sustained_bytes_per_second: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SustainedPacketsPerSecond"
        )]
        pub sustained_packets_per_second: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterface")]
        pub ethernet_interface: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffloadProcessors")]
        pub offload_processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OffloadProcessors@odata.count"
        )]
        pub offload_processors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffloadSystem")]
        pub offload_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunction")]
        pub pcie_function: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalNetworkPortAssignment"
        )]
        pub physical_network_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalPortAssignment"
        )]
        pub physical_port_assignment: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkDeviceFunction {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_device_function::v1_9_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowDeny")]
        pub allow_deny: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalNetworkPorts"
        )]
        pub assignable_physical_network_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalNetworkPorts@odata.count"
        )]
        pub assignable_physical_network_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalPorts"
        )]
        pub assignable_physical_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssignablePhysicalPorts@odata.count"
        )]
        pub assignable_physical_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BootMode")]
        pub boot_mode: Option<crate::network_device_function::v1_9_1::BootMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceEnabled")]
        pub device_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::network_device_function::v1_9_1::Ethernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::network_device_function::v1_9_1::FibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTPBoot")]
        pub http_boot: Option<crate::network_device_function::v1_9_1::HTTPBoot>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::network_device_function::v1_9_1::InfiniBand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "iSCSIBoot")]
        pub iscsi_boot: Option<crate::network_device_function::v1_9_1::ISCSIBoot>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Limits")]
        pub limits: Option<Vec<crate::network_device_function::v1_9_1::Limit>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::network_device_function::v1_9_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxVirtualFunctions"
        )]
        pub max_virtual_functions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncCapabilities"
        )]
        pub net_dev_func_capabilities:
            Option<Vec<crate::network_device_function::v1_9_1::NetworkDeviceTechnology>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetDevFuncType")]
        pub net_dev_func_type:
            Option<crate::network_device_function::v1_9_1::NetworkDeviceTechnology>,
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
            rename = "PhysicalNetworkPortAssignment"
        )]
        pub physical_network_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PhysicalPortAssignment"
        )]
        pub physical_port_assignment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAVIEnabled")]
        pub savi_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VirtualFunctionsEnabled"
        )]
        pub virtual_functions_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NetworkDeviceTechnology {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "FibreChannelOverEthernet")]
        FibreChannelOverEthernet,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
        #[serde(rename = "iSCSI")]
        ISCSI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum WWNSource {
        #[default]
        #[serde(rename = "ConfiguredLocally")]
        ConfiguredLocally,
        #[serde(rename = "ProvidedByFabric")]
        ProvidedByFabric,
    }
}
