use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AccessCapability {
    #[default]
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "ReadWrite")]
    ReadWrite,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MappedVolume {
    #[serde(skip_serializing_if = "Option::is_none", rename = "AccessCapability")]
    pub access_capability: Option<crate::swordfish::storage_group::AccessCapability>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalUnitNumber")]
    pub logical_unit_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
    pub volume: Option<crate::odata_v4::IdRef>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StorageGroup {
    V010500(crate::swordfish::storage_group::v1_5_0::StorageGroup),
    V010400(crate::swordfish::storage_group::v1_4_0::StorageGroup),
    V010300(crate::swordfish::storage_group::v1_3_0::StorageGroup),
    V010202(crate::swordfish::storage_group::v1_2_2::StorageGroup),
    V010102(crate::swordfish::storage_group::v1_1_2::StorageGroup),
    V010003(crate::swordfish::storage_group::v1_0_3::StorageGroup),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
impl Default for StorageGroup {
    fn default() -> Self {
        Self::V010500(Default::default())
    }
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_0_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_0_3::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_0_3::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_0_3::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_0_3::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_0_3::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_0_3::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_1_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_1_2::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_1_2::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_1_2::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_1_2::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_1_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_1_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MappedVolumes")]
        pub mapped_volumes: Option<Vec<crate::swordfish::storage_group::MappedVolume>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_2_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_2_2::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_2_2::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "DHCHAP")]
        DHCHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPInformation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorCHAPUser")]
        pub initiator_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPUser")]
        pub target_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetPassword")]
        pub target_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_2_2::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_2_2::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_2_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::swordfish::storage_group::v1_2_2::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChapInfo")]
        pub chap_info: Option<Vec<crate::swordfish::storage_group::v1_2_2::CHAPInformation>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_2_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MappedVolumes")]
        pub mapped_volumes: Option<Vec<crate::swordfish::storage_group::MappedVolume>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_3_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_3_0::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_3_0::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "DHCHAP")]
        DHCHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPInformation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPPassword")]
        pub chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUser")]
        pub chap_user: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorCHAPUser")]
        pub initiator_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPPassword")]
        pub target_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPUser")]
        pub target_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetPassword")]
        pub target_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCHAPInformation {
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
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_3_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_3_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_3_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::swordfish::storage_group::v1_3_0::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChapInfo")]
        pub chap_info: Option<Vec<crate::swordfish::storage_group::v1_3_0::CHAPInformation>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHChapInfo")]
        pub dh_chap_info: Option<Vec<crate::swordfish::storage_group::v1_3_0::DHCHAPInformation>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_3_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MappedVolumes")]
        pub mapped_volumes: Option<Vec<crate::swordfish::storage_group::MappedVolume>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_4_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_4_0::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_4_0::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "DHCHAP")]
        DHCHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPInformation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPPassword")]
        pub chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUser")]
        pub chap_user: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorCHAPUser")]
        pub initiator_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPPassword")]
        pub target_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPUser")]
        pub target_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetPassword")]
        pub target_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCHAPInformation {
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
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_4_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_4_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_4_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::swordfish::storage_group::v1_4_0::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChapInfo")]
        pub chap_info: Option<Vec<crate::swordfish::storage_group::v1_4_0::CHAPInformation>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHChapInfo")]
        pub dh_chap_info: Option<Vec<crate::swordfish::storage_group::v1_4_0::DHCHAPInformation>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_4_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MappedVolumes")]
        pub mapped_volumes: Option<Vec<crate::swordfish::storage_group::MappedVolume>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_group::v1_5_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.ExposeVolumes"
        )]
        pub storage_group_expose_volumes:
            Option<crate::swordfish::storage_group::v1_5_0::ExposeVolumes>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageGroup.HideVolumes"
        )]
        pub storage_group_hide_volumes:
            Option<crate::swordfish::storage_group::v1_5_0::HideVolumes>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationMethod {
        #[default]
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "DHCHAP")]
        DHCHAP,
        #[serde(rename = "MutualCHAP")]
        MutualCHAP,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CHAPInformation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPPassword")]
        pub chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CHAPUser")]
        pub chap_user: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InitiatorCHAPPassword"
        )]
        pub initiator_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitiatorCHAPUser")]
        pub initiator_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPPassword")]
        pub target_chap_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetCHAPUser")]
        pub target_chap_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetPassword")]
        pub target_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DHCHAPInformation {
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
    pub struct ExposeVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExposeVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumes {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HideVolumesRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChildStorageGroups")]
        pub child_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_5_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChildStorageGroups@odata.count"
        )]
        pub child_storage_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups"
        )]
        pub parent_storage_groups:
            Option<Vec<crate::swordfish::storage_group::v1_5_0::StorageGroup>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ParentStorageGroups@odata.count"
        )]
        pub parent_storage_groups_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::endpoint_group::AccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_group::v1_5_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationMethod"
        )]
        pub authentication_method:
            Option<crate::swordfish::storage_group::v1_5_0::AuthenticationMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChapInfo")]
        pub chap_info: Option<Vec<crate::swordfish::storage_group::v1_5_0::CHAPInformation>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups"
        )]
        pub client_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClientEndpointGroups@odata.count"
        )]
        pub client_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHChapInfo")]
        pub dh_chap_info: Option<Vec<crate::swordfish::storage_group::v1_5_0::DHCHAPInformation>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::storage_group::v1_5_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MappedVolumes")]
        pub mapped_volumes: Option<Vec<crate::swordfish::storage_group::MappedVolume>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MembersAreConsistent"
        )]
        pub members_are_consistent: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups"
        )]
        pub server_endpoint_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServerEndpointGroups@odata.count"
        )]
        pub server_endpoint_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VolumesAreExposed")]
        pub volumes_are_exposed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
}
