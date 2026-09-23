pub type Port = crate::port::v1_20_0::Port;
pub mod v1_19_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port::v1_19_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.Reset")]
        pub port_reset: Option<crate::port::v1_19_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.ResetPPB")]
        pub port_reset_ppb: Option<crate::port::v1_19_0::ResetPPB>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Congestion")]
        pub congestion: Option<crate::port::v1_19_0::CXLCongestion>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceMode"
        )]
        pub connected_device_mode: Option<crate::port::v1_19_0::CXLConnectedDeviceMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceType"
        )]
        pub connected_device_type: Option<crate::port::v1_19_0::CXLConnectedDeviceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentPortConfigurationState"
        )]
        pub current_port_configuration_state:
            Option<crate::port::v1_19_0::CXLCurrentPortConfigurationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxLogicalDeviceCount"
        )]
        pub max_logical_device_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities: Option<crate::port::v1_19_0::CXLQoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedCXLModes")]
        pub supported_cxl_modes: Option<Vec<crate::port::v1_19_0::CXLSupportedCXLModes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionEnabled"
        )]
        pub temporary_throughput_reduction_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLCongestion {
        V011900(crate::port::v1_19_0::Congestion),
        V000001(crate::port::v1_19_0::CXLCongestionN1),
    }
    impl Default for CXLCongestion {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::ConnectedDeviceMode),
        V000001(crate::port::v1_19_0::CXLConnectedDeviceModeN1),
    }
    impl Default for CXLConnectedDeviceMode {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::ConnectedDeviceType),
        V000001(crate::port::v1_19_0::CXLConnectedDeviceTypeN1),
    }
    impl Default for CXLConnectedDeviceType {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::CurrentPortConfigurationState),
        V000001(crate::port::v1_19_0::CXLCurrentPortConfigurationStateN1),
    }
    impl Default for CXLCurrentPortConfigurationState {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::QoSTelemetryCapabilities),
        V000001(crate::port::v1_19_0::CXLQoSTelemetryCapabilitiesN1),
    }
    impl Default for CXLQoSTelemetryCapabilities {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::ConnectedDeviceMode),
        V000001(crate::port::v1_19_0::CXLSupportedCXLModesN1),
    }
    impl Default for CXLSupportedCXLModes {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        #[serde(rename = "PBRComponent")]
        PBRComponent,
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
            Option<crate::port::v1_19_0::EthernetPropertiesFlowControlConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::port::v1_19_0::EthernetPropertiesFlowControlStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPReceive")]
        pub lldp_receive: Option<crate::port::v1_19_0::EthernetPropertiesLLDPReceive>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPTransmit")]
        pub lldp_transmit: Option<crate::port::v1_19_0::EthernetPropertiesLLDPTransmit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoE")]
        pub po_e: Option<crate::port::v1_19_0::EthernetPropertiesPoE>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::port::v1_19_0::EthernetPropertiesSupportedEthernetCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesFlowControlConfiguration {
        V011900(crate::port::v1_19_0::FlowControl),
        V000001(crate::port::v1_19_0::EthernetPropertiesFlowControlConfigurationN1),
    }
    impl Default for EthernetPropertiesFlowControlConfiguration {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::FlowControl),
        V000001(crate::port::v1_19_0::EthernetPropertiesFlowControlStatusN1),
    }
    impl Default for EthernetPropertiesFlowControlStatus {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::LLDPReceive),
        V000001(crate::port::v1_19_0::EthernetPropertiesLLDPReceiveN1),
    }
    impl Default for EthernetPropertiesLLDPReceive {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::LLDPTransmit),
        V000001(crate::port::v1_19_0::EthernetPropertiesLLDPTransmitN1),
    }
    impl Default for EthernetPropertiesLLDPTransmit {
        fn default() -> Self {
            Self::V011900(Default::default())
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
    pub enum EthernetPropertiesPoE {
        V011900(crate::port::v1_19_0::PoE),
        V000001(crate::port::v1_19_0::EthernetPropertiesPoEN1),
    }
    impl Default for EthernetPropertiesPoE {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesPoEN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesSupportedEthernetCapabilities {
        V011900(crate::port::v1_19_0::SupportedEthernetCapabilities),
        V000001(crate::port::v1_19_0::EthernetPropertiesSupportedEthernetCapabilitiesN1),
    }
    impl Default for EthernetPropertiesSupportedEthernetCapabilities {
        fn default() -> Self {
            Self::V011900(Default::default())
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
            Option<crate::port::v1_19_0::FibreChannelPropertiesPortConnectionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FibreChannelPropertiesPortConnectionType {
        V011900(crate::port::v1_19_0::PortConnectionType),
        V000001(crate::port::v1_19_0::FibreChannelPropertiesPortConnectionTypeN1),
    }
    impl Default for FibreChannelPropertiesPortConnectionType {
        fn default() -> Self {
            Self::V011900(Default::default())
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
    pub enum HostDeviceType {
        #[default]
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv4"
        )]
        pub additional_management_addresses_ipv4: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv6"
        )]
        pub additional_management_addresses_ipv6: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_19_0::LLDPReceiveChassisIdSubtype>,
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
        pub port_id_subtype: Option<crate::port::v1_19_0::LLDPReceivePortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_19_0::LLDPReceiveSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPReceiveChassisIdSubtype {
        V011900(crate::port::v1_19_0::IEEE802IdSubtype),
        V000001(crate::port::v1_19_0::LLDPReceiveChassisIdSubtypeN1),
    }
    impl Default for LLDPReceiveChassisIdSubtype {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::IEEE802IdSubtype),
        V000001(crate::port::v1_19_0::LLDPReceivePortIdSubtypeN1),
    }
    impl Default for LLDPReceivePortIdSubtype {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_19_0::LLDPReceiveSystemCapabilitiesN1),
    }
    impl Default for LLDPReceiveSystemCapabilities {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv4"
        )]
        pub additional_management_addresses_ipv4: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv6"
        )]
        pub additional_management_addresses_ipv6: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_19_0::LLDPTransmitChassisIdSubtype>,
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
        pub port_id_subtype: Option<crate::port::v1_19_0::LLDPTransmitPortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_19_0::LLDPTransmitSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPTransmitChassisIdSubtype {
        V011900(crate::port::v1_19_0::IEEE802IdSubtype),
        V000001(crate::port::v1_19_0::LLDPTransmitChassisIdSubtypeN1),
    }
    impl Default for LLDPTransmitChassisIdSubtype {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::IEEE802IdSubtype),
        V000001(crate::port::v1_19_0::LLDPTransmitPortIdSubtypeN1),
    }
    impl Default for LLDPTransmitPortIdSubtype {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_19_0::LLDPTransmitSystemCapabilitiesN1),
    }
    impl Default for LLDPTransmitSystemCapabilities {
        fn default() -> Self {
            Self::V011900(Default::default())
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
            Option<Vec<crate::port::v1_19_0::LinkConfigurationConfiguredNetworkLinks>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology:
            Option<crate::port::v1_19_0::LinkConfigurationLinkNetworkTechnology>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkConfigurationConfiguredNetworkLinks {
        V011900(crate::port::v1_19_0::ConfiguredNetworkLink),
        V000001(crate::port::v1_19_0::LinkConfigurationConfiguredNetworkLinksN1),
    }
    impl Default for LinkConfigurationConfiguredNetworkLinks {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkConfigurationConfiguredNetworkLinksN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkConfigurationLinkNetworkTechnology {
        V011900(crate::port::v1_19_0::LinkNetworkTechnology),
        V000001(crate::port::v1_19_0::LinkConfigurationLinkNetworkTechnologyN1),
    }
    impl Default for LinkConfigurationLinkNetworkTechnology {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkConfigurationLinkNetworkTechnologyN1 {
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPorts"
        )]
        pub associated_physical_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPorts@odata.count"
        )]
        pub associated_physical_ports_odata_count: Option<i64>,
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
    pub struct PCIeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReferenceClockMode")]
        pub reference_clock_mode: Option<crate::port::v1_19_0::PCIePropertiesReferenceClockMode>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIePropertiesReferenceClockMode {
        V011900(crate::port::v1_19_0::PCIeReferenceClockMode),
        V000001(crate::port::v1_19_0::PCIePropertiesReferenceClockModeN1),
    }
    impl Default for PCIePropertiesReferenceClockMode {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIePropertiesReferenceClockModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeReferenceClockMode {
        #[default]
        #[serde(rename = "CommonClock")]
        CommonClock,
        #[serde(rename = "SeparateClock")]
        SeparateClock,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PoE {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapablePowerWatts")]
        pub capable_power_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredPowerWatts"
        )]
        pub configured_power_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectionStatus")]
        pub detection_status: Option<crate::port::v1_19_0::PoEDetectionStatusAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerClass")]
        pub power_class: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::port::v1_19_0::PoEPowerModeAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Standard")]
        pub standard: Option<crate::port::v1_19_0::PoEStandardAnony>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEDetectionStatus {
        #[default]
        #[serde(rename = "DeliveringPower")]
        DeliveringPower,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Fault")]
        Fault,
        #[serde(rename = "OtherFault")]
        OtherFault,
        #[serde(rename = "Searching")]
        Searching,
        #[serde(rename = "Test")]
        Test,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEDetectionStatusAnony {
        V011900(crate::port::v1_19_0::PoEDetectionStatus),
        V000001(crate::port::v1_19_0::PoEDetectionStatusN1),
    }
    impl Default for PoEDetectionStatusAnony {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEDetectionStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEPowerMode {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "PD")]
        PD,
        #[serde(rename = "PSE")]
        PSE,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEPowerModeAnony {
        V011900(crate::port::v1_19_0::PoEPowerMode),
        V000001(crate::port::v1_19_0::PoEPowerModeN1),
    }
    impl Default for PoEPowerModeAnony {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEPowerModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEStandard {
        #[default]
        #[serde(rename = "IEEE8023af")]
        IEEE8023af,
        #[serde(rename = "IEEE8023at")]
        IEEE8023at,
        #[serde(rename = "IEEE8023bt")]
        IEEE8023bt,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEStandardAnony {
        V011900(crate::port::v1_19_0::PoEStandard),
        V000001(crate::port::v1_19_0::PoEStandardN1),
    }
    impl Default for PoEStandardAnony {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEStandardN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Port {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port::v1_19_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveWidth")]
        pub active_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPort"
        )]
        pub associated_physical_port: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableProtocolVersions"
        )]
        pub capable_protocol_versions: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredSpeedGbps"
        )]
        pub configured_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfiguredWidth")]
        pub configured_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentProtocolVersion"
        )]
        pub current_protocol_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentSpeedGbps")]
        pub current_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::port::v1_19_0::PortCXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::port::v1_19_0::PortDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::port::v1_19_0::PortEthernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port::v1_19_0::PortFibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirstLane")]
        pub first_lane: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMaxBandwidth"
        )]
        pub function_max_bandwidth: Option<Vec<crate::port::v1_19_0::FunctionMaxBandwidth>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMinBandwidth"
        )]
        pub function_min_bandwidth: Option<Vec<crate::port::v1_19_0::FunctionMinBandwidth>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port::v1_19_0::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostDevice")]
        pub host_device: Option<crate::port::v1_19_0::PortHostDevice>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::port::v1_19_0::PortInfiniBand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsAggregation")]
        pub is_aggregation: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSplit")]
        pub is_split: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkConfiguration")]
        pub link_configuration: Option<Vec<crate::port::v1_19_0::LinkConfiguration>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::port::v1_19_0::PortLinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkState")]
        pub link_state: Option<crate::port::v1_19_0::LinkState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::port::v1_19_0::LinkStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkTransitionIndicator"
        )]
        pub link_transition_indicator: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::port::v1_19_0::Links>,
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
        pub metrics: Option<crate::port::v1_19_0::PortMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIe")]
        pub pcie: Option<crate::port::v1_19_0::PortPCIe>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMedium")]
        pub port_medium: Option<crate::port::v1_19_0::PortPortMedium>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortProtocol")]
        pub port_protocol: Option<crate::port::v1_19_0::PortPortProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortType")]
        pub port_type: Option<crate::port::v1_19_0::PortPortType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemotePortId")]
        pub remote_port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SFP")]
        pub sfp: Option<crate::port::v1_19_0::PortSFP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UALink")]
        pub ua_link: Option<crate::port::v1_19_0::UALink>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
        pub width: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortCXL {
        V011900(crate::port::v1_19_0::CXL),
        V000001(crate::port::v1_19_0::PortCXLN1),
    }
    impl Default for PortCXL {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V000001(crate::port::v1_19_0::PortDescriptionN1),
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
        V011900(crate::port::v1_19_0::EthernetProperties),
        V000001(crate::port::v1_19_0::PortEthernetN1),
    }
    impl Default for PortEthernet {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::FibreChannelProperties),
        V000001(crate::port::v1_19_0::PortFibreChannelN1),
    }
    impl Default for PortFibreChannel {
        fn default() -> Self {
            Self::V011900(Default::default())
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
    pub enum PortHostDevice {
        V011900(crate::port::v1_19_0::HostDeviceType),
        V000001(crate::port::v1_19_0::PortHostDeviceN1),
    }
    impl Default for PortHostDevice {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortHostDeviceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortInfiniBand {
        V011900(crate::port::v1_19_0::InfiniBandProperties),
        V000001(crate::port::v1_19_0::PortInfiniBandN1),
    }
    impl Default for PortInfiniBand {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::LinkNetworkTechnology),
        V000001(crate::port::v1_19_0::PortLinkNetworkTechnologyN1),
    }
    impl Default for PortLinkNetworkTechnology {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V000001(crate::port::v1_19_0::PortMetricsN1),
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
    pub enum PortPCIe {
        V011900(crate::port::v1_19_0::PCIeProperties),
        V000001(crate::port::v1_19_0::PortPCIeN1),
    }
    impl Default for PortPCIe {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortPCIeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortPortMedium {
        V011900(crate::port::v1_19_0::PortMedium),
        V000001(crate::port::v1_19_0::PortPortMediumN1),
    }
    impl Default for PortPortMedium {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V000001(crate::port::v1_19_0::PortPortProtocolN1),
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
        V011900(crate::port::v1_19_0::PortType),
        V000001(crate::port::v1_19_0::PortPortTypeN1),
    }
    impl Default for PortPortType {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::SFP),
        V000001(crate::port::v1_19_0::PortSFPN1),
    }
    impl Default for PortSFP {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateCode")]
        pub date_code: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FiberConnectionType"
        )]
        pub fiber_connection_type: Option<crate::port::v1_19_0::SFPFiberConnectionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementInterfaceType"
        )]
        pub management_interface_type: Option<crate::port::v1_19_0::SFPManagementInterfaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediumType")]
        pub medium_type: Option<crate::port::v1_19_0::SFPMediumType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedSFPTypes")]
        pub supported_sfp_types: Option<Vec<crate::port::v1_19_0::SFPSupportedSFPTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::port::v1_19_0::SFPTypeAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorOUI")]
        pub vendor_oui: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPFiberConnectionType {
        V011900(crate::port::v1_19_0::FiberConnectionType),
        V000001(crate::port::v1_19_0::SFPFiberConnectionTypeN1),
    }
    impl Default for SFPFiberConnectionType {
        fn default() -> Self {
            Self::V011900(Default::default())
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
    pub enum SFPManagementInterfaceType {
        V011900(crate::port::v1_19_0::TransceiverManagementInterfaceType),
        V000001(crate::port::v1_19_0::SFPManagementInterfaceTypeN1),
    }
    impl Default for SFPManagementInterfaceType {
        fn default() -> Self {
            Self::V011900(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPManagementInterfaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPMediumType {
        V011900(crate::port::v1_19_0::MediumType),
        V000001(crate::port::v1_19_0::SFPMediumTypeN1),
    }
    impl Default for SFPMediumType {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        V011900(crate::port::v1_19_0::SFPType),
        V000001(crate::port::v1_19_0::SFPSupportedSFPTypesN1),
    }
    impl Default for SFPSupportedSFPTypes {
        fn default() -> Self {
            Self::V011900(Default::default())
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
        #[serde(rename = "CDFP")]
        CDFP,
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
        V011900(crate::port::v1_19_0::SFPType),
        V000001(crate::port::v1_19_0::SFPTypeN1),
    }
    impl Default for SFPTypeAnony {
        fn default() -> Self {
            Self::V011900(Default::default())
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransceiverManagementInterfaceType {
        #[default]
        #[serde(rename = "CMIS")]
        CMIS,
        #[serde(rename = "SFP")]
        SFP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UALink {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModeEnabled"
        )]
        pub authentication_mode_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Generation")]
        pub generation: Option<crate::port::v1_19_0::UALinkGeneration>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonStrictOrderingModeEnabled"
        )]
        pub non_strict_ordering_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UPLIWatchdogTimerMicroseconds"
        )]
        pub upli_watchdog_timer_microseconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkGeneration {
        #[default]
        #[serde(rename = "UALink128G")]
        UALink128G,
        #[serde(rename = "UALink200G")]
        UALink200G,
    }
}
pub mod v1_20_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port::v1_20_0::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.Reset")]
        pub port_reset: Option<crate::port::v1_20_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Port.ResetPPB")]
        pub port_reset_ppb: Option<crate::port::v1_20_0::ResetPPB>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Port.ResetTransceivers"
        )]
        pub port_reset_transceivers: Option<crate::port::v1_20_0::ResetTransceivers>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Congestion")]
        pub congestion: Option<crate::port::v1_20_0::CXLCongestion>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceMode"
        )]
        pub connected_device_mode: Option<crate::port::v1_20_0::CXLConnectedDeviceMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedDeviceType"
        )]
        pub connected_device_type: Option<crate::port::v1_20_0::CXLConnectedDeviceType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentPortConfigurationState"
        )]
        pub current_port_configuration_state:
            Option<crate::port::v1_20_0::CXLCurrentPortConfigurationState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxLogicalDeviceCount"
        )]
        pub max_logical_device_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities: Option<crate::port::v1_20_0::CXLQoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedCXLModes")]
        pub supported_cxl_modes: Option<Vec<crate::port::v1_20_0::CXLSupportedCXLModes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemporaryThroughputReductionEnabled"
        )]
        pub temporary_throughput_reduction_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CXLCongestion {
        V012000(crate::port::v1_20_0::Congestion),
        V000001(crate::port::v1_20_0::CXLCongestionN1),
    }
    impl Default for CXLCongestion {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::ConnectedDeviceMode),
        V000001(crate::port::v1_20_0::CXLConnectedDeviceModeN1),
    }
    impl Default for CXLConnectedDeviceMode {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::ConnectedDeviceType),
        V000001(crate::port::v1_20_0::CXLConnectedDeviceTypeN1),
    }
    impl Default for CXLConnectedDeviceType {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::CurrentPortConfigurationState),
        V000001(crate::port::v1_20_0::CXLCurrentPortConfigurationStateN1),
    }
    impl Default for CXLCurrentPortConfigurationState {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::QoSTelemetryCapabilities),
        V000001(crate::port::v1_20_0::CXLQoSTelemetryCapabilitiesN1),
    }
    impl Default for CXLQoSTelemetryCapabilities {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::ConnectedDeviceMode),
        V000001(crate::port::v1_20_0::CXLSupportedCXLModesN1),
    }
    impl Default for CXLSupportedCXLModes {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        #[serde(rename = "PBRComponent")]
        PBRComponent,
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
            Option<crate::port::v1_20_0::EthernetPropertiesFlowControlConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::port::v1_20_0::EthernetPropertiesFlowControlStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPEnabled")]
        pub lldp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPReceive")]
        pub lldp_receive: Option<crate::port::v1_20_0::EthernetPropertiesLLDPReceive>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLDPTransmit")]
        pub lldp_transmit: Option<crate::port::v1_20_0::EthernetPropertiesLLDPTransmit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoE")]
        pub po_e: Option<crate::port::v1_20_0::EthernetPropertiesPoE>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::port::v1_20_0::EthernetPropertiesSupportedEthernetCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesFlowControlConfiguration {
        V012000(crate::port::v1_20_0::FlowControl),
        V000001(crate::port::v1_20_0::EthernetPropertiesFlowControlConfigurationN1),
    }
    impl Default for EthernetPropertiesFlowControlConfiguration {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::FlowControl),
        V000001(crate::port::v1_20_0::EthernetPropertiesFlowControlStatusN1),
    }
    impl Default for EthernetPropertiesFlowControlStatus {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::LLDPReceive),
        V000001(crate::port::v1_20_0::EthernetPropertiesLLDPReceiveN1),
    }
    impl Default for EthernetPropertiesLLDPReceive {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::LLDPTransmit),
        V000001(crate::port::v1_20_0::EthernetPropertiesLLDPTransmitN1),
    }
    impl Default for EthernetPropertiesLLDPTransmit {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    pub enum EthernetPropertiesPoE {
        V012000(crate::port::v1_20_0::PoE),
        V000001(crate::port::v1_20_0::EthernetPropertiesPoEN1),
    }
    impl Default for EthernetPropertiesPoE {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EthernetPropertiesPoEN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EthernetPropertiesSupportedEthernetCapabilities {
        V012000(crate::port::v1_20_0::SupportedEthernetCapabilities),
        V000001(crate::port::v1_20_0::EthernetPropertiesSupportedEthernetCapabilitiesN1),
    }
    impl Default for EthernetPropertiesSupportedEthernetCapabilities {
        fn default() -> Self {
            Self::V012000(Default::default())
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
            Option<crate::port::v1_20_0::FibreChannelPropertiesPortConnectionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum FibreChannelPropertiesPortConnectionType {
        V012000(crate::port::v1_20_0::PortConnectionType),
        V000001(crate::port::v1_20_0::FibreChannelPropertiesPortConnectionTypeN1),
    }
    impl Default for FibreChannelPropertiesPortConnectionType {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    pub enum HostDeviceType {
        #[default]
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "System")]
        System,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv4"
        )]
        pub additional_management_addresses_ipv4: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv6"
        )]
        pub additional_management_addresses_ipv6: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_20_0::LLDPReceiveChassisIdSubtype>,
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
        pub port_id_subtype: Option<crate::port::v1_20_0::LLDPReceivePortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_20_0::LLDPReceiveSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPReceiveChassisIdSubtype {
        V012000(crate::port::v1_20_0::IEEE802IdSubtype),
        V000001(crate::port::v1_20_0::LLDPReceiveChassisIdSubtypeN1),
    }
    impl Default for LLDPReceiveChassisIdSubtype {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::IEEE802IdSubtype),
        V000001(crate::port::v1_20_0::LLDPReceivePortIdSubtypeN1),
    }
    impl Default for LLDPReceivePortIdSubtype {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_20_0::LLDPReceiveSystemCapabilitiesN1),
    }
    impl Default for LLDPReceiveSystemCapabilities {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv4"
        )]
        pub additional_management_addresses_ipv4: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalManagementAddressesIPv6"
        )]
        pub additional_management_addresses_ipv6: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisId")]
        pub chassis_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChassisIdSubtype")]
        pub chassis_id_subtype: Option<crate::port::v1_20_0::LLDPTransmitChassisIdSubtype>,
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
        pub port_id_subtype: Option<crate::port::v1_20_0::LLDPTransmitPortIdSubtype>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemCapabilities")]
        pub system_capabilities: Option<Vec<crate::port::v1_20_0::LLDPTransmitSystemCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemDescription")]
        pub system_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SystemName")]
        pub system_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LLDPTransmitChassisIdSubtype {
        V012000(crate::port::v1_20_0::IEEE802IdSubtype),
        V000001(crate::port::v1_20_0::LLDPTransmitChassisIdSubtypeN1),
    }
    impl Default for LLDPTransmitChassisIdSubtype {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::IEEE802IdSubtype),
        V000001(crate::port::v1_20_0::LLDPTransmitPortIdSubtypeN1),
    }
    impl Default for LLDPTransmitPortIdSubtype {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::LLDPSystemCapabilities),
        V000001(crate::port::v1_20_0::LLDPTransmitSystemCapabilitiesN1),
    }
    impl Default for LLDPTransmitSystemCapabilities {
        fn default() -> Self {
            Self::V012000(Default::default())
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
            Option<Vec<crate::port::v1_20_0::LinkConfigurationConfiguredNetworkLinks>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology:
            Option<crate::port::v1_20_0::LinkConfigurationLinkNetworkTechnology>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkConfigurationConfiguredNetworkLinks {
        V012000(crate::port::v1_20_0::ConfiguredNetworkLink),
        V000001(crate::port::v1_20_0::LinkConfigurationConfiguredNetworkLinksN1),
    }
    impl Default for LinkConfigurationConfiguredNetworkLinks {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkConfigurationConfiguredNetworkLinksN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkConfigurationLinkNetworkTechnology {
        V012000(crate::port::v1_20_0::LinkNetworkTechnology),
        V000001(crate::port::v1_20_0::LinkConfigurationLinkNetworkTechnologyN1),
    }
    impl Default for LinkConfigurationLinkNetworkTechnology {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkConfigurationLinkNetworkTechnologyN1 {
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
    pub struct LinkPartnerExpected {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::port::v1_20_0::LinkPartnerExpectedDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkPartnerId")]
        pub link_partner_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkPartnerExpectedDeviceType {
        V012000(crate::port::v1_20_0::PartnerDeviceType),
        V000001(crate::port::v1_20_0::LinkPartnerExpectedDeviceTypeN1),
    }
    impl Default for LinkPartnerExpectedDeviceType {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkPartnerExpectedDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LinkPartnerReceived {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::port::v1_20_0::LinkPartnerReceivedDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkPartnerId")]
        pub link_partner_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PBRId")]
        pub pbr_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryFMId")]
        pub primary_fm_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryFMId")]
        pub secondary_fm_id: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkPartnerReceivedDeviceType {
        V012000(crate::port::v1_20_0::PartnerDeviceType),
        V000001(crate::port::v1_20_0::LinkPartnerReceivedDeviceTypeN1),
    }
    impl Default for LinkPartnerReceivedDeviceType {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkPartnerReceivedDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LinkPartnerTransmit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceType")]
        pub device_type: Option<crate::port::v1_20_0::LinkPartnerTransmitDeviceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkPartnerId")]
        pub link_partner_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PBRId")]
        pub pbr_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrimaryFMId")]
        pub primary_fm_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecondaryFMId")]
        pub secondary_fm_id: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkPartnerTransmitDeviceType {
        V012000(crate::port::v1_20_0::PartnerDeviceType),
        V000001(crate::port::v1_20_0::LinkPartnerTransmitDeviceTypeN1),
    }
    impl Default for LinkPartnerTransmitDeviceType {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkPartnerTransmitDeviceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LinkPartnerValidation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceCheckEnabled")]
        pub device_check_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortCheckEnabled")]
        pub port_check_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidationState")]
        pub validation_state: Option<crate::port::v1_20_0::LinkPartnerValidationValidationState>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinkPartnerValidationValidationState {
        V012000(crate::port::v1_20_0::PartnerValidationState),
        V000001(crate::port::v1_20_0::LinkPartnerValidationValidationStateN1),
    }
    impl Default for LinkPartnerValidationValidationState {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkPartnerValidationValidationStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPorts"
        )]
        pub associated_physical_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPorts@odata.count"
        )]
        pub associated_physical_ports_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevices")]
        pub cxl_logical_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CXLLogicalDevices@odata.count"
        )]
        pub cxl_logical_devices_odata_count: Option<i64>,
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
    pub struct PCIeProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReferenceClockMode")]
        pub reference_clock_mode: Option<crate::port::v1_20_0::PCIePropertiesReferenceClockMode>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIePropertiesReferenceClockMode {
        V012000(crate::port::v1_20_0::PCIeReferenceClockMode),
        V000001(crate::port::v1_20_0::PCIePropertiesReferenceClockModeN1),
    }
    impl Default for PCIePropertiesReferenceClockMode {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIePropertiesReferenceClockModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeReferenceClockMode {
        #[default]
        #[serde(rename = "CommonClock")]
        CommonClock,
        #[serde(rename = "SeparateClock")]
        SeparateClock,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PartnerDeviceType {
        #[default]
        #[serde(rename = "Endpoint")]
        Endpoint,
        #[serde(rename = "Switch")]
        Switch,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PartnerValidationState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Discovering")]
        Discovering,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "LinkDown")]
        LinkDown,
        #[serde(rename = "Validated")]
        Validated,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PoE {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapablePowerWatts")]
        pub capable_power_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredPowerWatts"
        )]
        pub configured_power_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectionStatus")]
        pub detection_status: Option<crate::port::v1_20_0::PoEDetectionStatusAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerClass")]
        pub power_class: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMode")]
        pub power_mode: Option<crate::port::v1_20_0::PoEPowerModeAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Standard")]
        pub standard: Option<crate::port::v1_20_0::PoEStandardAnony>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEDetectionStatus {
        #[default]
        #[serde(rename = "DeliveringPower")]
        DeliveringPower,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Fault")]
        Fault,
        #[serde(rename = "OtherFault")]
        OtherFault,
        #[serde(rename = "Searching")]
        Searching,
        #[serde(rename = "Test")]
        Test,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEDetectionStatusAnony {
        V012000(crate::port::v1_20_0::PoEDetectionStatus),
        V000001(crate::port::v1_20_0::PoEDetectionStatusN1),
    }
    impl Default for PoEDetectionStatusAnony {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEDetectionStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEPowerMode {
        #[default]
        #[serde(rename = "None")]
        None,
        #[serde(rename = "PD")]
        PD,
        #[serde(rename = "PSE")]
        PSE,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEPowerModeAnony {
        V012000(crate::port::v1_20_0::PoEPowerMode),
        V000001(crate::port::v1_20_0::PoEPowerModeN1),
    }
    impl Default for PoEPowerModeAnony {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEPowerModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEStandard {
        #[default]
        #[serde(rename = "IEEE8023af")]
        IEEE8023af,
        #[serde(rename = "IEEE8023at")]
        IEEE8023at,
        #[serde(rename = "IEEE8023bt")]
        IEEE8023bt,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PoEStandardAnony {
        V012000(crate::port::v1_20_0::PoEStandard),
        V000001(crate::port::v1_20_0::PoEStandardN1),
    }
    impl Default for PoEStandardAnony {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoEStandardN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Port {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port::v1_20_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveWidth")]
        pub active_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedPhysicalPort"
        )]
        pub associated_physical_port: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableProtocolVersions"
        )]
        pub capable_protocol_versions: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfiguredSpeedGbps"
        )]
        pub configured_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConfiguredWidth")]
        pub configured_width: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentProtocolVersion"
        )]
        pub current_protocol_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentSpeedGbps")]
        pub current_speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::port::v1_20_0::PortCXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::port::v1_20_0::PortDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::port::v1_20_0::PortEthernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port::v1_20_0::PortFibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirstLane")]
        pub first_lane: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMaxBandwidth"
        )]
        pub function_max_bandwidth: Option<Vec<crate::port::v1_20_0::FunctionMaxBandwidth>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FunctionMinBandwidth"
        )]
        pub function_min_bandwidth: Option<Vec<crate::port::v1_20_0::FunctionMinBandwidth>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port::v1_20_0::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostDevice")]
        pub host_device: Option<crate::port::v1_20_0::PortHostDevice>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfiniBand")]
        pub infini_band: Option<crate::port::v1_20_0::PortInfiniBand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsAggregation")]
        pub is_aggregation: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSplit")]
        pub is_split: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkConfiguration")]
        pub link_configuration: Option<Vec<crate::port::v1_20_0::LinkConfiguration>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::port::v1_20_0::PortLinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkPartnerExpected"
        )]
        pub link_partner_expected: Option<crate::port::v1_20_0::PortLinkPartnerExpected>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkPartnerReceived"
        )]
        pub link_partner_received: Option<crate::port::v1_20_0::PortLinkPartnerReceived>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkPartnerTransmit"
        )]
        pub link_partner_transmit: Option<crate::port::v1_20_0::PortLinkPartnerTransmit>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkPartnerValidation"
        )]
        pub link_partner_validation: Option<crate::port::v1_20_0::PortLinkPartnerValidation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkState")]
        pub link_state: Option<crate::port::v1_20_0::LinkState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::port::v1_20_0::LinkStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkTransitionIndicator"
        )]
        pub link_transition_indicator: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::port::v1_20_0::Links>,
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
        pub metrics: Option<crate::port::v1_20_0::PortMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIe")]
        pub pcie: Option<crate::port::v1_20_0::PortPCIe>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortId")]
        pub port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMedium")]
        pub port_medium: Option<crate::port::v1_20_0::PortPortMedium>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortProtocol")]
        pub port_protocol: Option<crate::port::v1_20_0::PortPortProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortType")]
        pub port_type: Option<crate::port::v1_20_0::PortPortType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemotePortId")]
        pub remote_port_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SFP")]
        pub sfp: Option<crate::port::v1_20_0::PortSFP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UALink")]
        pub ua_link: Option<crate::port::v1_20_0::UALink>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
        pub width: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortCXL {
        V012000(crate::port::v1_20_0::CXL),
        V000001(crate::port::v1_20_0::PortCXLN1),
    }
    impl Default for PortCXL {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V000001(crate::port::v1_20_0::PortDescriptionN1),
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
        V012000(crate::port::v1_20_0::EthernetProperties),
        V000001(crate::port::v1_20_0::PortEthernetN1),
    }
    impl Default for PortEthernet {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::FibreChannelProperties),
        V000001(crate::port::v1_20_0::PortFibreChannelN1),
    }
    impl Default for PortFibreChannel {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    pub enum PortHostDevice {
        V012000(crate::port::v1_20_0::HostDeviceType),
        V000001(crate::port::v1_20_0::PortHostDeviceN1),
    }
    impl Default for PortHostDevice {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortHostDeviceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortInfiniBand {
        V012000(crate::port::v1_20_0::InfiniBandProperties),
        V000001(crate::port::v1_20_0::PortInfiniBandN1),
    }
    impl Default for PortInfiniBand {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::LinkNetworkTechnology),
        V000001(crate::port::v1_20_0::PortLinkNetworkTechnologyN1),
    }
    impl Default for PortLinkNetworkTechnology {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkNetworkTechnologyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortLinkPartnerExpected {
        V012000(crate::port::v1_20_0::LinkPartnerExpected),
        V000001(crate::port::v1_20_0::PortLinkPartnerExpectedN1),
    }
    impl Default for PortLinkPartnerExpected {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkPartnerExpectedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortLinkPartnerReceived {
        V012000(crate::port::v1_20_0::LinkPartnerReceived),
        V000001(crate::port::v1_20_0::PortLinkPartnerReceivedN1),
    }
    impl Default for PortLinkPartnerReceived {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkPartnerReceivedN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortLinkPartnerTransmit {
        V012000(crate::port::v1_20_0::LinkPartnerTransmit),
        V000001(crate::port::v1_20_0::PortLinkPartnerTransmitN1),
    }
    impl Default for PortLinkPartnerTransmit {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkPartnerTransmitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortLinkPartnerValidation {
        V012000(crate::port::v1_20_0::LinkPartnerValidation),
        V000001(crate::port::v1_20_0::PortLinkPartnerValidationN1),
    }
    impl Default for PortLinkPartnerValidation {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortLinkPartnerValidationN1 {
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
        V000001(crate::port::v1_20_0::PortMetricsN1),
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
    pub enum PortPCIe {
        V012000(crate::port::v1_20_0::PCIeProperties),
        V000001(crate::port::v1_20_0::PortPCIeN1),
    }
    impl Default for PortPCIe {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortPCIeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PortPortMedium {
        V012000(crate::port::v1_20_0::PortMedium),
        V000001(crate::port::v1_20_0::PortPortMediumN1),
    }
    impl Default for PortPortMedium {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V000001(crate::port::v1_20_0::PortPortProtocolN1),
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
        V012000(crate::port::v1_20_0::PortType),
        V000001(crate::port::v1_20_0::PortPortTypeN1),
    }
    impl Default for PortPortType {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::SFP),
        V000001(crate::port::v1_20_0::PortSFPN1),
    }
    impl Default for PortSFP {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    pub struct ResetTransceivers {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetTransceiversRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SFP {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateCode")]
        pub date_code: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FiberConnectionType"
        )]
        pub fiber_connection_type: Option<crate::port::v1_20_0::SFPFiberConnectionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagementInterfaceType"
        )]
        pub management_interface_type: Option<crate::port::v1_20_0::SFPManagementInterfaceType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediumType")]
        pub medium_type: Option<crate::port::v1_20_0::SFPMediumType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedSFPTypes")]
        pub supported_sfp_types: Option<Vec<crate::port::v1_20_0::SFPSupportedSFPTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
        pub r#type: Option<crate::port::v1_20_0::SFPTypeAnony>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorOUI")]
        pub vendor_oui: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPFiberConnectionType {
        V012000(crate::port::v1_20_0::FiberConnectionType),
        V000001(crate::port::v1_20_0::SFPFiberConnectionTypeN1),
    }
    impl Default for SFPFiberConnectionType {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    pub enum SFPManagementInterfaceType {
        V012000(crate::port::v1_20_0::TransceiverManagementInterfaceType),
        V000001(crate::port::v1_20_0::SFPManagementInterfaceTypeN1),
    }
    impl Default for SFPManagementInterfaceType {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SFPManagementInterfaceTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SFPMediumType {
        V012000(crate::port::v1_20_0::MediumType),
        V000001(crate::port::v1_20_0::SFPMediumTypeN1),
    }
    impl Default for SFPMediumType {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        V012000(crate::port::v1_20_0::SFPType),
        V000001(crate::port::v1_20_0::SFPSupportedSFPTypesN1),
    }
    impl Default for SFPSupportedSFPTypes {
        fn default() -> Self {
            Self::V012000(Default::default())
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
        #[serde(rename = "CDFP")]
        CDFP,
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
        V012000(crate::port::v1_20_0::SFPType),
        V000001(crate::port::v1_20_0::SFPTypeN1),
    }
    impl Default for SFPTypeAnony {
        fn default() -> Self {
            Self::V012000(Default::default())
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransceiverManagementInterfaceType {
        #[default]
        #[serde(rename = "CMIS")]
        CMIS,
        #[serde(rename = "SFP")]
        SFP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UALink {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationModeEnabled"
        )]
        pub authentication_mode_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DLDownDelayMilliseconds"
        )]
        pub dl_down_delay_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DLState")]
        pub dl_state: Option<crate::port::v1_20_0::UALinkDLState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Generation")]
        pub generation: Option<crate::port::v1_20_0::UALinkGeneration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkFoldingEnabled")]
        pub link_folding_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkFoldingSupported"
        )]
        pub link_folding_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkResiliencyEnabled"
        )]
        pub link_resiliency_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkResiliencySupported"
        )]
        pub link_resiliency_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonStrictOrderingModeEnabled"
        )]
        pub non_strict_ordering_mode_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PLState")]
        pub pl_state: Option<crate::port::v1_20_0::UALinkPLState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PLDLAdminEnabled")]
        pub pldl_admin_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TLState")]
        pub tl_state: Option<crate::port::v1_20_0::UALinkTLState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TLUPLIAdminEnabled")]
        pub tlupli_admin_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXAckTimeoutNanoseconds"
        )]
        pub tx_ack_timeout_nanoseconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UALink128GExtendedSpeedMode"
        )]
        pub ua_link128_g_extended_speed_mode:
            Option<crate::port::v1_20_0::UALink128GExtendedSpeedMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UALink200GCodewordInterleave"
        )]
        pub ua_link200_g_codeword_interleave:
            Option<crate::port::v1_20_0::UALink200GCodewordInterleave>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UALink200GSerialRate"
        )]
        pub ua_link200_g_serial_rate: Option<crate::port::v1_20_0::UALink200GSerialRate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UARTAdminEnabled")]
        pub uart_admin_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UPLICompleterState")]
        pub upli_completer_state: Option<crate::port::v1_20_0::UALinkUPLICompleterState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UPLIOriginatorState"
        )]
        pub upli_originator_state: Option<crate::port::v1_20_0::UALinkUPLIOriginatorState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UPLIWatchdogTimerMicroseconds"
        )]
        pub upli_watchdog_timer_microseconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALink128GExtendedSpeedMode {
        #[default]
        #[serde(rename = "104G")]
        N104G,
        #[serde(rename = "112G")]
        N112G,
        #[serde(rename = "120G")]
        N120G,
        #[serde(rename = "128G")]
        N128G,
        #[serde(rename = "NoESM")]
        NoESM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALink200GCodewordInterleave {
        #[default]
        #[serde(rename = "FourWay")]
        FourWay,
        #[serde(rename = "OneWay")]
        OneWay,
        #[serde(rename = "TwoWay")]
        TwoWay,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALink200GSerialRate {
        #[default]
        #[serde(rename = "100G")]
        N100G,
        #[serde(rename = "200G")]
        N200G,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkDLState {
        #[default]
        #[serde(rename = "Fault")]
        Fault,
        #[serde(rename = "Idle")]
        Idle,
        #[serde(rename = "NOP")]
        NOP,
        #[serde(rename = "PowerDown")]
        PowerDown,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkGeneration {
        #[default]
        #[serde(rename = "UALink128G")]
        UALink128G,
        #[serde(rename = "UALink200G")]
        UALink200G,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkPLState {
        #[default]
        #[serde(rename = "Idle")]
        Idle,
        #[serde(rename = "TrainingFailed")]
        TrainingFailed,
        #[serde(rename = "TrainingInProgress")]
        TrainingInProgress,
        #[serde(rename = "TrainingTimeout")]
        TrainingTimeout,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkTLState {
        #[default]
        #[serde(rename = "DropMode")]
        DropMode,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkUPLICompleterState {
        #[default]
        #[serde(rename = "DropMode")]
        DropMode,
        #[serde(rename = "Enabled")]
        Enabled,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UALinkUPLIOriginatorState {
        #[default]
        #[serde(rename = "DropMode")]
        DropMode,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "IsolationMode")]
        IsolationMode,
    }
}
