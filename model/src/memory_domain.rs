pub type MemoryDomain = crate::memory_domain::v1_5_0::MemoryDomain;
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_domain::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLLogicalDevices")]
        pub cxl_logical_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CXLLogicalDevices@odata.count"
        )]
        pub cxl_logical_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricAdapters@odata.count"
        )]
        pub fabric_adapters_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaControllers")]
        pub media_controllers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MediaControllers@odata.count"
        )]
        pub media_controllers_odata_count: Option<i64>,
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
    pub struct MemoryDomain {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_domain::v1_5_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowsBlockProvisioning"
        )]
        pub allows_block_provisioning: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowsMemoryChunkCreation"
        )]
        pub allows_memory_chunk_creation: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowsMirroring")]
        pub allows_mirroring: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowsSparing")]
        pub allows_sparing: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InterleavableMemorySets"
        )]
        pub interleavable_memory_sets: Option<Vec<crate::memory_domain::v1_5_0::MemorySet>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory_domain::v1_5_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryChunkIncrementMiB"
        )]
        pub memory_chunk_increment_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunks")]
        pub memory_chunks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySizeMiB")]
        pub memory_size_mib: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinMemoryChunkSizeMiB"
        )]
        pub min_memory_chunk_size_mib: Option<i64>,
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
    pub struct MemorySet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemorySet")]
        pub memory_set: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemorySet@odata.count"
        )]
        pub memory_set_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
