pub type Connection = crate::connection::v1_4_0::Connection;
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccessCapability {
        #[default]
        #[serde(rename = "Read")]
        Read,
        #[serde(rename = "Write")]
        Write,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccessState {
        #[default]
        #[serde(rename = "NonOptimized")]
        NonOptimized,
        #[serde(rename = "Optimized")]
        Optimized,
        #[serde(rename = "Standby")]
        Standby,
        #[serde(rename = "Transitioning")]
        Transitioning,
        #[serde(rename = "Unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::connection::v1_3_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPPassword")]
        pub chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUsername")]
        pub chap_username: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPUsername"
        )]
        pub initiator_chap_username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPPassword")]
        pub target_chap_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Connection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::connection::v1_3_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionKeys")]
        pub connection_keys: Option<crate::connection::v1_3_2::ConnectionKey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionType")]
        pub connection_type: Option<crate::connection::v1_3_2::ConnectionConnectionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::connection::v1_3_2::ConnectionDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection::v1_3_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunkInfo")]
        pub memory_chunk_info: Option<Vec<crate::connection::v1_3_2::MemoryChunkInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegionInfo")]
        pub memory_region_info: Option<Vec<crate::connection::v1_3_2::MemoryRegionInfo>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeInfo")]
        pub volume_info: Option<Vec<crate::connection::v1_3_2::ConnectionVolumeInfo>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionConnectionType {
        V010302(crate::connection::v1_3_2::ConnectionType),
        V000001(crate::connection::v1_3_2::ConnectionConnectionTypeN1),
    }
    impl Default for ConnectionConnectionType {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionConnectionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionDescription {
        V000001(crate::connection::v1_3_2::ConnectionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ConnectionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAP")]
        pub chap: Option<crate::connection::v1_3_2::ConnectionKeyCHAP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCHAP")]
        pub dhchap: Option<crate::connection::v1_3_2::ConnectionKeyDHCHAP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::connection::v1_3_2::ConnectionKeyGenZ>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyCHAP {
        V010302(crate::connection::v1_3_2::CHAPConnectionKey),
        V000001(crate::connection::v1_3_2::ConnectionKeyCHAPN1),
    }
    impl Default for ConnectionKeyCHAP {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyCHAPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyDHCHAP {
        V010302(crate::connection::v1_3_2::DHCHAPKey),
        V000001(crate::connection::v1_3_2::ConnectionKeyDHCHAPN1),
    }
    impl Default for ConnectionKeyDHCHAP {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyDHCHAPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyGenZ {
        V010302(crate::connection::v1_3_2::GenZConnectionKey),
        V000001(crate::connection::v1_3_2::ConnectionKeyGenZN1),
    }
    impl Default for ConnectionKeyGenZ {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyGenZN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionType {
        #[default]
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Storage")]
        Storage,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionVolumeInfo {
        V010302(crate::connection::v1_3_2::VolumeInfo),
        V000001(crate::connection::v1_3_2::ConnectionVolumeInfoN1),
    }
    impl Default for ConnectionVolumeInfo {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionVolumeInfoN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCHAPKey {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalDHCHAPAuthSecret"
        )]
        pub local_dhchap_auth_secret: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeerDHCHAPAuthSecret"
        )]
        pub peer_dhchap_auth_secret: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RKeyDomainCheckingEnabled"
        )]
        pub r_key_domain_checking_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RKeyReadOnlyKey")]
        pub r_key_read_only_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RKeyReadWriteKey")]
        pub r_key_read_write_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpointGroups"
        )]
        pub initiator_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpointGroups@odata.count"
        )]
        pub initiator_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorEndpoints")]
        pub initiator_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpoints@odata.count"
        )]
        pub initiator_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroups"
        )]
        pub target_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroups@odata.count"
        )]
        pub target_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetEndpoints")]
        pub target_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpoints@odata.count"
        )]
        pub target_endpoints_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryChunkInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_3_2::MemoryChunkInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_3_2::MemoryChunkInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunk")]
        pub memory_chunk: Option<crate::connection::v1_3_2::MemoryChunkInfoMemoryChunk>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoAccessCapabilities {
        V010302(crate::connection::v1_3_2::AccessCapability),
        V000001(crate::connection::v1_3_2::MemoryChunkInfoAccessCapabilitiesN1),
    }
    impl Default for MemoryChunkInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoAccessState {
        V010302(crate::connection::v1_3_2::AccessState),
        V000001(crate::connection::v1_3_2::MemoryChunkInfoAccessStateN1),
    }
    impl Default for MemoryChunkInfoAccessState {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoMemoryChunk {
        V000001(crate::connection::v1_3_2::MemoryChunkInfoMemoryChunkN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for MemoryChunkInfoMemoryChunk {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoMemoryChunkN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryRegionInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_3_2::MemoryRegionInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_3_2::MemoryRegionInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegion")]
        pub memory_region: Option<crate::connection::v1_3_2::MemoryRegionInfoMemoryRegion>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoAccessCapabilities {
        V010302(crate::connection::v1_3_2::AccessCapability),
        V000001(crate::connection::v1_3_2::MemoryRegionInfoAccessCapabilitiesN1),
    }
    impl Default for MemoryRegionInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoAccessState {
        V010302(crate::connection::v1_3_2::AccessState),
        V000001(crate::connection::v1_3_2::MemoryRegionInfoAccessStateN1),
    }
    impl Default for MemoryRegionInfoAccessState {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoMemoryRegion {
        V000001(crate::connection::v1_3_2::MemoryRegionInfoMemoryRegionN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for MemoryRegionInfoMemoryRegion {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoMemoryRegionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VolumeInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_3_2::VolumeInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_3_2::VolumeInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUN")]
        pub lun: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
        pub volume: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInfoAccessCapabilities {
        V010302(crate::connection::v1_3_2::AccessCapability),
        V000001(crate::connection::v1_3_2::VolumeInfoAccessCapabilitiesN1),
    }
    impl Default for VolumeInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInfoAccessState {
        V010302(crate::connection::v1_3_2::AccessState),
        V000001(crate::connection::v1_3_2::VolumeInfoAccessStateN1),
    }
    impl Default for VolumeInfoAccessState {
        fn default() -> Self {
            Self::V010302(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccessCapability {
        #[default]
        #[serde(rename = "Read")]
        Read,
        #[serde(rename = "Write")]
        Write,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccessState {
        #[default]
        #[serde(rename = "NonOptimized")]
        NonOptimized,
        #[serde(rename = "Optimized")]
        Optimized,
        #[serde(rename = "Standby")]
        Standby,
        #[serde(rename = "Transitioning")]
        Transitioning,
        #[serde(rename = "Unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Connection.AddVolumeInfo"
        )]
        pub connection_add_volume_info: Option<crate::connection::v1_4_0::AddVolumeInfo>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Connection.RemoveVolumeInfo"
        )]
        pub connection_remove_volume_info: Option<crate::connection::v1_4_0::RemoveVolumeInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::connection::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddVolumeInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddVolumeInfoRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities: Option<Vec<crate::connection::v1_4_0::AccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUN")]
        pub lun: Option<i64>,
        #[serde(rename = "Volume")]
        pub volume: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPPassword")]
        pub chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUsername")]
        pub chap_username: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPUsername"
        )]
        pub initiator_chap_username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPPassword")]
        pub target_chap_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Connection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::connection::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionKeys")]
        pub connection_keys: Option<crate::connection::v1_4_0::ConnectionKey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionType")]
        pub connection_type: Option<crate::connection::v1_4_0::ConnectionConnectionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::connection::v1_4_0::ConnectionDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection::v1_4_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunkInfo")]
        pub memory_chunk_info: Option<Vec<crate::connection::v1_4_0::MemoryChunkInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegionInfo")]
        pub memory_region_info: Option<Vec<crate::connection::v1_4_0::MemoryRegionInfo>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeInfo")]
        pub volume_info: Option<Vec<crate::connection::v1_4_0::ConnectionVolumeInfo>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionConnectionType {
        V010400(crate::connection::v1_4_0::ConnectionType),
        V000001(crate::connection::v1_4_0::ConnectionConnectionTypeN1),
    }
    impl Default for ConnectionConnectionType {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionConnectionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionDescription {
        V000001(crate::connection::v1_4_0::ConnectionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ConnectionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAP")]
        pub chap: Option<crate::connection::v1_4_0::ConnectionKeyCHAP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCHAP")]
        pub dhchap: Option<crate::connection::v1_4_0::ConnectionKeyDHCHAP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::connection::v1_4_0::ConnectionKeyGenZ>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyCHAP {
        V010400(crate::connection::v1_4_0::CHAPConnectionKey),
        V000001(crate::connection::v1_4_0::ConnectionKeyCHAPN1),
    }
    impl Default for ConnectionKeyCHAP {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyCHAPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyDHCHAP {
        V010400(crate::connection::v1_4_0::DHCHAPKey),
        V000001(crate::connection::v1_4_0::ConnectionKeyDHCHAPN1),
    }
    impl Default for ConnectionKeyDHCHAP {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyDHCHAPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionKeyGenZ {
        V010400(crate::connection::v1_4_0::GenZConnectionKey),
        V000001(crate::connection::v1_4_0::ConnectionKeyGenZN1),
    }
    impl Default for ConnectionKeyGenZ {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionKeyGenZN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionType {
        #[default]
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Storage")]
        Storage,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionVolumeInfo {
        V010400(crate::connection::v1_4_0::VolumeInfo),
        V000001(crate::connection::v1_4_0::ConnectionVolumeInfoN1),
    }
    impl Default for ConnectionVolumeInfo {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionVolumeInfoN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCHAPKey {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalDHCHAPAuthSecret"
        )]
        pub local_dhchap_auth_secret: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PeerDHCHAPAuthSecret"
        )]
        pub peer_dhchap_auth_secret: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RKeyDomainCheckingEnabled"
        )]
        pub r_key_domain_checking_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RKeyReadOnlyKey")]
        pub r_key_read_only_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RKeyReadWriteKey")]
        pub r_key_read_write_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpointGroups"
        )]
        pub initiator_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpointGroups@odata.count"
        )]
        pub initiator_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorEndpoints")]
        pub initiator_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorEndpoints@odata.count"
        )]
        pub initiator_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroups"
        )]
        pub target_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpointGroups@odata.count"
        )]
        pub target_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetEndpoints")]
        pub target_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetEndpoints@odata.count"
        )]
        pub target_endpoints_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryChunkInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_4_0::MemoryChunkInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_4_0::MemoryChunkInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunk")]
        pub memory_chunk: Option<crate::connection::v1_4_0::MemoryChunkInfoMemoryChunk>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoAccessCapabilities {
        V010400(crate::connection::v1_4_0::AccessCapability),
        V000001(crate::connection::v1_4_0::MemoryChunkInfoAccessCapabilitiesN1),
    }
    impl Default for MemoryChunkInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoAccessState {
        V010400(crate::connection::v1_4_0::AccessState),
        V000001(crate::connection::v1_4_0::MemoryChunkInfoAccessStateN1),
    }
    impl Default for MemoryChunkInfoAccessState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryChunkInfoMemoryChunk {
        V000001(crate::connection::v1_4_0::MemoryChunkInfoMemoryChunkN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for MemoryChunkInfoMemoryChunk {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryChunkInfoMemoryChunkN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryRegionInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_4_0::MemoryRegionInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_4_0::MemoryRegionInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryRegion")]
        pub memory_region: Option<crate::connection::v1_4_0::MemoryRegionInfoMemoryRegion>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoAccessCapabilities {
        V010400(crate::connection::v1_4_0::AccessCapability),
        V000001(crate::connection::v1_4_0::MemoryRegionInfoAccessCapabilitiesN1),
    }
    impl Default for MemoryRegionInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoAccessState {
        V010400(crate::connection::v1_4_0::AccessState),
        V000001(crate::connection::v1_4_0::MemoryRegionInfoAccessStateN1),
    }
    impl Default for MemoryRegionInfoAccessState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MemoryRegionInfoMemoryRegion {
        V000001(crate::connection::v1_4_0::MemoryRegionInfoMemoryRegionN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for MemoryRegionInfoMemoryRegion {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MemoryRegionInfoMemoryRegionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveVolumeInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveVolumeInfoRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUN")]
        pub lun: Option<i64>,
        #[serde(rename = "Volume")]
        pub volume: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VolumeInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities:
            Option<Vec<crate::connection::v1_4_0::VolumeInfoAccessCapabilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_4_0::VolumeInfoAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUN")]
        pub lun: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
        pub volume: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInfoAccessCapabilities {
        V010400(crate::connection::v1_4_0::AccessCapability),
        V000001(crate::connection::v1_4_0::VolumeInfoAccessCapabilitiesN1),
    }
    impl Default for VolumeInfoAccessCapabilities {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInfoAccessCapabilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VolumeInfoAccessState {
        V010400(crate::connection::v1_4_0::AccessState),
        V000001(crate::connection::v1_4_0::VolumeInfoAccessStateN1),
    }
    impl Default for VolumeInfoAccessState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VolumeInfoAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
