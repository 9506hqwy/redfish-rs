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
    StorageReplicaInfoV1N0N2ReplicaInfo(
        crate::swordfish::storage_replica_info::v1_0_2::ReplicaInfo,
    ),
    StorageReplicaInfoV1N1N2ReplicaInfo(
        crate::swordfish::storage_replica_info::v1_1_2::ReplicaInfo,
    ),
    StorageReplicaInfoV1N2N0ReplicaInfo(
        crate::swordfish::storage_replica_info::v1_2_0::ReplicaInfo,
    ),
    StorageReplicaInfoV1N3N0ReplicaInfo(
        crate::swordfish::storage_replica_info::v1_3_0::ReplicaInfo,
    ),
    StorageReplicaInfoV1N4N0ReplicaInfo(
        crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo,
    ),
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
        pub consistency_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<String>,
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
        pub replica_priority: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element: Option<String>,
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
        pub consistency_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<String>,
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
        pub replica_priority: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element: Option<String>,
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
        pub consistency_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<String>,
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
        pub replica_priority: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element: Option<String>,
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
        pub consistency_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<String>,
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
        pub replica_fault_domain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element: Option<String>,
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
        pub consistency_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type: Option<String>,
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
        pub replica_fault_domain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element: Option<String>,
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
    pub enum UndiscoveredElement {
        #[default]
        #[serde(rename = "ReplicaElement")]
        ReplicaElement,
        #[serde(rename = "SourceElement")]
        SourceElement,
    }
}
