pub mod v1_2_0 {
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
        pub oem: Option<crate::connection::v1_2_0::OemActions>,
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
        pub actions: Option<crate::connection::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionKeys")]
        pub connection_keys: Option<crate::connection::v1_2_0::ConnectionKey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionType")]
        pub connection_type: Option<crate::connection::v1_2_0::ConnectionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection::v1_2_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunkInfo")]
        pub memory_chunk_info: Option<Vec<crate::connection::v1_2_0::MemoryChunkInfo>>,
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
        pub volume_info: Option<Vec<crate::connection::v1_2_0::VolumeInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAP")]
        pub chap: Option<crate::connection::v1_2_0::CHAPConnectionKey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCHAP")]
        pub dhchap: Option<crate::connection::v1_2_0::DHCHAPKey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::connection::v1_2_0::GenZConnectionKey>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionType {
        #[default]
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Storage")]
        Storage,
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
        pub access_capabilities: Option<Vec<crate::connection::v1_2_0::AccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_2_0::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryChunk")]
        pub memory_chunk: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VolumeInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapabilities")]
        pub access_capabilities: Option<Vec<crate::connection::v1_2_0::AccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::connection::v1_2_0::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LUN")]
        pub lun: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
        pub volume: Option<crate::odata_v4::IdRef>,
    }
}
