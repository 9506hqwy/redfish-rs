pub type Port = crate::port::v1_13_0::Port;
pub mod v1_11_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port::v1_11_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.Reset")]
        pub port_reset: Option<crate::port::v1_11_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.ResetPPB")]
        pub port_reset_ppb: Option<crate::port::v1_11_0::ResetPPB>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Congestion")]
        pub congestion: Option<crate::port::v1_11_0::Congestion>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceMode"
        )]
        pub connected_device_mode: Option<crate::port::v1_11_0::ConnectedDeviceMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceType"
        )]
        pub connected_device_type: Option<crate::port::v1_11_0::ConnectedDeviceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentPortConfigurationState"
        )]
        pub current_port_configuration_state:
            Option<crate::port::v1_11_0::CurrentPortConfigurationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxLogicalDeviceCount"
        )]
        pub max_logical_device_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities: Option<crate::port::v1_11_0::QoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedCXLModes")]
        pub supported_cxl_modes: Option<Vec<crate::port::v1_11_0::ConnectedDeviceMode>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionEnabled"
        )]
        pub temporary_throughput_reduction_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConfiguredNetworkLink {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredLinkSpeedGbps"
        )]
        pub configured_link_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfiguredWidth")]
        pub configured_width: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Congestion {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BackpressureSampleInterval"
        )]
        pub backpressure_sample_interval: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompletionCollectionInterval"
        )]
        pub completion_collection_interval: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CongestionTelemetryEnabled"
        )]
        pub congestion_telemetry_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressModeratePercentage"
        )]
        pub egress_moderate_percentage: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressSeverePercentage"
        )]
        pub egress_severe_percentage: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxSustainedRequestCmpBias"
        )]
        pub max_sustained_request_cmp_bias: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedDeviceMode {
        #[default]
        #[serde(rename = "CXL68BFlitAndVH")]
        CXL68BFlitAndVH,
        #[serde(rename = "CXLLatencyOptimized256BFlit")]
        CXLLatencyOptimized256BFlit,
        #[serde(rename = "Disconnected")]
        Disconnected,
        #[serde(rename = "PBR")]
        PBR,
        #[serde(rename = "RCD")]
        RCD,
        #[serde(rename = "Standard256BFlit")]
        Standard256BFlit,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedDeviceType {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "PCIeDevice")]
        PCIeDevice,
        #[serde(rename = "Type1")]
        Type1,
        #[serde(rename = "Type2")]
        Type2,
        #[serde(rename = "Type3MLD")]
        Type3MLD,
        #[serde(rename = "Type3SLD")]
        Type3SLD,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentPortConfigurationState {
        #[default]
        #[serde(rename = "BindInProgress")]
        BindInProgress,
        #[serde(rename = "DSP")]
        DSP,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "FabricLink")]
        FabricLink,
        #[serde(rename = "Reserved")]
        Reserved,
        #[serde(rename = "USP")]
        USP,
        #[serde(rename = "UnbindInProgress")]
        UnbindInProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EthernetProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedMACAddresses"
        )]
        pub associated_mac_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::port::v1_11_0::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::port::v1_11_0::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPReceive")]
        pub lldp_receive: Option<crate::port::v1_11_0::LLDPReceive>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPTransmit")]
        pub lldp_transmit: Option<crate::port::v1_11_0::LLDPTransmit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::port::v1_11_0::SupportedEthernetCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FiberConnectionType {
        #[default]
        #[serde(rename = "MultiMode")]
        MultiMode,
        #[serde(rename = "SingleMode")]
        SingleMode,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannelProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedWorldWideNames"
        )]
        pub associated_world_wide_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricName")]
        pub fabric_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberDiscoveredRemotePorts"
        )]
        pub number_discovered_remote_ports: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortConnectionType")]
        pub port_connection_type: Option<crate::port::v1_11_0::PortConnectionType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FlowControl {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RX")]
        RX,
        #[serde(rename = "TX")]
        TX,
        #[serde(rename = "TX_RX")]
        TXRX,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FunctionMaxBandwidth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocationPercent")]
        pub allocation_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FunctionMinBandwidth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocationPercent")]
        pub allocation_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LPRT")]
        pub lprt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MPRT")]
        pub mprt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCAT")]
        pub vcat: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IEEE802IdSubtype {
        #[default]
        #[serde(rename = "AgentId")]
        AgentId,
        #[serde(rename = "ChassisComp")]
        ChassisComp,
        #[serde(rename = "IfAlias")]
        IfAlias,
        #[serde(rename = "IfName")]
        IfName,
        #[serde(rename = "LocalAssign")]
        LocalAssign,
        #[serde(rename = "MacAddr")]
        MacAddr,
        #[serde(rename = "NetworkAddr")]
        NetworkAddr,
        #[serde(rename = "NotTransmitted")]
        NotTransmitted,
        #[serde(rename = "PortComp")]
        PortComp,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InfiniBandProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNodeGUIDs"
        )]
        pub associated_node_gu_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPortGUIDs"
        )]
        pub associated_port_gu_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedSystemGUIDs"
        )]
        pub associated_system_gu_ids: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LLDPReceive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_11_0::IEEE802IdSubtype>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv4"
        )]
        pub management_address_ipv4: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv6"
        )]
        pub management_address_ipv6: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressMAC"
        )]
        pub management_address_mac: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementVlanId")]
        pub management_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortIdSubtype")]
        pub port_id_subtype: Option<crate::port::v1_11_0::IEEE802IdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_11_0::LLDPSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPSystemCapabilities {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DOCSISCableDevice")]
        DOCSISCableDevice,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "Repeater")]
        Repeater,
        #[serde(rename = "Router")]
        Router,
        #[serde(rename = "Station")]
        Station,
        #[serde(rename = "Telephone")]
        Telephone,
        #[serde(rename = "WLANAccessPoint")]
        WLANAccessPoint,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LLDPTransmit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_11_0::IEEE802IdSubtype>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv4"
        )]
        pub management_address_ipv4: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv6"
        )]
        pub management_address_ipv6: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressMAC"
        )]
        pub management_address_mac: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementVlanId")]
        pub management_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortIdSubtype")]
        pub port_id_subtype: Option<crate::port::v1_11_0::IEEE802IdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_11_0::LLDPSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LinkConfiguration {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiationCapable"
        )]
        pub auto_speed_negotiation_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiationEnabled"
        )]
        pub auto_speed_negotiation_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableLinkSpeedGbps"
        )]
        pub capable_link_speed_gbps: Option<Vec<f64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredNetworkLinks"
        )]
        pub configured_network_links: Option<Vec<crate::port::v1_11_0::ConfiguredNetworkLink>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "GenZ")]
        GenZ,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
        #[serde(rename = "PCIe")]
        PCIe,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
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
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Training")]
        Training,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedEndpoints"
        )]
        pub associated_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedEndpoints@odata.count"
        )]
        pub associated_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables@odata.count")]
        pub cables_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitchPorts"
        )]
        pub connected_switch_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitchPorts@odata.count"
        )]
        pub connected_switch_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedSwitches")]
        pub connected_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitches@odata.count"
        )]
        pub connected_switches_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediumType {
        #[default]
        #[serde(rename = "Copper")]
        Copper,
        #[serde(rename = "FiberOptic")]
        FiberOptic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Port {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port::v1_11_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveWidth")]
        pub active_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableProtocolVersions"
        )]
        pub capable_protocol_versions: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentProtocolVersion"
        )]
        pub current_protocol_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentSpeedGbps")]
        pub current_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::port::v1_11_0::CXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::port::v1_11_0::EthernetProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port::v1_11_0::FibreChannelProperties>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMaxBandwidth"
        )]
        pub function_max_bandwidth: Option<Vec<crate::port::v1_11_0::FunctionMaxBandwidth>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMinBandwidth"
        )]
        pub function_min_bandwidth: Option<Vec<crate::port::v1_11_0::FunctionMinBandwidth>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port::v1_11_0::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::port::v1_11_0::InfiniBandProperties>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkConfiguration")]
        pub link_configuration: Option<Vec<crate::port::v1_11_0::LinkConfiguration>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::port::v1_11_0::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkState")]
        pub link_state: Option<crate::port::v1_11_0::LinkState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::port::v1_11_0::LinkStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkTransitionIndicator"
        )]
        pub link_transition_indicator: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::port::v1_11_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxFrameSize")]
        pub max_frame_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedGbps")]
        pub max_speed_gbps: Option<f64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMedium")]
        pub port_medium: Option<crate::port::v1_11_0::PortMedium>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortProtocol")]
        pub port_protocol: Option<crate::protocol::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortType")]
        pub port_type: Option<crate::port::v1_11_0::PortType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemotePortId")]
        pub remote_port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SFP")]
        pub sfp: Option<crate::port::v1_11_0::SFP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
        pub width: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortConnectionType {
        #[default]
        #[serde(rename = "DPort")]
        DPort,
        #[serde(rename = "EPort")]
        EPort,
        #[serde(rename = "EXPort")]
        EXPort,
        #[serde(rename = "ExtenderFabric")]
        ExtenderFabric,
        #[serde(rename = "FLPort")]
        FLPort,
        #[serde(rename = "FPort")]
        FPort,
        #[serde(rename = "GPort")]
        GPort,
        #[serde(rename = "Generic")]
        Generic,
        #[serde(rename = "NLPort")]
        NLPort,
        #[serde(rename = "NPPort")]
        NPPort,
        #[serde(rename = "NPort")]
        NPort,
        #[serde(rename = "NotConnected")]
        NotConnected,
        #[serde(rename = "PointToPoint")]
        PointToPoint,
        #[serde(rename = "PrivateLoop")]
        PrivateLoop,
        #[serde(rename = "PublicLoop")]
        PublicLoop,
        #[serde(rename = "TEPort")]
        TEPort,
        #[serde(rename = "UPort")]
        UPort,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortMedium {
        #[default]
        #[serde(rename = "Electrical")]
        Electrical,
        #[serde(rename = "Optical")]
        Optical,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortType {
        #[default]
        #[serde(rename = "BidirectionalPort")]
        BidirectionalPort,
        #[serde(rename = "DownstreamPort")]
        DownstreamPort,
        #[serde(rename = "InterswitchPort")]
        InterswitchPort,
        #[serde(rename = "ManagementPort")]
        ManagementPort,
        #[serde(rename = "UnconfiguredPort")]
        UnconfiguredPort,
        #[serde(rename = "UpstreamPort")]
        UpstreamPort,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct QoSTelemetryCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressPortBackpressureSupported"
        )]
        pub egress_port_backpressure_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionSupported"
        )]
        pub temporary_throughput_reduction_supported: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetPPB {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetPPBRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SFP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FiberConnectionType"
        )]
        pub fiber_connection_type: Option<crate::port::v1_11_0::FiberConnectionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediumType")]
        pub medium_type: Option<crate::port::v1_11_0::MediumType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedSFPTypes")]
        pub supported_sfp_types: Option<Vec<crate::port::v1_11_0::SFPType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::port::v1_11_0::SFPType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPType {
        #[default]
        #[serde(rename = "MiniSASHD")]
        MiniSASHD,
        #[serde(rename = "OSFP")]
        OSFP,
        #[serde(rename = "QSFP")]
        QSFP,
        #[serde(rename = "QSFP14")]
        QSFP14,
        #[serde(rename = "QSFP28")]
        QSFP28,
        #[serde(rename = "QSFP56")]
        QSFP56,
        #[serde(rename = "QSFPDD")]
        QSFPDD,
        #[serde(rename = "QSFPPlus")]
        QSFPPlus,
        #[serde(rename = "SFP")]
        SFP,
        #[serde(rename = "SFP28")]
        SFP28,
        #[serde(rename = "SFPDD")]
        SFPDD,
        #[serde(rename = "SFPPlus")]
        SFPPlus,
        #[serde(rename = "cSFP")]
        CSFP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SupportedEthernetCapabilities {
        #[default]
        #[serde(rename = "EEE")]
        EEE,
        #[serde(rename = "WakeOnLAN")]
        WakeOnLAN,
    }
}
pub mod v1_13_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port::v1_13_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.Reset")]
        pub port_reset: Option<crate::port::v1_13_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.ResetPPB")]
        pub port_reset_ppb: Option<crate::port::v1_13_0::ResetPPB>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Congestion")]
        pub congestion: Option<crate::port::v1_13_0::CXLCongestion>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceMode"
        )]
        pub connected_device_mode: Option<crate::port::v1_13_0::CXLConnectedDeviceMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceType"
        )]
        pub connected_device_type: Option<crate::port::v1_13_0::CXLConnectedDeviceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentPortConfigurationState"
        )]
        pub current_port_configuration_state:
            Option<crate::port::v1_13_0::CXLCurrentPortConfigurationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxLogicalDeviceCount"
        )]
        pub max_logical_device_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities: Option<crate::port::v1_13_0::CXLQoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedCXLModes")]
        pub supported_cxl_modes: Option<Vec<crate::port::v1_13_0::CXLSupportedCXLModes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionEnabled"
        )]
        pub temporary_throughput_reduction_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLCongestion {
        V011300(crate::port::v1_13_0::Congestion),
        V000001(crate::port::v1_13_0::CXLCongestionN1),
    }
    impl Default for CXLCongestion {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLCongestionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLConnectedDeviceMode {
        V011300(crate::port::v1_13_0::ConnectedDeviceMode),
        V000001(crate::port::v1_13_0::CXLConnectedDeviceModeN1),
    }
    impl Default for CXLConnectedDeviceMode {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLConnectedDeviceModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLConnectedDeviceType {
        V011300(crate::port::v1_13_0::ConnectedDeviceType),
        V000001(crate::port::v1_13_0::CXLConnectedDeviceTypeN1),
    }
    impl Default for CXLConnectedDeviceType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLConnectedDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLCurrentPortConfigurationState {
        V011300(crate::port::v1_13_0::CurrentPortConfigurationState),
        V000001(crate::port::v1_13_0::CXLCurrentPortConfigurationStateN1),
    }
    impl Default for CXLCurrentPortConfigurationState {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLCurrentPortConfigurationStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLQoSTelemetryCapabilities {
        V011300(crate::port::v1_13_0::QoSTelemetryCapabilities),
        V000001(crate::port::v1_13_0::CXLQoSTelemetryCapabilitiesN1),
    }
    impl Default for CXLQoSTelemetryCapabilities {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLQoSTelemetryCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLSupportedCXLModes {
        V011300(crate::port::v1_13_0::ConnectedDeviceMode),
        V000001(crate::port::v1_13_0::CXLSupportedCXLModesN1),
    }
    impl Default for CXLSupportedCXLModes {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLSupportedCXLModesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConfiguredNetworkLink {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredLinkSpeedGbps"
        )]
        pub configured_link_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfiguredWidth")]
        pub configured_width: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Congestion {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BackpressureSampleInterval"
        )]
        pub backpressure_sample_interval: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompletionCollectionInterval"
        )]
        pub completion_collection_interval: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CongestionTelemetryEnabled"
        )]
        pub congestion_telemetry_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressModeratePercentage"
        )]
        pub egress_moderate_percentage: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressSeverePercentage"
        )]
        pub egress_severe_percentage: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxSustainedRequestCmpBias"
        )]
        pub max_sustained_request_cmp_bias: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedDeviceMode {
        #[default]
        #[serde(rename = "CXL68BFlitAndVH")]
        CXL68BFlitAndVH,
        #[serde(rename = "CXLLatencyOptimized256BFlit")]
        CXLLatencyOptimized256BFlit,
        #[serde(rename = "Disconnected")]
        Disconnected,
        #[serde(rename = "PBR")]
        PBR,
        #[serde(rename = "RCD")]
        RCD,
        #[serde(rename = "Standard256BFlit")]
        Standard256BFlit,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedDeviceType {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "PCIeDevice")]
        PCIeDevice,
        #[serde(rename = "Type1")]
        Type1,
        #[serde(rename = "Type2")]
        Type2,
        #[serde(rename = "Type3MLD")]
        Type3MLD,
        #[serde(rename = "Type3SLD")]
        Type3SLD,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentPortConfigurationState {
        #[default]
        #[serde(rename = "BindInProgress")]
        BindInProgress,
        #[serde(rename = "DSP")]
        DSP,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "FabricLink")]
        FabricLink,
        #[serde(rename = "Reserved")]
        Reserved,
        #[serde(rename = "USP")]
        USP,
        #[serde(rename = "UnbindInProgress")]
        UnbindInProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EthernetProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedMACAddresses"
        )]
        pub associated_mac_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration:
            Option<crate::port::v1_13_0::EthernetPropertiesFlowControlConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::port::v1_13_0::EthernetPropertiesFlowControlStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPReceive")]
        pub lldp_receive: Option<crate::port::v1_13_0::EthernetPropertiesLLDPReceive>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPTransmit")]
        pub lldp_transmit: Option<crate::port::v1_13_0::EthernetPropertiesLLDPTransmit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::port::v1_13_0::EthernetPropertiesSupportedEthernetCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesFlowControlConfiguration {
        V011300(crate::port::v1_13_0::FlowControl),
        V000001(crate::port::v1_13_0::EthernetPropertiesFlowControlConfigurationN1),
    }
    impl Default for EthernetPropertiesFlowControlConfiguration {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesFlowControlConfigurationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesFlowControlStatus {
        V011300(crate::port::v1_13_0::FlowControl),
        V000001(crate::port::v1_13_0::EthernetPropertiesFlowControlStatusN1),
    }
    impl Default for EthernetPropertiesFlowControlStatus {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesFlowControlStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesLLDPReceive {
        V011300(crate::port::v1_13_0::LLDPReceive),
        V000001(crate::port::v1_13_0::EthernetPropertiesLLDPReceiveN1),
    }
    impl Default for EthernetPropertiesLLDPReceive {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesLLDPReceiveN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesLLDPTransmit {
        V011300(crate::port::v1_13_0::LLDPTransmit),
        V000001(crate::port::v1_13_0::EthernetPropertiesLLDPTransmitN1),
    }
    impl Default for EthernetPropertiesLLDPTransmit {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesLLDPTransmitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesSupportedEthernetCapabilities {
        V011300(crate::port::v1_13_0::SupportedEthernetCapabilities),
        V000001(crate::port::v1_13_0::EthernetPropertiesSupportedEthernetCapabilitiesN1),
    }
    impl Default for EthernetPropertiesSupportedEthernetCapabilities {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesSupportedEthernetCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FiberConnectionType {
        #[default]
        #[serde(rename = "MultiMode")]
        MultiMode,
        #[serde(rename = "SingleMode")]
        SingleMode,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannelProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedWorldWideNames"
        )]
        pub associated_world_wide_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricName")]
        pub fabric_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberDiscoveredRemotePorts"
        )]
        pub number_discovered_remote_ports: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortConnectionType")]
        pub port_connection_type:
            Option<crate::port::v1_13_0::FibreChannelPropertiesPortConnectionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FibreChannelPropertiesPortConnectionType {
        V011300(crate::port::v1_13_0::PortConnectionType),
        V000001(crate::port::v1_13_0::FibreChannelPropertiesPortConnectionTypeN1),
    }
    impl Default for FibreChannelPropertiesPortConnectionType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FibreChannelPropertiesPortConnectionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FlowControl {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RX")]
        RX,
        #[serde(rename = "TX")]
        TX,
        #[serde(rename = "TX_RX")]
        TXRX,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FunctionMaxBandwidth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocationPercent")]
        pub allocation_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FunctionMinBandwidth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocationPercent")]
        pub allocation_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LPRT")]
        pub lprt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MPRT")]
        pub mprt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VCAT")]
        pub vcat: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IEEE802IdSubtype {
        #[default]
        #[serde(rename = "AgentId")]
        AgentId,
        #[serde(rename = "ChassisComp")]
        ChassisComp,
        #[serde(rename = "IfAlias")]
        IfAlias,
        #[serde(rename = "IfName")]
        IfName,
        #[serde(rename = "LocalAssign")]
        LocalAssign,
        #[serde(rename = "MacAddr")]
        MacAddr,
        #[serde(rename = "NetworkAddr")]
        NetworkAddr,
        #[serde(rename = "NotTransmitted")]
        NotTransmitted,
        #[serde(rename = "PortComp")]
        PortComp,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InfiniBandProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNodeGUIDs"
        )]
        pub associated_node_gu_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPortGUIDs"
        )]
        pub associated_port_gu_ids: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedSystemGUIDs"
        )]
        pub associated_system_gu_ids: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LLDPReceive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_13_0::LLDPReceiveChassisIdSubtype>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv4"
        )]
        pub management_address_ipv4: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv6"
        )]
        pub management_address_ipv6: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressMAC"
        )]
        pub management_address_mac: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementVlanId")]
        pub management_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortIdSubtype")]
        pub port_id_subtype: Option<crate::port::v1_13_0::LLDPReceivePortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_13_0::LLDPReceiveSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPReceiveChassisIdSubtype {
        V011300(crate::port::v1_13_0::IEEE802IdSubtype),
        V000001(crate::port::v1_13_0::LLDPReceiveChassisIdSubtypeN1),
    }
    impl Default for LLDPReceiveChassisIdSubtype {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPReceiveChassisIdSubtypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPReceivePortIdSubtype {
        V011300(crate::port::v1_13_0::IEEE802IdSubtype),
        V000001(crate::port::v1_13_0::LLDPReceivePortIdSubtypeN1),
    }
    impl Default for LLDPReceivePortIdSubtype {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPReceivePortIdSubtypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPReceiveSystemCapabilities {
        V011300(crate::port::v1_13_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_13_0::LLDPReceiveSystemCapabilitiesN1),
    }
    impl Default for LLDPReceiveSystemCapabilities {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPReceiveSystemCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPSystemCapabilities {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DOCSISCableDevice")]
        DOCSISCableDevice,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "Repeater")]
        Repeater,
        #[serde(rename = "Router")]
        Router,
        #[serde(rename = "Station")]
        Station,
        #[serde(rename = "Telephone")]
        Telephone,
        #[serde(rename = "WLANAccessPoint")]
        WLANAccessPoint,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LLDPTransmit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_13_0::LLDPTransmitChassisIdSubtype>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv4"
        )]
        pub management_address_ipv4: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressIPv6"
        )]
        pub management_address_ipv6: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementAddressMAC"
        )]
        pub management_address_mac: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagementVlanId")]
        pub management_vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortIdSubtype")]
        pub port_id_subtype: Option<crate::port::v1_13_0::LLDPTransmitPortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_13_0::LLDPTransmitSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPTransmitChassisIdSubtype {
        V011300(crate::port::v1_13_0::IEEE802IdSubtype),
        V000001(crate::port::v1_13_0::LLDPTransmitChassisIdSubtypeN1),
    }
    impl Default for LLDPTransmitChassisIdSubtype {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPTransmitChassisIdSubtypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPTransmitPortIdSubtype {
        V011300(crate::port::v1_13_0::IEEE802IdSubtype),
        V000001(crate::port::v1_13_0::LLDPTransmitPortIdSubtypeN1),
    }
    impl Default for LLDPTransmitPortIdSubtype {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPTransmitPortIdSubtypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPTransmitSystemCapabilities {
        V011300(crate::port::v1_13_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_13_0::LLDPTransmitSystemCapabilitiesN1),
    }
    impl Default for LLDPTransmitSystemCapabilities {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LLDPTransmitSystemCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LinkConfiguration {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiationCapable"
        )]
        pub auto_speed_negotiation_capable: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiationEnabled"
        )]
        pub auto_speed_negotiation_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableLinkSpeedGbps"
        )]
        pub capable_link_speed_gbps: Option<Vec<f64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredNetworkLinks"
        )]
        pub configured_network_links:
            Option<Vec<crate::port::v1_13_0::LinkConfigurationConfiguredNetworkLinks>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkConfigurationConfiguredNetworkLinks {
        V011300(crate::port::v1_13_0::ConfiguredNetworkLink),
        V000001(crate::port::v1_13_0::LinkConfigurationConfiguredNetworkLinksN1),
    }
    impl Default for LinkConfigurationConfiguredNetworkLinks {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkConfigurationConfiguredNetworkLinksN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "GenZ")]
        GenZ,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
        #[serde(rename = "PCIe")]
        PCIe,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
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
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Training")]
        Training,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedEndpoints"
        )]
        pub associated_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedEndpoints@odata.count"
        )]
        pub associated_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables@odata.count")]
        pub cables_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitchPorts"
        )]
        pub connected_switch_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitchPorts@odata.count"
        )]
        pub connected_switch_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedSwitches")]
        pub connected_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedSwitches@odata.count"
        )]
        pub connected_switches_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediumType {
        #[default]
        #[serde(rename = "Copper")]
        Copper,
        #[serde(rename = "FiberOptic")]
        FiberOptic,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Port {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port::v1_13_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveWidth")]
        pub active_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableProtocolVersions"
        )]
        pub capable_protocol_versions: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentProtocolVersion"
        )]
        pub current_protocol_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentSpeedGbps")]
        pub current_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::port::v1_13_0::PortCXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::port::v1_13_0::PortDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::port::v1_13_0::PortEthernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port::v1_13_0::PortFibreChannel>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMaxBandwidth"
        )]
        pub function_max_bandwidth: Option<Vec<crate::port::v1_13_0::FunctionMaxBandwidth>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMinBandwidth"
        )]
        pub function_min_bandwidth: Option<Vec<crate::port::v1_13_0::FunctionMinBandwidth>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port::v1_13_0::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::port::v1_13_0::PortInfiniBand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkConfiguration")]
        pub link_configuration: Option<Vec<crate::port::v1_13_0::LinkConfiguration>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::port::v1_13_0::PortLinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkState")]
        pub link_state: Option<crate::port::v1_13_0::LinkState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::port::v1_13_0::LinkStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkTransitionIndicator"
        )]
        pub link_transition_indicator: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::port::v1_13_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxFrameSize")]
        pub max_frame_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedGbps")]
        pub max_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::port::v1_13_0::PortMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMedium")]
        pub port_medium: Option<crate::port::v1_13_0::PortPortMedium>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortProtocol")]
        pub port_protocol: Option<crate::port::v1_13_0::PortPortProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortType")]
        pub port_type: Option<crate::port::v1_13_0::PortPortType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemotePortId")]
        pub remote_port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SFP")]
        pub sfp: Option<crate::port::v1_13_0::PortSFP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
        pub width: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortCXL {
        V011300(crate::port::v1_13_0::CXL),
        V000001(crate::port::v1_13_0::PortCXLN1),
    }
    impl Default for PortCXL {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortCXLN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortConnectionType {
        #[default]
        #[serde(rename = "DPort")]
        DPort,
        #[serde(rename = "EPort")]
        EPort,
        #[serde(rename = "EXPort")]
        EXPort,
        #[serde(rename = "ExtenderFabric")]
        ExtenderFabric,
        #[serde(rename = "FLPort")]
        FLPort,
        #[serde(rename = "FPort")]
        FPort,
        #[serde(rename = "GPort")]
        GPort,
        #[serde(rename = "Generic")]
        Generic,
        #[serde(rename = "NLPort")]
        NLPort,
        #[serde(rename = "NPPort")]
        NPPort,
        #[serde(rename = "NPort")]
        NPort,
        #[serde(rename = "NotConnected")]
        NotConnected,
        #[serde(rename = "PointToPoint")]
        PointToPoint,
        #[serde(rename = "PrivateLoop")]
        PrivateLoop,
        #[serde(rename = "PublicLoop")]
        PublicLoop,
        #[serde(rename = "TEPort")]
        TEPort,
        #[serde(rename = "UPort")]
        UPort,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortDescription {
        V000001(crate::port::v1_13_0::PortDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PortDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortEthernet {
        V011300(crate::port::v1_13_0::EthernetProperties),
        V000001(crate::port::v1_13_0::PortEthernetN1),
    }
    impl Default for PortEthernet {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortEthernetN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortFibreChannel {
        V011300(crate::port::v1_13_0::FibreChannelProperties),
        V000001(crate::port::v1_13_0::PortFibreChannelN1),
    }
    impl Default for PortFibreChannel {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortFibreChannelN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortInfiniBand {
        V011300(crate::port::v1_13_0::InfiniBandProperties),
        V000001(crate::port::v1_13_0::PortInfiniBandN1),
    }
    impl Default for PortInfiniBand {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortInfiniBandN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortLinkNetworkTechnology {
        V011300(crate::port::v1_13_0::LinkNetworkTechnology),
        V000001(crate::port::v1_13_0::PortLinkNetworkTechnologyN1),
    }
    impl Default for PortLinkNetworkTechnology {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkNetworkTechnologyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortMedium {
        #[default]
        #[serde(rename = "Electrical")]
        Electrical,
        #[serde(rename = "Optical")]
        Optical,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortMetrics {
        V000001(crate::port::v1_13_0::PortMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for PortMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortPortMedium {
        V011300(crate::port::v1_13_0::PortMedium),
        V000001(crate::port::v1_13_0::PortPortMediumN1),
    }
    impl Default for PortPortMedium {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortPortMediumN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortPortProtocol {
        V000001(crate::port::v1_13_0::PortPortProtocolN1),
        ProtocolProtocol(crate::protocol::Protocol),
    }
    impl Default for PortPortProtocol {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortPortProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortPortType {
        V011300(crate::port::v1_13_0::PortType),
        V000001(crate::port::v1_13_0::PortPortTypeN1),
    }
    impl Default for PortPortType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortPortTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortSFP {
        V011300(crate::port::v1_13_0::SFP),
        V000001(crate::port::v1_13_0::PortSFPN1),
    }
    impl Default for PortSFP {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortSFPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortType {
        #[default]
        #[serde(rename = "BidirectionalPort")]
        BidirectionalPort,
        #[serde(rename = "DownstreamPort")]
        DownstreamPort,
        #[serde(rename = "InterswitchPort")]
        InterswitchPort,
        #[serde(rename = "ManagementPort")]
        ManagementPort,
        #[serde(rename = "UnconfiguredPort")]
        UnconfiguredPort,
        #[serde(rename = "UpstreamPort")]
        UpstreamPort,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct QoSTelemetryCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EgressPortBackpressureSupported"
        )]
        pub egress_port_backpressure_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionSupported"
        )]
        pub temporary_throughput_reduction_supported: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetPPB {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetPPBRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SFP {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FiberConnectionType"
        )]
        pub fiber_connection_type: Option<crate::port::v1_13_0::SFPFiberConnectionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediumType")]
        pub medium_type: Option<crate::port::v1_13_0::SFPMediumType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedSFPTypes")]
        pub supported_sfp_types: Option<Vec<crate::port::v1_13_0::SFPSupportedSFPTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::port::v1_13_0::SFPTypeAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorOUI")]
        pub vendor_oui: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPFiberConnectionType {
        V011300(crate::port::v1_13_0::FiberConnectionType),
        V000001(crate::port::v1_13_0::SFPFiberConnectionTypeN1),
    }
    impl Default for SFPFiberConnectionType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPFiberConnectionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPMediumType {
        V011300(crate::port::v1_13_0::MediumType),
        V000001(crate::port::v1_13_0::SFPMediumTypeN1),
    }
    impl Default for SFPMediumType {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPMediumTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPSupportedSFPTypes {
        V011300(crate::port::v1_13_0::SFPType),
        V000001(crate::port::v1_13_0::SFPSupportedSFPTypesN1),
    }
    impl Default for SFPSupportedSFPTypes {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPSupportedSFPTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPType {
        #[default]
        #[serde(rename = "MiniSASHD")]
        MiniSASHD,
        #[serde(rename = "OSFP")]
        OSFP,
        #[serde(rename = "QSFP")]
        QSFP,
        #[serde(rename = "QSFP14")]
        QSFP14,
        #[serde(rename = "QSFP28")]
        QSFP28,
        #[serde(rename = "QSFP56")]
        QSFP56,
        #[serde(rename = "QSFPDD")]
        QSFPDD,
        #[serde(rename = "QSFPPlus")]
        QSFPPlus,
        #[serde(rename = "SFP")]
        SFP,
        #[serde(rename = "SFP28")]
        SFP28,
        #[serde(rename = "SFPDD")]
        SFPDD,
        #[serde(rename = "SFPPlus")]
        SFPPlus,
        #[serde(rename = "cSFP")]
        CSFP,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPTypeAnony {
        V011300(crate::port::v1_13_0::SFPType),
        V000001(crate::port::v1_13_0::SFPTypeN1),
    }
    impl Default for SFPTypeAnony {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SupportedEthernetCapabilities {
        #[default]
        #[serde(rename = "EEE")]
        EEE,
        #[serde(rename = "WakeOnLAN")]
        WakeOnLAN,
    }
}
