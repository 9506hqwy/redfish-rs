pub type NetworkAdapterMetrics = crate::network_adapter_metrics::v1_0_1::NetworkAdapterMetrics;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_adapter_metrics::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkAdapterMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_adapter_metrics::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPUCorePercent")]
        pub cpu_core_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostBusRXPercent")]
        pub host_bus_rx_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostBusTXPercent")]
        pub host_bus_tx_percent: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSIRXBytes")]
        pub ncsirx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSIRXFrames")]
        pub ncsirx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSITXBytes")]
        pub ncsitx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSITXFrames")]
        pub ncsitx_frames: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::network_adapter_metrics::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NetworkAdapterMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::network_adapter_metrics::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPUCorePercent")]
        pub cpu_core_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostBusRXPercent")]
        pub host_bus_rx_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostBusTXPercent")]
        pub host_bus_tx_percent: Option<f64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSIRXBytes")]
        pub ncsirx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSIRXFrames")]
        pub ncsirx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSITXBytes")]
        pub ncsitx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NCSITXFrames")]
        pub ncsitx_frames: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
