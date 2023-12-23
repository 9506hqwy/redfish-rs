pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::ethernet_interface::v1_10_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DHCPFallback {
        #[default]
        #[serde(rename = "AutoConfig")]
        AutoConfig,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCPv4Configuration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPEnabled")]
        pub dhcp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FallbackAddress")]
        pub fallback_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseDNSServers")]
        pub use_dns_servers: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseDomainName")]
        pub use_domain_name: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseGateway")]
        pub use_gateway: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseNTPServers")]
        pub use_ntp_servers: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseStaticRoutes")]
        pub use_static_routes: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCPv6Configuration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingMode")]
        pub operating_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseDNSServers")]
        pub use_dns_servers: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseDomainName")]
        pub use_domain_name: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseNTPServers")]
        pub use_ntp_servers: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UseRapidCommit")]
        pub use_rapid_commit: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DHCPv6OperatingMode {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Stateful")]
        Stateful,
        #[serde(rename = "Stateless")]
        Stateless,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetDeviceType {
        #[default]
        #[serde(rename = "Physical")]
        Physical,
        #[serde(rename = "Virtual")]
        Virtual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EthernetInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::ethernet_interface::v1_10_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoNeg")]
        pub auto_neg: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPv4")]
        pub dhc_pv4: Option<crate::ethernet_interface::v1_10_0::DHCPv4Configuration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPv6")]
        pub dhc_pv6: Option<crate::ethernet_interface::v1_10_0::DHCPv6Configuration>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaceType"
        )]
        pub ethernet_interface_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FQDN")]
        pub fqdn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FullDuplex")]
        pub full_duplex: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Addresses")]
        pub ipv4_addresses: Option<Vec<crate::ip_addresses::IPv4Address>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv4StaticAddresses"
        )]
        pub ipv4_static_addresses: Option<Vec<crate::ip_addresses::IPv4Address>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv6AddressPolicyTable"
        )]
        pub ipv6_address_policy_table:
            Option<Vec<crate::ethernet_interface::v1_10_0::IPv6AddressPolicyEntry>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Addresses")]
        pub ipv6_addresses: Option<Vec<crate::ip_addresses::IPv6Address>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6DefaultGateway")]
        pub ipv6_default_gateway: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv6StaticAddresses"
        )]
        pub ipv6_static_addresses: Option<Vec<crate::ip_addresses::IPv6StaticAddress>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv6StaticDefaultGateways"
        )]
        pub ipv6_static_default_gateways:
            Option<Vec<crate::ip_addresses::IPv6GatewayStaticAddress>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::ethernet_interface::v1_10_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MACAddress")]
        pub mac_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxIPv6StaticAddresses"
        )]
        pub max_ipv6_static_addresses: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MTUSize")]
        pub mtu_size: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NameServers")]
        pub name_servers: Option<Vec<String>>,
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
            rename = "PermanentMACAddress"
        )]
        pub permanent_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedMbps")]
        pub speed_mbps: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StatelessAddressAutoConfig"
        )]
        pub stateless_address_auto_config:
            Option<crate::ethernet_interface::v1_10_0::StatelessAddressAutoConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StaticNameServers")]
        pub static_name_servers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TeamMode")]
        pub team_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiDevicePath")]
        pub uefi_device_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLAN")]
        pub vlan: Option<crate::vlan_network_interface::VLAN>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANs")]
        pub vlans: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6AddressPolicyEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Label")]
        pub label: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precedence")]
        pub precedence: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Prefix")]
        pub prefix: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "LinkDown")]
        LinkDown,
        #[serde(rename = "LinkUp")]
        LinkUp,
        #[serde(rename = "NoLink")]
        NoLink,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AffiliatedInterfaces"
        )]
        pub affiliated_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AffiliatedInterfaces@odata.count"
        )]
        pub affiliated_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostInterface")]
        pub host_interface: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedInterfaces")]
        pub related_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedInterfaces@odata.count"
        )]
        pub related_interfaces_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StatelessAddressAutoConfiguration {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv4AutoConfigEnabled"
        )]
        pub ipv4_auto_config_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IPv6AutoConfigEnabled"
        )]
        pub ipv6_auto_config_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TeamMode {
        #[default]
        #[serde(rename = "ActiveBackup")]
        ActiveBackup,
        #[serde(rename = "AdaptiveLoadBalancing")]
        AdaptiveLoadBalancing,
        #[serde(rename = "AdaptiveTransmitLoadBalancing")]
        AdaptiveTransmitLoadBalancing,
        #[serde(rename = "Broadcast")]
        Broadcast,
        #[serde(rename = "IEEE802_3ad")]
        IEEE8023ad,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RoundRobin")]
        RoundRobin,
        #[serde(rename = "XOR")]
        XOR,
    }
}
