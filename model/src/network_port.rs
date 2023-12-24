use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NetworkPort {
    V010401(crate::network_port::v1_4_1::NetworkPort),
    V010302(crate::network_port::v1_3_2::NetworkPort),
    V010207(crate::network_port::v1_2_7::NetworkPort),
    V010107(crate::network_port::v1_1_7::NetworkPort),
    V010008(crate::network_port::v1_0_8::NetworkPort),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_8 {
    use serde::{Deserialize, Serialize};
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
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "Down")]
        Down,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMaxBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBWAllocPercent")]
        pub max_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMinBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinBWAllocPercent")]
        pub min_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkPort {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveLinkTechnology"
        )]
        pub active_link_technology: Option<crate::network_port::v1_0_8::LinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNetworkAddresses"
        )]
        pub associated_network_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::network_port::v1_0_8::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::network_port::v1_0_8::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::network_port::v1_0_8::LinkStatus>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMaxBWAlloc"
        )]
        pub net_dev_func_max_bw_alloc:
            Option<Vec<crate::network_port::v1_0_8::NetDevFuncMaxBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMinBWAlloc"
        )]
        pub net_dev_func_min_bw_alloc:
            Option<Vec<crate::network_port::v1_0_8::NetDevFuncMinBWAlloc>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalPortNumber")]
        pub physical_port_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMaximumMTU")]
        pub port_maximum_mtu: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::network_port::v1_0_8::SupportedEthernetCapabilities>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinkCapabilities"
        )]
        pub supported_link_capabilities:
            Option<Vec<crate::network_port::v1_0_8::SupportedLinkCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
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
    pub struct SupportedLinkCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::network_port::v1_0_8::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedMbps")]
        pub link_speed_mbps: Option<i64>,
    }
}
pub mod v1_1_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_port::v1_1_7::OemActions>,
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
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "Down")]
        Down,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMaxBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBWAllocPercent")]
        pub max_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMinBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinBWAllocPercent")]
        pub min_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkPort {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_port::v1_1_7::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveLinkTechnology"
        )]
        pub active_link_technology: Option<crate::network_port::v1_1_7::LinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNetworkAddresses"
        )]
        pub associated_network_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::network_port::v1_1_7::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::network_port::v1_1_7::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::network_port::v1_1_7::LinkStatus>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMaxBWAlloc"
        )]
        pub net_dev_func_max_bw_alloc:
            Option<Vec<crate::network_port::v1_1_7::NetDevFuncMaxBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMinBWAlloc"
        )]
        pub net_dev_func_min_bw_alloc:
            Option<Vec<crate::network_port::v1_1_7::NetDevFuncMinBWAlloc>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalPortNumber")]
        pub physical_port_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMaximumMTU")]
        pub port_maximum_mtu: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::network_port::v1_1_7::SupportedEthernetCapabilities>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinkCapabilities"
        )]
        pub supported_link_capabilities:
            Option<Vec<crate::network_port::v1_1_7::SupportedLinkCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SupportedEthernetCapabilities {
        #[default]
        #[serde(rename = "EEE")]
        EEE,
        #[serde(rename = "WakeOnLAN")]
        WakeOnLAN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SupportedLinkCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::network_port::v1_1_7::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedMbps")]
        pub link_speed_mbps: Option<i64>,
    }
}
pub mod v1_2_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_port::v1_2_7::OemActions>,
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
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "Down")]
        Down,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMaxBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBWAllocPercent")]
        pub max_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMinBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinBWAllocPercent")]
        pub min_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkPort {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_port::v1_2_7::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveLinkTechnology"
        )]
        pub active_link_technology: Option<crate::network_port::v1_2_7::LinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNetworkAddresses"
        )]
        pub associated_network_addresses: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentLinkSpeedMbps"
        )]
        pub current_link_speed_mbps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCFabricName")]
        pub fc_fabric_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FCPortConnectionType"
        )]
        pub fc_port_connection_type: Option<crate::network_port::v1_2_7::PortConnectionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::network_port::v1_2_7::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::network_port::v1_2_7::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::network_port::v1_2_7::LinkStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxFrameSize")]
        pub max_frame_size: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMaxBWAlloc"
        )]
        pub net_dev_func_max_bw_alloc:
            Option<Vec<crate::network_port::v1_2_7::NetDevFuncMaxBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMinBWAlloc"
        )]
        pub net_dev_func_min_bw_alloc:
            Option<Vec<crate::network_port::v1_2_7::NetDevFuncMinBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberDiscoveredRemotePorts"
        )]
        pub number_discovered_remote_ports: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalPortNumber")]
        pub physical_port_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMaximumMTU")]
        pub port_maximum_mtu: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::network_port::v1_2_7::SupportedEthernetCapabilities>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinkCapabilities"
        )]
        pub supported_link_capabilities:
            Option<Vec<crate::network_port::v1_2_7::SupportedLinkCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortConnectionType {
        #[default]
        #[serde(rename = "ExtenderFabric")]
        ExtenderFabric,
        #[serde(rename = "Generic")]
        Generic,
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
    pub struct SupportedLinkCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiation"
        )]
        pub auto_speed_negotiation: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableLinkSpeedMbps"
        )]
        pub capable_link_speed_mbps: Option<Vec<i64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::network_port::v1_2_7::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedMbps")]
        pub link_speed_mbps: Option<i64>,
    }
}
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_port::v1_3_2::OemActions>,
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
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "Down")]
        Down,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Training")]
        Training,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMaxBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBWAllocPercent")]
        pub max_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMinBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinBWAllocPercent")]
        pub min_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkPort {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_port::v1_3_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveLinkTechnology"
        )]
        pub active_link_technology: Option<crate::network_port::v1_3_2::LinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNetworkAddresses"
        )]
        pub associated_network_addresses: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentLinkSpeedMbps"
        )]
        pub current_link_speed_mbps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCFabricName")]
        pub fc_fabric_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FCPortConnectionType"
        )]
        pub fc_port_connection_type: Option<crate::network_port::v1_3_2::PortConnectionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::network_port::v1_3_2::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::network_port::v1_3_2::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::network_port::v1_3_2::LinkStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxFrameSize")]
        pub max_frame_size: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMaxBWAlloc"
        )]
        pub net_dev_func_max_bw_alloc:
            Option<Vec<crate::network_port::v1_3_2::NetDevFuncMaxBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMinBWAlloc"
        )]
        pub net_dev_func_min_bw_alloc:
            Option<Vec<crate::network_port::v1_3_2::NetDevFuncMinBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberDiscoveredRemotePorts"
        )]
        pub number_discovered_remote_ports: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalPortNumber")]
        pub physical_port_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMaximumMTU")]
        pub port_maximum_mtu: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::network_port::v1_3_2::SupportedEthernetCapabilities>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinkCapabilities"
        )]
        pub supported_link_capabilities:
            Option<Vec<crate::network_port::v1_3_2::SupportedLinkCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortConnectionType {
        #[default]
        #[serde(rename = "ExtenderFabric")]
        ExtenderFabric,
        #[serde(rename = "Generic")]
        Generic,
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
    pub struct SupportedLinkCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiation"
        )]
        pub auto_speed_negotiation: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableLinkSpeedMbps"
        )]
        pub capable_link_speed_mbps: Option<Vec<i64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::network_port::v1_3_2::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedMbps")]
        pub link_speed_mbps: Option<i64>,
    }
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_port::v1_4_1::OemActions>,
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
    pub enum LinkNetworkTechnology {
        #[default]
        #[serde(rename = "Ethernet")]
        Ethernet,
        #[serde(rename = "FibreChannel")]
        FibreChannel,
        #[serde(rename = "InfiniBand")]
        InfiniBand,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinkStatus {
        #[default]
        #[serde(rename = "Down")]
        Down,
        #[serde(rename = "Starting")]
        Starting,
        #[serde(rename = "Training")]
        Training,
        #[serde(rename = "Up")]
        Up,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMaxBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxBWAllocPercent")]
        pub max_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetDevFuncMinBWAlloc {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinBWAllocPercent")]
        pub min_bw_alloc_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkPort {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_port::v1_4_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ActiveLinkTechnology"
        )]
        pub active_link_technology: Option<crate::network_port::v1_4_1::LinkNetworkTechnology>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedNetworkAddresses"
        )]
        pub associated_network_addresses: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CurrentLinkSpeedMbps"
        )]
        pub current_link_speed_mbps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EEEEnabled")]
        pub eee_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FCFabricName")]
        pub fc_fabric_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FCPortConnectionType"
        )]
        pub fc_port_connection_type: Option<crate::network_port::v1_4_1::PortConnectionType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlConfiguration"
        )]
        pub flow_control_configuration: Option<crate::network_port::v1_4_1::FlowControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControlStatus")]
        pub flow_control_status: Option<crate::network_port::v1_4_1::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkStatus")]
        pub link_status: Option<crate::network_port::v1_4_1::LinkStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxFrameSize")]
        pub max_frame_size: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMaxBWAlloc"
        )]
        pub net_dev_func_max_bw_alloc:
            Option<Vec<crate::network_port::v1_4_1::NetDevFuncMaxBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetDevFuncMinBWAlloc"
        )]
        pub net_dev_func_min_bw_alloc:
            Option<Vec<crate::network_port::v1_4_1::NetDevFuncMinBWAlloc>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberDiscoveredRemotePorts"
        )]
        pub number_discovered_remote_ports: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalPortNumber")]
        pub physical_port_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortMaximumMTU")]
        pub port_maximum_mtu: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalDetected")]
        pub signal_detected: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedEthernetCapabilities"
        )]
        pub supported_ethernet_capabilities:
            Option<Vec<crate::network_port::v1_4_1::SupportedEthernetCapabilities>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinkCapabilities"
        )]
        pub supported_link_capabilities:
            Option<Vec<crate::network_port::v1_4_1::SupportedLinkCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WakeOnLANEnabled")]
        pub wake_on_lan_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PortConnectionType {
        #[default]
        #[serde(rename = "ExtenderFabric")]
        ExtenderFabric,
        #[serde(rename = "Generic")]
        Generic,
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
    pub struct SupportedLinkCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoSpeedNegotiation"
        )]
        pub auto_speed_negotiation: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapableLinkSpeedMbps"
        )]
        pub capable_link_speed_mbps: Option<Vec<i64>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LinkNetworkTechnology"
        )]
        pub link_network_technology: Option<crate::network_port::v1_4_1::LinkNetworkTechnology>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkSpeedMbps")]
        pub link_speed_mbps: Option<i64>,
    }
}
