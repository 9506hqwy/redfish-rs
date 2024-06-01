pub type MemoryRegion = crate::memory_region::v1_0_0::MemoryRegion;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_region::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryChunk {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChunkLink")]
        pub chunk_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChunkOffsetMiB")]
        pub chunk_offset_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryExtent {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExtentOffsetMiB")]
        pub extent_offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExtentSizeMiB")]
        pub extent_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SequenceNumber")]
        pub sequence_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
        pub tag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryRegion {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_region::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeMiB")]
        pub block_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExtentsCount")]
        pub extents_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HardwareManagedCoherencyRegion"
        )]
        pub hardware_managed_coherency_region: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunks")]
        pub memory_chunks: Option<Vec<crate::memory_region::v1_0_0::MemoryChunk>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryExtents")]
        pub memory_extents: Option<Vec<crate::memory_region::v1_0_0::MemoryExtent>>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonVolatileRegion")]
        pub non_volatile_region: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionBaseOffetMiB")]
        pub region_base_offet_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionNumber")]
        pub region_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionSizeMiB")]
        pub region_size_mib: Option<i64>,
        #[serde(rename = "RegionType")]
        pub region_type: crate::memory_region::v1_0_0::RegionType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SanitizeOnRelease")]
        pub sanitize_on_release: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ShareableRegion")]
        pub shareable_region: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RegionType {
        #[default]
        #[serde(rename = "Dynamic")]
        Dynamic,
        #[serde(rename = "Static")]
        Static,
    }
}
