pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_device_function_metrics::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Ethernet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumOffloadedIPv4Conns"
        )]
        pub num_offloaded_ipv4_conns: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumOffloadedIPv6Conns"
        )]
        pub num_offloaded_ipv6_conns: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortLoginAccepts")]
        pub port_login_accepts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortLoginRejects")]
        pub port_login_rejects: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PortLoginRequests")]
        pub port_login_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXCongestionFPINs")]
        pub rx_congestion_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXDeliveryFPINs")]
        pub rx_delivery_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXExchanges")]
        pub rx_exchanges: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXLinkIntegrityFPINs"
        )]
        pub rx_link_integrity_fpi_ns: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXPeerCongestionFPINs"
        )]
        pub rx_peer_congestion_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXSequences")]
        pub rx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXCongestionFPINs")]
        pub tx_congestion_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXDeliveryFPINs")]
        pub tx_delivery_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXExchanges")]
        pub tx_exchanges: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXLinkIntegrityFPINs"
        )]
        pub tx_link_integrity_fpi_ns: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXPeerCongestionFPINs"
        )]
        pub tx_peer_congestion_fpi_ns: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSequences")]
        pub tx_sequences: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkDeviceFunctionMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_device_function_metrics::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ethernet")]
        pub ethernet: Option<crate::network_device_function_metrics::v1_1_0::Ethernet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::network_device_function_metrics::v1_1_0::FibreChannel>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXAvgQueueDepthPercent"
        )]
        pub rx_avg_queue_depth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFrames")]
        pub rx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXQueuesEmpty")]
        pub rx_queues_empty: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXQueuesFull")]
        pub rx_queues_full: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXAvgQueueDepthPercent"
        )]
        pub tx_avg_queue_depth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXFrames")]
        pub tx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXQueuesEmpty")]
        pub tx_queues_empty: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXQueuesFull")]
        pub tx_queues_full: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
