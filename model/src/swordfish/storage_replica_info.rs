use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReplicaFaultDomain {
    #[default]
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Remote")]
    Remote,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReplicaInfo {
    V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo),
    V010300(crate::swordfish::storage_replica_info::v1_3_0::ReplicaInfo),
    V010200(crate::swordfish::storage_replica_info::v1_2_0::ReplicaInfo),
    V010102(crate::swordfish::storage_replica_info::v1_1_2::ReplicaInfo),
    V010002(crate::swordfish::storage_replica_info::v1_0_2::ReplicaInfo),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReplicaType {
    #[default]
    #[serde(rename = "Clone")]
    Clone,
    #[serde(rename = "Mirror")]
    Mirror,
    #[serde(rename = "Snapshot")]
    Snapshot,
    #[serde(rename = "TokenizedClone")]
    TokenizedClone,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReplicaUpdateMode {
    #[default]
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Adaptive")]
    Adaptive,
    #[serde(rename = "Asynchronous")]
    Asynchronous,
    #[serde(rename = "Synchronous")]
    Synchronous,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StorageReplicaInfo {
    V010400(crate::swordfish::storage_replica_info::v1_4_0::StorageReplicaInfo),
    V010300(crate::swordfish::storage_replica_info::v1_3_0::StorageReplicaInfo),
    V010200(crate::swordfish::storage_replica_info::v1_2_0::StorageReplicaInfo),
    V010102(crate::swordfish::storage_replica_info::v1_1_2::StorageReplicaInfo),
    V010002(crate::swordfish::storage_replica_info::v1_0_2::StorageReplicaInfo),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyState {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Inconsistent")]
        Inconsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyStatus {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "InError")]
        InError,
        #[serde(rename = "InProgress")]
        InProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyType {
        #[default]
        #[serde(rename = "SequentiallyConsistent")]
        SequentiallyConsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyEnabled")]
        pub consistency_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyState")]
        pub consistency_state:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ConsistencyType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FailedCopyStopsHostIO"
        )]
        pub failed_copy_stops_host_io: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentSynced")]
        pub percent_synced: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replica")]
        pub replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaProgressStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaReadOnlyAccess>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<crate::swordfish::storage_replica_info::ReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state:
            Option<crate::swordfish::storage_replica_info::v1_0_2::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_0_2::UndiscoveredElement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenActivated")]
        pub when_activated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenDeactivated")]
        pub when_deactivated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenEstablished")]
        pub when_established: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSuspended")]
        pub when_suspended: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynced")]
        pub when_synced: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynchronized")]
        pub when_synchronized: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaPriority {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Same")]
        Same,
        #[serde(rename = "Urgent")]
        Urgent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaProgressStatus {
        #[default]
        #[serde(rename = "Aborting")]
        Aborting,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Detaching")]
        Detaching,
        #[serde(rename = "Dormant")]
        Dormant,
        #[serde(rename = "FailingBack")]
        FailingBack,
        #[serde(rename = "FailingOver")]
        FailingOver,
        #[serde(rename = "Fracturing")]
        Fracturing,
        #[serde(rename = "Initializing")]
        Initializing,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Preparing")]
        Preparing,
        #[serde(rename = "RequiresActivate")]
        RequiresActivate,
        #[serde(rename = "RequiresDetach")]
        RequiresDetach,
        #[serde(rename = "RequiresFracture")]
        RequiresFracture,
        #[serde(rename = "RequiresResume")]
        RequiresResume,
        #[serde(rename = "RequiresResync")]
        RequiresResync,
        #[serde(rename = "RequiresSplit")]
        RequiresSplit,
        #[serde(rename = "Restoring")]
        Restoring,
        #[serde(rename = "Resyncing")]
        Resyncing,
        #[serde(rename = "Splitting")]
        Splitting,
        #[serde(rename = "Suspending")]
        Suspending,
        #[serde(rename = "Synchronizing")]
        Synchronizing,
        #[serde(rename = "Terminating")]
        Terminating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaReadOnlyAccess {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRecoveryMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRole {
        #[default]
        #[serde(rename = "Source")]
        Source,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaState {
        #[default]
        #[serde(rename = "Aborted")]
        Aborted,
        #[serde(rename = "Broken")]
        Broken,
        #[serde(rename = "Failedover")]
        Failedover,
        #[serde(rename = "Fractured")]
        Fractured,
        #[serde(rename = "Inactive")]
        Inactive,
        #[serde(rename = "Initialized")]
        Initialized,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Partitioned")]
        Partitioned,
        #[serde(rename = "Prepared")]
        Prepared,
        #[serde(rename = "Restored")]
        Restored,
        #[serde(rename = "Skewed")]
        Skewed,
        #[serde(rename = "Split")]
        Split,
        #[serde(rename = "Suspended")]
        Suspended,
        #[serde(rename = "Synchronized")]
        Synchronized,
        #[serde(rename = "Unsynchronized")]
        Unsynchronized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageReplicaInfo {
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyState {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Inconsistent")]
        Inconsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyStatus {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "InError")]
        InError,
        #[serde(rename = "InProgress")]
        InProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyType {
        #[default]
        #[serde(rename = "SequentiallyConsistent")]
        SequentiallyConsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyEnabled")]
        pub consistency_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyState")]
        pub consistency_state:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ConsistencyType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLineOfService"
        )]
        pub data_protection_line_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FailedCopyStopsHostIO"
        )]
        pub failed_copy_stops_host_io: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentSynced")]
        pub percent_synced: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replica")]
        pub replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaProgressStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaReadOnlyAccess>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<crate::swordfish::storage_replica_info::ReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state:
            Option<crate::swordfish::storage_replica_info::v1_1_2::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_1_2::UndiscoveredElement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenActivated")]
        pub when_activated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenDeactivated")]
        pub when_deactivated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenEstablished")]
        pub when_established: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSuspended")]
        pub when_suspended: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynced")]
        pub when_synced: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynchronized")]
        pub when_synchronized: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaPriority {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Same")]
        Same,
        #[serde(rename = "Urgent")]
        Urgent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaProgressStatus {
        #[default]
        #[serde(rename = "Aborting")]
        Aborting,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Detaching")]
        Detaching,
        #[serde(rename = "Dormant")]
        Dormant,
        #[serde(rename = "FailingBack")]
        FailingBack,
        #[serde(rename = "FailingOver")]
        FailingOver,
        #[serde(rename = "Fracturing")]
        Fracturing,
        #[serde(rename = "Initializing")]
        Initializing,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Preparing")]
        Preparing,
        #[serde(rename = "RequiresActivate")]
        RequiresActivate,
        #[serde(rename = "RequiresDetach")]
        RequiresDetach,
        #[serde(rename = "RequiresFracture")]
        RequiresFracture,
        #[serde(rename = "RequiresResume")]
        RequiresResume,
        #[serde(rename = "RequiresResync")]
        RequiresResync,
        #[serde(rename = "RequiresSplit")]
        RequiresSplit,
        #[serde(rename = "Restoring")]
        Restoring,
        #[serde(rename = "Resyncing")]
        Resyncing,
        #[serde(rename = "Splitting")]
        Splitting,
        #[serde(rename = "Suspending")]
        Suspending,
        #[serde(rename = "Synchronizing")]
        Synchronizing,
        #[serde(rename = "Terminating")]
        Terminating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaReadOnlyAccess {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRecoveryMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRole {
        #[default]
        #[serde(rename = "Source")]
        Source,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaState {
        #[default]
        #[serde(rename = "Aborted")]
        Aborted,
        #[serde(rename = "Broken")]
        Broken,
        #[serde(rename = "Failedover")]
        Failedover,
        #[serde(rename = "Fractured")]
        Fractured,
        #[serde(rename = "Inactive")]
        Inactive,
        #[serde(rename = "Initialized")]
        Initialized,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Partitioned")]
        Partitioned,
        #[serde(rename = "Prepared")]
        Prepared,
        #[serde(rename = "Restored")]
        Restored,
        #[serde(rename = "Skewed")]
        Skewed,
        #[serde(rename = "Split")]
        Split,
        #[serde(rename = "Suspended")]
        Suspended,
        #[serde(rename = "Synchronized")]
        Synchronized,
        #[serde(rename = "Unsynchronized")]
        Unsynchronized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageReplicaInfo {
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_replica_info::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyState {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Inconsistent")]
        Inconsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyStatus {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "InError")]
        InError,
        #[serde(rename = "InProgress")]
        InProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyType {
        #[default]
        #[serde(rename = "SequentiallyConsistent")]
        SequentiallyConsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyEnabled")]
        pub consistency_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyState")]
        pub consistency_state:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ConsistencyType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLineOfService"
        )]
        pub data_protection_line_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FailedCopyStopsHostIO"
        )]
        pub failed_copy_stops_host_io: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentSynced")]
        pub percent_synced: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replica")]
        pub replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaProgressStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaReadOnlyAccess>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<crate::swordfish::storage_replica_info::ReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state:
            Option<crate::swordfish::storage_replica_info::v1_2_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_2_0::UndiscoveredElement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenActivated")]
        pub when_activated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenDeactivated")]
        pub when_deactivated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenEstablished")]
        pub when_established: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSuspended")]
        pub when_suspended: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynced")]
        pub when_synced: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynchronized")]
        pub when_synchronized: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaPriority {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Same")]
        Same,
        #[serde(rename = "Urgent")]
        Urgent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaProgressStatus {
        #[default]
        #[serde(rename = "Aborting")]
        Aborting,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Detaching")]
        Detaching,
        #[serde(rename = "Dormant")]
        Dormant,
        #[serde(rename = "FailingBack")]
        FailingBack,
        #[serde(rename = "FailingOver")]
        FailingOver,
        #[serde(rename = "Fracturing")]
        Fracturing,
        #[serde(rename = "Initializing")]
        Initializing,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Preparing")]
        Preparing,
        #[serde(rename = "RequiresActivate")]
        RequiresActivate,
        #[serde(rename = "RequiresDetach")]
        RequiresDetach,
        #[serde(rename = "RequiresFracture")]
        RequiresFracture,
        #[serde(rename = "RequiresResume")]
        RequiresResume,
        #[serde(rename = "RequiresResync")]
        RequiresResync,
        #[serde(rename = "RequiresSplit")]
        RequiresSplit,
        #[serde(rename = "Restoring")]
        Restoring,
        #[serde(rename = "Resyncing")]
        Resyncing,
        #[serde(rename = "Splitting")]
        Splitting,
        #[serde(rename = "Suspending")]
        Suspending,
        #[serde(rename = "Synchronizing")]
        Synchronizing,
        #[serde(rename = "Terminating")]
        Terminating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaReadOnlyAccess {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRecoveryMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRole {
        #[default]
        #[serde(rename = "Source")]
        Source,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaState {
        #[default]
        #[serde(rename = "Aborted")]
        Aborted,
        #[serde(rename = "Broken")]
        Broken,
        #[serde(rename = "Failedover")]
        Failedover,
        #[serde(rename = "Fractured")]
        Fractured,
        #[serde(rename = "Inactive")]
        Inactive,
        #[serde(rename = "Initialized")]
        Initialized,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Partitioned")]
        Partitioned,
        #[serde(rename = "Prepared")]
        Prepared,
        #[serde(rename = "Restored")]
        Restored,
        #[serde(rename = "Skewed")]
        Skewed,
        #[serde(rename = "Split")]
        Split,
        #[serde(rename = "Suspended")]
        Suspended,
        #[serde(rename = "Synchronized")]
        Synchronized,
        #[serde(rename = "Unsynchronized")]
        Unsynchronized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_replica_info::v1_2_0::Actions>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_replica_info::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyState {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Inconsistent")]
        Inconsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyStatus {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "InError")]
        InError,
        #[serde(rename = "InProgress")]
        InProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyType {
        #[default]
        #[serde(rename = "SequentiallyConsistent")]
        SequentiallyConsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyEnabled")]
        pub consistency_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyState")]
        pub consistency_state:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ConsistencyType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLineOfService"
        )]
        pub data_protection_line_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FailedCopyStopsHostIO"
        )]
        pub failed_copy_stops_host_io: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentSynced")]
        pub percent_synced: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replica")]
        pub replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaFaultDomain")]
        pub replica_fault_domain:
            Option<crate::swordfish::storage_replica_info::ReplicaFaultDomain>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaProgressStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaReadOnlyAccess>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<crate::swordfish::storage_replica_info::ReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state:
            Option<crate::swordfish::storage_replica_info::v1_3_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_3_0::UndiscoveredElement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenActivated")]
        pub when_activated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenDeactivated")]
        pub when_deactivated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenEstablished")]
        pub when_established: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSuspended")]
        pub when_suspended: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynced")]
        pub when_synced: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynchronized")]
        pub when_synchronized: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaPriority {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Same")]
        Same,
        #[serde(rename = "Urgent")]
        Urgent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaProgressStatus {
        #[default]
        #[serde(rename = "Aborting")]
        Aborting,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Detaching")]
        Detaching,
        #[serde(rename = "Dormant")]
        Dormant,
        #[serde(rename = "FailingBack")]
        FailingBack,
        #[serde(rename = "FailingOver")]
        FailingOver,
        #[serde(rename = "Fracturing")]
        Fracturing,
        #[serde(rename = "Initializing")]
        Initializing,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Preparing")]
        Preparing,
        #[serde(rename = "RequiresActivate")]
        RequiresActivate,
        #[serde(rename = "RequiresDetach")]
        RequiresDetach,
        #[serde(rename = "RequiresFracture")]
        RequiresFracture,
        #[serde(rename = "RequiresResume")]
        RequiresResume,
        #[serde(rename = "RequiresResync")]
        RequiresResync,
        #[serde(rename = "RequiresSplit")]
        RequiresSplit,
        #[serde(rename = "Restoring")]
        Restoring,
        #[serde(rename = "Resyncing")]
        Resyncing,
        #[serde(rename = "Splitting")]
        Splitting,
        #[serde(rename = "Suspending")]
        Suspending,
        #[serde(rename = "Synchronizing")]
        Synchronizing,
        #[serde(rename = "Terminating")]
        Terminating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaReadOnlyAccess {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRecoveryMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRole {
        #[default]
        #[serde(rename = "Source")]
        Source,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaState {
        #[default]
        #[serde(rename = "Aborted")]
        Aborted,
        #[serde(rename = "Broken")]
        Broken,
        #[serde(rename = "Failedover")]
        Failedover,
        #[serde(rename = "Fractured")]
        Fractured,
        #[serde(rename = "Inactive")]
        Inactive,
        #[serde(rename = "Initialized")]
        Initialized,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Partitioned")]
        Partitioned,
        #[serde(rename = "Prepared")]
        Prepared,
        #[serde(rename = "Restored")]
        Restored,
        #[serde(rename = "Skewed")]
        Skewed,
        #[serde(rename = "Split")]
        Split,
        #[serde(rename = "Suspended")]
        Suspended,
        #[serde(rename = "Synchronized")]
        Synchronized,
        #[serde(rename = "Unsynchronized")]
        Unsynchronized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_replica_info::v1_3_0::Actions>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::storage_replica_info::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyState {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Inconsistent")]
        Inconsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyStatus {
        #[default]
        #[serde(rename = "Consistent")]
        Consistent,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "InError")]
        InError,
        #[serde(rename = "InProgress")]
        InProgress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConsistencyType {
        #[default]
        #[serde(rename = "SequentiallyConsistent")]
        SequentiallyConsistent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyEnabled")]
        pub consistency_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyState")]
        pub consistency_state:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ConsistencyType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLineOfService"
        )]
        pub data_protection_line_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FailedCopyStopsHostIO"
        )]
        pub failed_copy_stops_host_io: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PercentSynced")]
        pub percent_synced: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteSourceReplica"
        )]
        pub remote_source_replica: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replica")]
        pub replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaFaultDomain")]
        pub replica_fault_domain:
            Option<crate::swordfish::storage_replica_info::ReplicaFaultDomain>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaProgressStatus>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaReadOnlyAccess>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<crate::swordfish::storage_replica_info::ReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_4_0::UndiscoveredElement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenActivated")]
        pub when_activated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenDeactivated")]
        pub when_deactivated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenEstablished")]
        pub when_established: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSuspended")]
        pub when_suspended: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynced")]
        pub when_synced: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WhenSynchronized")]
        pub when_synchronized: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaPriority {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Same")]
        Same,
        #[serde(rename = "Urgent")]
        Urgent,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaProgressStatus {
        #[default]
        #[serde(rename = "Aborting")]
        Aborting,
        #[serde(rename = "Completed")]
        Completed,
        #[serde(rename = "Detaching")]
        Detaching,
        #[serde(rename = "Dormant")]
        Dormant,
        #[serde(rename = "FailingBack")]
        FailingBack,
        #[serde(rename = "FailingOver")]
        FailingOver,
        #[serde(rename = "Fracturing")]
        Fracturing,
        #[serde(rename = "Initializing")]
        Initializing,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Pending")]
        Pending,
        #[serde(rename = "Preparing")]
        Preparing,
        #[serde(rename = "RequiresActivate")]
        RequiresActivate,
        #[serde(rename = "RequiresDetach")]
        RequiresDetach,
        #[serde(rename = "RequiresFracture")]
        RequiresFracture,
        #[serde(rename = "RequiresResume")]
        RequiresResume,
        #[serde(rename = "RequiresResync")]
        RequiresResync,
        #[serde(rename = "RequiresSplit")]
        RequiresSplit,
        #[serde(rename = "Restoring")]
        Restoring,
        #[serde(rename = "Resyncing")]
        Resyncing,
        #[serde(rename = "Splitting")]
        Splitting,
        #[serde(rename = "Suspending")]
        Suspending,
        #[serde(rename = "Synchronizing")]
        Synchronizing,
        #[serde(rename = "Terminating")]
        Terminating,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaReadOnlyAccess {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRecoveryMode {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaRole {
        #[default]
        #[serde(rename = "Source")]
        Source,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaState {
        #[default]
        #[serde(rename = "Aborted")]
        Aborted,
        #[serde(rename = "Broken")]
        Broken,
        #[serde(rename = "Failedover")]
        Failedover,
        #[serde(rename = "Fractured")]
        Fractured,
        #[serde(rename = "Inactive")]
        Inactive,
        #[serde(rename = "Initialized")]
        Initialized,
        #[serde(rename = "Invalid")]
        Invalid,
        #[serde(rename = "Mixed")]
        Mixed,
        #[serde(rename = "Partitioned")]
        Partitioned,
        #[serde(rename = "Prepared")]
        Prepared,
        #[serde(rename = "Restored")]
        Restored,
        #[serde(rename = "Skewed")]
        Skewed,
        #[serde(rename = "Split")]
        Split,
        #[serde(rename = "Suspended")]
        Suspended,
        #[serde(rename = "Synchronized")]
        Synchronized,
        #[serde(rename = "Unsynchronized")]
        Unsynchronized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageReplicaInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::storage_replica_info::v1_4_0::Actions>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
