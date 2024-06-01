pub type CXLLogicalDevice = crate::cxl_logical_device::v1_1_0::CXLLogicalDevice;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::cxl_logical_device::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLLogicalDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::cxl_logical_device::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::cxl_logical_device::v1_0_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySizeMiB")]
        pub memory_size_mib: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "QoS")]
        pub qos: Option<crate::cxl_logical_device::v1_0_0::QoS>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities:
            Option<crate::cxl_logical_device::v1_0_0::QoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SemanticsSupported")]
        pub semantics_supported: Option<Vec<crate::cxl_logical_device::v1_0_0::CXLSemantic>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLSemantic {
        #[default]
        #[serde(rename = "CXLcache")]
        CXLcache,
        #[serde(rename = "CXLio")]
        CXLio,
        #[serde(rename = "CXLmem")]
        CXLmem,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunks")]
        pub memory_chunks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryChunks@odata.count"
        )]
        pub memory_chunks_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct QoS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedBandwidth")]
        pub allocated_bandwidth: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitPercent")]
        pub limit_percent: Option<i64>,
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
}
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::cxl_logical_device::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXLLogicalDevice {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::cxl_logical_device::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::cxl_logical_device::v1_1_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Log")]
        pub log: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegions")]
        pub memory_regions: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySizeMiB")]
        pub memory_size_mib: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "QoS")]
        pub qos: Option<crate::cxl_logical_device::v1_1_0::QoS>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "QoSTelemetryCapabilities"
        )]
        pub qos_telemetry_capabilities:
            Option<crate::cxl_logical_device::v1_1_0::QoSTelemetryCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SemanticsSupported")]
        pub semantics_supported: Option<Vec<crate::cxl_logical_device::v1_1_0::CXLSemantic>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLSemantic {
        #[default]
        #[serde(rename = "CXLcache")]
        CXLcache,
        #[serde(rename = "CXLio")]
        CXLio,
        #[serde(rename = "CXLmem")]
        CXLmem,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunks")]
        pub memory_chunks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryChunks@odata.count"
        )]
        pub memory_chunks_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct QoS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedBandwidth")]
        pub allocated_bandwidth: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitPercent")]
        pub limit_percent: Option<i64>,
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
}
