pub type MemoryExtent = crate::memory_extent::v1_0_0::MemoryExtent;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_extent::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegions")]
        pub memory_regions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryRegions@odata.count"
        )]
        pub memory_regions_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_extent::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::memory_extent::v1_0_0::MemoryExtentDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExtentOffsetMiB")]
        pub extent_offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExtentSizeMiB")]
        pub extent_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FMReferencedExtent")]
        pub fm_referenced_extent: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory_extent::v1_0_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunks")]
        pub memory_chunks: Option<Vec<crate::memory_extent::v1_0_0::MemoryExtentMemoryChunks>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SequenceNumber")]
        pub sequence_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
        pub tag: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryExtentDescription {
        V000001(crate::memory_extent::v1_0_0::MemoryExtentDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for MemoryExtentDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryExtentDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryExtentMemoryChunks {
        V010000(crate::memory_extent::v1_0_0::MemoryChunk),
        V000001(crate::memory_extent::v1_0_0::MemoryExtentMemoryChunksN1),
    }
    impl Default for MemoryExtentMemoryChunks {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryExtentMemoryChunksN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
