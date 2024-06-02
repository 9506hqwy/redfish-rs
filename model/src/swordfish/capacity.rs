use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Capacity {
    V010201(crate::swordfish::capacity::v1_2_1::Capacity),
    V010104(crate::swordfish::capacity::v1_1_4::Capacity),
    V010004(crate::swordfish::capacity::v1_0_4::Capacity),
}
impl Default for Capacity {
    fn default() -> Self {
        Self::V010201(Default::default())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CapacityInfo {
    V010201(crate::swordfish::capacity::v1_2_1::CapacityInfo),
    V010104(crate::swordfish::capacity::v1_1_4::CapacityInfo),
    V010004(crate::swordfish::capacity::v1_0_4::CapacityInfo),
}
impl Default for CapacityInfo {
    fn default() -> Self {
        Self::V010201(Default::default())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CapacitySource {
    V010201(crate::swordfish::capacity::v1_2_1::CapacitySource),
    V010104(crate::swordfish::capacity::v1_1_4::CapacitySource),
    V010004(crate::swordfish::capacity::v1_0_4::CapacitySource),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
impl Default for CapacitySource {
    fn default() -> Self {
        Self::V010201(Default::default())
    }
}
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
        pub description: Option<String>,
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
}
pub mod v1_0_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Capacity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<crate::swordfish::capacity::v1_0_4::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsThinProvisioned")]
        pub is_thin_provisioned: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metadata")]
        pub metadata: Option<crate::swordfish::capacity::v1_0_4::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Snapshot")]
        pub snapshot: Option<crate::swordfish::capacity::v1_0_4::CapacityInfo>,
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
        pub description: Option<String>,
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
        pub provided_capacity: Option<crate::swordfish::capacity::v1_0_4::Capacity>,
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
}
pub mod v1_1_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::capacity::v1_1_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Capacity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<crate::swordfish::capacity::v1_1_4::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsThinProvisioned")]
        pub is_thin_provisioned: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metadata")]
        pub metadata: Option<crate::swordfish::capacity::v1_1_4::CapacityInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Snapshot")]
        pub snapshot: Option<crate::swordfish::capacity::v1_1_4::CapacityInfo>,
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
        pub actions: Option<crate::swordfish::capacity::v1_1_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        pub provided_capacity: Option<crate::swordfish::capacity::v1_1_4::Capacity>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
        pub description: Option<String>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
