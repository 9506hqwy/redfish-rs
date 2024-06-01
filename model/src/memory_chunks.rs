pub type MemoryChunks = crate::memory_chunks::v1_6_0::MemoryChunks;
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_chunks::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AddressRangeType {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InterleaveSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLevel")]
        pub memory_level: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediaLocation {
        #[default]
        #[serde(rename = "Local")]
        Local,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Remote")]
        Remote,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryChunks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_chunks::v1_5_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressRangeOffsetMiB"
        )]
        pub address_range_offset_mib: Option<i64>,
        #[serde(rename = "AddressRangeType")]
        pub address_range_type: Option<crate::memory_chunks::v1_5_0::AddressRangeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterleaveSets")]
        pub interleave_sets: Option<Vec<crate::memory_chunks::v1_5_0::InterleaveSet>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsMirrorEnabled")]
        pub is_mirror_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpare")]
        pub is_spare: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory_chunks::v1_5_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaLocation")]
        pub media_location: Option<crate::memory_chunks::v1_5_0::MediaLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunkSizeMiB")]
        pub memory_chunk_size_mib: Option<i64>,
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
            rename = "RequestedOperationalState"
        )]
        pub requested_operational_state: Option<crate::memory_chunks::v1_5_0::OperationalState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationalState {
        #[default]
        #[serde(rename = "Offline")]
        Offline,
        #[serde(rename = "Online")]
        Online,
    }
}
pub mod v1_6_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_chunks::v1_6_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AddressRangeType {
        #[default]
        #[serde(rename = "Block")]
        Block,
        #[serde(rename = "PMEM")]
        PMEM,
        #[serde(rename = "Volatile")]
        Volatile,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InterleaveSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryLevel")]
        pub memory_level: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OffsetMiB")]
        pub offset_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionId")]
        pub region_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SizeMiB")]
        pub size_mib: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
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
    pub enum MediaLocation {
        #[default]
        #[serde(rename = "Local")]
        Local,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Remote")]
        Remote,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryChunks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_chunks::v1_6_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressRangeOffsetMiB"
        )]
        pub address_range_offset_mib: Option<i64>,
        #[serde(rename = "AddressRangeType")]
        pub address_range_type: Option<crate::memory_chunks::v1_6_0::AddressRangeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterleaveSets")]
        pub interleave_sets: Option<Vec<crate::memory_chunks::v1_6_0::InterleaveSet>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsMirrorEnabled")]
        pub is_mirror_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSpare")]
        pub is_spare: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::memory_chunks::v1_6_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaLocation")]
        pub media_location: Option<crate::memory_chunks::v1_6_0::MediaLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunkSizeMiB")]
        pub memory_chunk_size_mib: Option<i64>,
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
            rename = "RequestedOperationalState"
        )]
        pub requested_operational_state: Option<crate::memory_chunks::v1_6_0::OperationalState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationalState {
        #[default]
        #[serde(rename = "Offline")]
        Offline,
        #[serde(rename = "Online")]
        Online,
    }
}
