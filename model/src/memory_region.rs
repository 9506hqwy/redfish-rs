pub type MemoryRegion = crate::memory_region::v1_0_3::MemoryRegion;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_region::v1_0_1::OemActions>,
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
        pub actions: Option<crate::memory_region::v1_0_1::Actions>,
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
        pub memory_chunks: Option<Vec<crate::memory_region::v1_0_1::MemoryChunk>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryExtents")]
        pub memory_extents: Option<Vec<crate::memory_region::v1_0_1::MemoryExtent>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RegionBaseOffsetMiB"
        )]
        pub region_base_offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionNumber")]
        pub region_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionSizeMiB")]
        pub region_size_mib: Option<i64>,
        #[serde(rename = "RegionType")]
        pub region_type: crate::memory_region::v1_0_1::RegionType,
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
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_region::v1_0_3::OemActions>,
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
        pub actions: Option<crate::memory_region::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeMiB")]
        pub block_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::memory_region::v1_0_3::MemoryRegionDescription>,
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
        pub memory_chunks: Option<Vec<crate::memory_region::v1_0_3::MemoryRegionMemoryChunks>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryExtents")]
        pub memory_extents: Option<Vec<crate::memory_region::v1_0_3::MemoryRegionMemoryExtents>>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RegionBaseOffsetMiB"
        )]
        pub region_base_offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionNumber")]
        pub region_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionSizeMiB")]
        pub region_size_mib: Option<i64>,
        #[serde(rename = "RegionType")]
        pub region_type: crate::memory_region::v1_0_3::RegionType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SanitizeOnRelease")]
        pub sanitize_on_release: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ShareableRegion")]
        pub shareable_region: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionDescription {
        V000001(crate::memory_region::v1_0_3::MemoryRegionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for MemoryRegionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionMemoryChunks {
        V010003(crate::memory_region::v1_0_3::MemoryChunk),
        V000001(crate::memory_region::v1_0_3::MemoryRegionMemoryChunksN1),
    }
    impl Default for MemoryRegionMemoryChunks {
        fn default() -> Self {
            Self::V010003(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionMemoryChunksN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionMemoryExtents {
        V010003(crate::memory_region::v1_0_3::MemoryExtent),
        V000001(crate::memory_region::v1_0_3::MemoryRegionMemoryExtentsN1),
    }
    impl Default for MemoryRegionMemoryExtents {
        fn default() -> Self {
            Self::V010003(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionMemoryExtentsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
