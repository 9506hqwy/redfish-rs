use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AddressPool {
    V010202(crate::address_pool::v1_2_2::AddressPool),
    V010103(crate::address_pool::v1_1_3::AddressPool),
    V010002(crate::address_pool::v1_0_2::AddressPool),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::address_pool::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddressPool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::address_pool::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::address_pool::v1_0_2::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::address_pool::v1_0_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCID")]
        pub max_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSID")]
        pub max_sid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinCID")]
        pub min_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSID")]
        pub min_sid: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ASNumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::address_pool::v1_1_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddressPool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::address_pool::v1_1_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::address_pool::v1_1_3::Ethernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::address_pool::v1_1_3::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::address_pool::v1_1_3::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BFDSingleHopOnly {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DemandModeEnabled")]
        pub demand_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DesiredMinTxIntervalMilliseconds"
        )]
        pub desired_min_tx_interval_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyChain")]
        pub key_chain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalMultiplier")]
        pub local_multiplier: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeticulousModeEnabled"
        )]
        pub meticulous_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequiredMinRxIntervalMilliseconds"
        )]
        pub required_min_rx_interval_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourcePort")]
        pub source_port: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPEvpn {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayIPAddress"
        )]
        pub anycast_gateway_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayMACAddress"
        )]
        pub anycast_gateway_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ARPProxyEnabled")]
        pub arp_proxy_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ARPSupressionEnabled"
        )]
        pub arp_supression_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ESINumberRange")]
        pub esi_number_range: Option<crate::address_pool::v1_1_3::ESINumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EVINumberRange")]
        pub evi_number_range: Option<crate::address_pool::v1_1_3::EVINumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GatewayIPAddress")]
        pub gateway_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NDPProxyEnabled")]
        pub ndp_proxy_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NDPSupressionEnabled"
        )]
        pub ndp_supression_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouteDistinguisherRange"
        )]
        pub route_distinguisher_range: Option<crate::address_pool::v1_1_3::RouteDistinguisherRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RouteTargetRange")]
        pub route_target_range: Option<crate::address_pool::v1_1_3::RouteTargetRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderlayMulticastEnabled"
        )]
        pub underlay_multicast_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnknownUnicastSuppressionEnabled"
        )]
        pub unknown_unicast_suppression_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VLANIdentifierAddressRange"
        )]
        pub vlan_identifier_address_range:
            Option<crate::address_pool::v1_1_3::VLANIdentifierAddressRange>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPNeighbor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowOwnASEnabled")]
        pub allow_own_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectRetrySeconds"
        )]
        pub connect_retry_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HoldTimeSeconds")]
        pub hold_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "KeepaliveIntervalSeconds"
        )]
        pub keepalive_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAS")]
        pub local_as: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogStateChangesEnabled"
        )]
        pub log_state_changes_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPrefix")]
        pub max_prefix: Option<crate::address_pool::v1_1_3::MaxPrefix>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinimumAdvertisementIntervalSeconds"
        )]
        pub minimum_advertisement_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassiveModeEnabled")]
        pub passive_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PathMTUDiscoveryEnabled"
        )]
        pub path_mtu_discovery_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeerAS")]
        pub peer_as: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplacePeerASEnabled"
        )]
        pub replace_peer_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TCPMaxSegmentSizeBytes"
        )]
        pub tcp_max_segment_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TreatAsWithdrawEnabled"
        )]
        pub treat_as_withdraw_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPRoute {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdvertiseInactiveRoutesEnabled"
        )]
        pub advertise_inactive_routes_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceExternal")]
        pub distance_external: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceInternal")]
        pub distance_internal: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceLocal")]
        pub distance_local: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternalCompareRouterIdEnabled"
        )]
        pub external_compare_router_id_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlapDampingEnabled")]
        pub flap_damping_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendDefaultRouteEnabled"
        )]
        pub send_default_route_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommonBGPProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASNumberRange")]
        pub as_number_range: Option<crate::address_pool::v1_1_3::ASNumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPNeighbor")]
        pub bgp_neighbor: Option<crate::address_pool::v1_1_3::BGPNeighbor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPRoute")]
        pub bgp_route: Option<crate::address_pool::v1_1_3::BGPRoute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracefulRestart")]
        pub graceful_restart: Option<crate::address_pool::v1_1_3::GracefulRestart>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiplePaths")]
        pub multiple_paths: Option<crate::address_pool::v1_1_3::MultiplePaths>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendCommunityEnabled"
        )]
        pub send_community_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DHCPInterfaceMTUBytes"
        )]
        pub dhcp_interface_mtu_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPRelayEnabled")]
        pub dhcp_relay_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPServer")]
        pub dhcp_server: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EBGP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowDuplicateASEnabled"
        )]
        pub allow_duplicate_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowOverrideASEnabled"
        )]
        pub allow_override_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AlwaysCompareMEDEnabled"
        )]
        pub always_compare_med_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASNumberRange")]
        pub as_number_range: Option<crate::address_pool::v1_1_3::ASNumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPLocalPreference")]
        pub bgp_local_preference: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPNeighbor")]
        pub bgp_neighbor: Option<crate::address_pool::v1_1_3::BGPNeighbor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPRoute")]
        pub bgp_route: Option<crate::address_pool::v1_1_3::BGPRoute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPWeight")]
        pub bgp_weight: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracefulRestart")]
        pub graceful_restart: Option<crate::address_pool::v1_1_3::GracefulRestart>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MED")]
        pub med: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultihopEnabled")]
        pub multihop_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultihopTTL")]
        pub multihop_ttl: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiplePaths")]
        pub multiple_paths: Option<crate::address_pool::v1_1_3::MultiplePaths>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendCommunityEnabled"
        )]
        pub send_community_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ESINumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EVINumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Ethernet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BFDSingleHopOnly")]
        pub bfd_single_hop_only: Option<crate::address_pool::v1_1_3::BFDSingleHopOnly>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPEvpn")]
        pub bgp_evpn: Option<crate::address_pool::v1_1_3::BGPEvpn>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EBGP")]
        pub ebgp: Option<crate::address_pool::v1_1_3::EBGP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4")]
        pub ipv4: Option<crate::address_pool::v1_1_3::IPv4>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiProtocolEBGP")]
        pub multi_protocol_ebgp: Option<crate::address_pool::v1_1_3::EBGP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiProtocolIBGP")]
        pub multi_protocol_ibgp: Option<crate::address_pool::v1_1_3::CommonBGPProperties>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCID")]
        pub max_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSID")]
        pub max_sid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinCID")]
        pub min_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSID")]
        pub min_sid: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GracefulRestart {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GracefulRestartEnabled"
        )]
        pub graceful_restart_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HelperModeEnabled")]
        pub helper_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StaleRoutesTimeSeconds"
        )]
        pub stale_routes_time_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeSeconds")]
        pub time_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4 {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayIPAddress"
        )]
        pub anycast_gateway_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayMACAddress"
        )]
        pub anycast_gateway_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCP")]
        pub dhcp: Option<crate::address_pool::v1_1_3::DHCP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DistributeIntoUnderlayEnabled"
        )]
        pub distribute_into_underlay_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DNSDomainName")]
        pub dns_domain_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DNSServer")]
        pub dns_server: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EBGPAddressRange")]
        pub ebgp_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricLinkAddressRange"
        )]
        pub fabric_link_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GatewayIPAddress")]
        pub gateway_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostAddressRange")]
        pub host_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IBGPAddressRange")]
        pub ibgp_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LoopbackAddressRange"
        )]
        pub loopback_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressRange"
        )]
        pub management_address_range: Option<crate::address_pool::v1_1_3::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NativeVLAN")]
        pub native_vlan: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NTPOffsetHoursMinutes"
        )]
        pub ntp_offset_hours_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTPServer")]
        pub ntp_server: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTPTimezone")]
        pub ntp_timezone: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VLANIdentifierAddressRange"
        )]
        pub vlan_identifier_address_range:
            Option<crate::address_pool::v1_1_3::VLANIdentifierAddressRange>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4AddressRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MaxPrefix {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPrefixNumber")]
        pub max_prefix_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestartTimerSeconds"
        )]
        pub restart_timer_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ShutdownThresholdPercentage"
        )]
        pub shutdown_threshold_percentage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThresholdWarningOnlyEnabled"
        )]
        pub threshold_warning_only_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MultiplePaths {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumPaths")]
        pub maximum_paths: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UseMultiplePathsEnabled"
        )]
        pub use_multiple_paths_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RouteDistinguisherRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RouteTargetRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLANIdentifierAddressRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
}
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ASNumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::address_pool::v1_2_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddressPool {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::address_pool::v1_2_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::address_pool::v1_2_2::Ethernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::address_pool::v1_2_2::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::address_pool::v1_2_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BFDSingleHopOnly {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DemandModeEnabled")]
        pub demand_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DesiredMinTxIntervalMilliseconds"
        )]
        pub desired_min_tx_interval_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyChain")]
        pub key_chain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalMultiplier")]
        pub local_multiplier: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeticulousModeEnabled"
        )]
        pub meticulous_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequiredMinRxIntervalMilliseconds"
        )]
        pub required_min_rx_interval_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourcePort")]
        pub source_port: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPEvpn {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayIPAddress"
        )]
        pub anycast_gateway_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayMACAddress"
        )]
        pub anycast_gateway_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ARPProxyEnabled")]
        pub arp_proxy_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ARPSupressionEnabled"
        )]
        pub arp_supression_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ESINumberRange")]
        pub esi_number_range: Option<crate::address_pool::v1_2_2::ESINumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EVINumberRange")]
        pub evi_number_range: Option<crate::address_pool::v1_2_2::EVINumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GatewayIPAddress")]
        pub gateway_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GatewayIPAddressRange"
        )]
        pub gateway_ip_address_range: Option<crate::address_pool::v1_2_2::GatewayIPAddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NDPProxyEnabled")]
        pub ndp_proxy_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NDPSupressionEnabled"
        )]
        pub ndp_supression_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouteDistinguisherAdministratorSubfield"
        )]
        pub route_distinguisher_administrator_subfield: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouteDistinguisherRange"
        )]
        pub route_distinguisher_range: Option<crate::address_pool::v1_2_2::RouteDistinguisherRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RouteTargetAdministratorSubfield"
        )]
        pub route_target_administrator_subfield: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RouteTargetRange")]
        pub route_target_range: Option<crate::address_pool::v1_2_2::RouteTargetRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderlayMulticastEnabled"
        )]
        pub underlay_multicast_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnknownUnicastSuppressionEnabled"
        )]
        pub unknown_unicast_suppression_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VLANIdentifierAddressRange"
        )]
        pub vlan_identifier_address_range:
            Option<crate::address_pool::v1_2_2::VLANIdentifierAddressRange>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPNeighbor {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowOwnASEnabled")]
        pub allow_own_as_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CIDR")]
        pub cidr: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectRetrySeconds"
        )]
        pub connect_retry_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HoldTimeSeconds")]
        pub hold_time_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "KeepaliveIntervalSeconds"
        )]
        pub keepalive_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAS")]
        pub local_as: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LogStateChangesEnabled"
        )]
        pub log_state_changes_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPrefix")]
        pub max_prefix: Option<crate::address_pool::v1_2_2::MaxPrefix>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinimumAdvertisementIntervalSeconds"
        )]
        pub minimum_advertisement_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PassiveModeEnabled")]
        pub passive_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PathMTUDiscoveryEnabled"
        )]
        pub path_mtu_discovery_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PeerAS")]
        pub peer_as: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplacePeerASEnabled"
        )]
        pub replace_peer_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TCPMaxSegmentSizeBytes"
        )]
        pub tcp_max_segment_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TreatAsWithdrawEnabled"
        )]
        pub treat_as_withdraw_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BGPRoute {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdvertiseInactiveRoutesEnabled"
        )]
        pub advertise_inactive_routes_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceExternal")]
        pub distance_external: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceInternal")]
        pub distance_internal: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DistanceLocal")]
        pub distance_local: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternalCompareRouterIdEnabled"
        )]
        pub external_compare_router_id_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlapDampingEnabled")]
        pub flap_damping_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendDefaultRouteEnabled"
        )]
        pub send_default_route_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommonBGPProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASNumberRange")]
        pub as_number_range: Option<crate::address_pool::v1_2_2::ASNumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPNeighbor")]
        pub bgp_neighbor: Option<crate::address_pool::v1_2_2::BGPNeighbor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPRoute")]
        pub bgp_route: Option<crate::address_pool::v1_2_2::BGPRoute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracefulRestart")]
        pub graceful_restart: Option<crate::address_pool::v1_2_2::GracefulRestart>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiplePaths")]
        pub multiple_paths: Option<crate::address_pool::v1_2_2::MultiplePaths>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendCommunityEnabled"
        )]
        pub send_community_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DHCPInterfaceMTUBytes"
        )]
        pub dhcp_interface_mtu_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPRelayEnabled")]
        pub dhcp_relay_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPServer")]
        pub dhcp_server: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EBGP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowDuplicateASEnabled"
        )]
        pub allow_duplicate_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowOverrideASEnabled"
        )]
        pub allow_override_as_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AlwaysCompareMEDEnabled"
        )]
        pub always_compare_med_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASNumberRange")]
        pub as_number_range: Option<crate::address_pool::v1_2_2::ASNumberRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPLocalPreference")]
        pub bgp_local_preference: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPNeighbor")]
        pub bgp_neighbor: Option<crate::address_pool::v1_2_2::BGPNeighbor>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPRoute")]
        pub bgp_route: Option<crate::address_pool::v1_2_2::BGPRoute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPWeight")]
        pub bgp_weight: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracefulRestart")]
        pub graceful_restart: Option<crate::address_pool::v1_2_2::GracefulRestart>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MED")]
        pub med: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultihopEnabled")]
        pub multihop_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultihopTTL")]
        pub multihop_ttl: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiplePaths")]
        pub multiple_paths: Option<crate::address_pool::v1_2_2::MultiplePaths>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SendCommunityEnabled"
        )]
        pub send_community_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ESINumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EVINumberRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Ethernet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BFDSingleHopOnly")]
        pub bfd_single_hop_only: Option<crate::address_pool::v1_2_2::BFDSingleHopOnly>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BGPEvpn")]
        pub bgp_evpn: Option<crate::address_pool::v1_2_2::BGPEvpn>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EBGP")]
        pub ebgp: Option<crate::address_pool::v1_2_2::EBGP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4")]
        pub ipv4: Option<crate::address_pool::v1_2_2::IPv4>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiProtocolEBGP")]
        pub multi_protocol_ebgp: Option<crate::address_pool::v1_2_2::EBGP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiProtocolIBGP")]
        pub multi_protocol_ibgp: Option<crate::address_pool::v1_2_2::CommonBGPProperties>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GatewayIPAddressRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCID")]
        pub max_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSID")]
        pub max_sid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinCID")]
        pub min_cid: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinSID")]
        pub min_sid: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GracefulRestart {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GracefulRestartEnabled"
        )]
        pub graceful_restart_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HelperModeEnabled")]
        pub helper_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "StaleRoutesTimeSeconds"
        )]
        pub stale_routes_time_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeSeconds")]
        pub time_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4 {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayIPAddress"
        )]
        pub anycast_gateway_ip_address: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AnycastGatewayMACAddress"
        )]
        pub anycast_gateway_mac_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCP")]
        pub dhcp: Option<crate::address_pool::v1_2_2::DHCP>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DistributeIntoUnderlayEnabled"
        )]
        pub distribute_into_underlay_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DNSDomainName")]
        pub dns_domain_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DNSServer")]
        pub dns_server: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EBGPAddressRange")]
        pub ebgp_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricLinkAddressRange"
        )]
        pub fabric_link_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GatewayIPAddress")]
        pub gateway_ip_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostAddressRange")]
        pub host_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IBGPAddressRange")]
        pub ibgp_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LoopbackAddressRange"
        )]
        pub loopback_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressRange"
        )]
        pub management_address_range: Option<crate::address_pool::v1_2_2::IPv4AddressRange>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NativeVLAN")]
        pub native_vlan: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NTPOffsetHoursMinutes"
        )]
        pub ntp_offset_hours_minutes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTPServer")]
        pub ntp_server: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTPTimezone")]
        pub ntp_timezone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemMACRange")]
        pub system_mac_range: Option<crate::address_pool::v1_2_2::SystemMACRange>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VLANIdentifierAddressRange"
        )]
        pub vlan_identifier_address_range:
            Option<crate::address_pool::v1_2_2::VLANIdentifierAddressRange>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4AddressRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MaxPrefix {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPrefixNumber")]
        pub max_prefix_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestartTimerSeconds"
        )]
        pub restart_timer_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ShutdownThresholdPercentage"
        )]
        pub shutdown_threshold_percentage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThresholdWarningOnlyEnabled"
        )]
        pub threshold_warning_only_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MultiplePaths {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumPaths")]
        pub maximum_paths: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UseMultiplePathsEnabled"
        )]
        pub use_multiple_paths_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RouteDistinguisherRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RouteTargetRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SystemMACRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLANIdentifierAddressRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lower")]
        pub lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Upper")]
        pub upper: Option<i64>,
    }
}
