use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ApplicationConsistencyMethod {
    #[default]
    #[serde(rename = "HotStandby")]
    HotStandby,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "VASA")]
    VASA,
    #[serde(rename = "VDI")]
    VDI,
    #[serde(rename = "VSS")]
    VSS,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ConsistencyGroup {
    V010101(crate::swordfish::consistency_group::v1_1_1::ConsistencyGroup),
    V010001(crate::swordfish::consistency_group::v1_0_1::ConsistencyGroup),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ConsistencyType {
    #[default]
    #[serde(rename = "ApplicationConsistent")]
    ApplicationConsistent,
    #[serde(rename = "CrashConsistent")]
    CrashConsistent,
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.AssignReplicaTarget"
        )]
        pub consistency_group_assign_replica_target:
            Option<crate::swordfish::consistency_group::v1_0_1::AssignReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.CreateReplicaTarget"
        )]
        pub consistency_group_create_replica_target:
            Option<crate::swordfish::consistency_group::v1_0_1::CreateReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.RemoveReplicaRelationship"
        )]
        pub consistency_group_remove_replica_relationship:
            Option<crate::swordfish::consistency_group::v1_0_1::RemoveReplicaRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.ResumeReplication"
        )]
        pub consistency_group_resume_replication:
            Option<crate::swordfish::consistency_group::v1_0_1::ResumeReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.ReverseReplicationRelationship"
        )]
        pub consistency_group_reverse_replication_relationship:
            Option<crate::swordfish::consistency_group::v1_0_1::ReverseReplicationRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.SplitReplication"
        )]
        pub consistency_group_split_replication:
            Option<crate::swordfish::consistency_group::v1_0_1::SplitReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.SuspendReplication"
        )]
        pub consistency_group_suspend_replication:
            Option<crate::swordfish::consistency_group::v1_0_1::SuspendReplication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::consistency_group::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConsistencyGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::consistency_group::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyMethod")]
        pub consistency_method:
            Option<crate::swordfish::consistency_group::ApplicationConsistencyMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<crate::swordfish::consistency_group::ConsistencyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsConsistent")]
        pub is_consistent: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::consistency_group::v1_0_1::Links>,
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
        pub replica_info: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTargetRequestBody {
        #[serde(rename = "ConsistencyGroupName")]
        pub consistency_group_name: String,
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetStoragePool")]
        pub target_storage_pool: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationshipRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeleteTargetConsistencyGroup"
        )]
        pub delete_target_consistency_group: Option<bool>,
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationshipRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
}
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.AssignReplicaTarget"
        )]
        pub consistency_group_assign_replica_target:
            Option<crate::swordfish::consistency_group::v1_1_1::AssignReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.CreateReplicaTarget"
        )]
        pub consistency_group_create_replica_target:
            Option<crate::swordfish::consistency_group::v1_1_1::CreateReplicaTarget>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.RemoveReplicaRelationship"
        )]
        pub consistency_group_remove_replica_relationship:
            Option<crate::swordfish::consistency_group::v1_1_1::RemoveReplicaRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.ResumeReplication"
        )]
        pub consistency_group_resume_replication:
            Option<crate::swordfish::consistency_group::v1_1_1::ResumeReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.ReverseReplicationRelationship"
        )]
        pub consistency_group_reverse_replication_relationship:
            Option<crate::swordfish::consistency_group::v1_1_1::ReverseReplicationRelationship>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.SplitReplication"
        )]
        pub consistency_group_split_replication:
            Option<crate::swordfish::consistency_group::v1_1_1::SplitReplication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ConsistencyGroup.SuspendReplication"
        )]
        pub consistency_group_suspend_replication:
            Option<crate::swordfish::consistency_group::v1_1_1::SuspendReplication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::consistency_group::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AssignReplicaTargetRequestBody {
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConsistencyGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::consistency_group::v1_1_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyMethod")]
        pub consistency_method:
            Option<crate::swordfish::consistency_group::ApplicationConsistencyMethod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<crate::swordfish::consistency_group::ConsistencyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsConsistent")]
        pub is_consistent: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::consistency_group::v1_1_1::Links>,
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
            rename = "RemoteReplicaTargets"
        )]
        pub remote_replica_targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaInfo")]
        pub replica_info: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaTargets")]
        pub replica_targets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaTargets@odata.count"
        )]
        pub replica_targets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volumes")]
        pub volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Volumes@odata.count"
        )]
        pub volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTarget {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicaTargetRequestBody {
        #[serde(rename = "ConsistencyGroupName")]
        pub consistency_group_name: String,
        #[serde(rename = "ReplicaType")]
        pub replica_type: crate::swordfish::storage_replica_info::ReplicaType,
        #[serde(rename = "ReplicaUpdateMode")]
        pub replica_update_mode: crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        #[serde(rename = "TargetStoragePool")]
        pub target_storage_pool: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveReplicaRelationshipRequestBody {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeleteTargetConsistencyGroup"
        )]
        pub delete_target_consistency_group: Option<bool>,
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResumeReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationship {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReverseReplicationRelationshipRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SplitReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SuspendReplicationRequestBody {
        #[serde(rename = "TargetConsistencyGroup")]
        pub target_consistency_group: String,
    }
}
