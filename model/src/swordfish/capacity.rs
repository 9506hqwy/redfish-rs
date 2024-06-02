pub type Capacity = crate::swordfish::capacity::v1_2_1::Capacity;
pub type CapacityInfo = crate::swordfish::capacity::v1_2_1::CapacityInfo;
pub type CapacitySource = crate::swordfish::capacity::v1_2_1::CapacitySource;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Capacity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<crate::swordfish::capacity::v1_0_0::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsThinProvisioned")]
        pub is_thin_provisioned: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metadata")]
        pub metadata: Option<crate::swordfish::capacity::v1_0_0::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Snapshot")]
        pub snapshot: Option<crate::swordfish::capacity::v1_0_0::CapacityInfo>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CapacityInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedBytes")]
        pub allocated_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsumedBytes")]
        pub consumed_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GuaranteedBytes")]
        pub guaranteed_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisionedBytes")]
        pub provisioned_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CapacitySource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::capacity::v1_0_0::CapacitySourceDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidedCapacity")]
        pub provided_capacity: Option<crate::swordfish::capacity::v1_0_0::Capacity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidedClassOfService"
        )]
        pub provided_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingDrives")]
        pub providing_drives: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingPools")]
        pub providing_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingVolumes")]
        pub providing_volumes: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CapacitySourceDescription {
        V000001(crate::swordfish::capacity::v1_0_0::CapacitySourceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CapacitySourceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CapacitySourceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::capacity::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Capacity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<crate::swordfish::capacity::v1_2_1::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsThinProvisioned")]
        pub is_thin_provisioned: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metadata")]
        pub metadata: Option<crate::swordfish::capacity::v1_2_1::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Snapshot")]
        pub snapshot: Option<crate::swordfish::capacity::v1_2_1::CapacityInfo>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CapacityInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedBytes")]
        pub allocated_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsumedBytes")]
        pub consumed_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GuaranteedBytes")]
        pub guaranteed_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvisionedBytes")]
        pub provisioned_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CapacitySource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::capacity::v1_2_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::capacity::v1_2_1::CapacitySourceDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidedCapacity")]
        pub provided_capacity: Option<crate::swordfish::capacity::v1_2_1::Capacity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidedClassOfService"
        )]
        pub provided_class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingDrives")]
        pub providing_drives: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingMemory")]
        pub providing_memory: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidingMemoryChunks"
        )]
        pub providing_memory_chunks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingPools")]
        pub providing_pools: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProvidingVolumes")]
        pub providing_volumes: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CapacitySourceDescription {
        V000001(crate::swordfish::capacity::v1_2_1::CapacitySourceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CapacitySourceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CapacitySourceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
