use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReplicaFaultDomain {
    #[default]
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Remote")]
    Remote,
}
pub type ReplicaInfo = crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfo;
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
pub type StorageReplicaInfo = crate::swordfish::storage_replica_info::v1_4_0::StorageReplicaInfo;
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
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyStatus")]
        pub consistency_status:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConsistencyType")]
        pub consistency_type:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyType>,
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
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaFaultDomain>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaPriority")]
        pub replica_priority:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaPriority>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaProgressStatus"
        )]
        pub replica_progress_status: Option<
            crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaProgressStatus,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaReadOnlyAccess"
        )]
        pub replica_read_only_access: Option<
            crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaReadOnlyAccess,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReplicaRecoveryMode"
        )]
        pub replica_recovery_mode:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaRecoveryMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRole")]
        pub replica_role:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSkewBytes")]
        pub replica_skew_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaState")]
        pub replica_state:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaType")]
        pub replica_type:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaUpdateMode")]
        pub replica_update_mode:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaUpdateMode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequestedReplicaState"
        )]
        pub requested_replica_state: Option<
            crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoRequestedReplicaState,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceReplica")]
        pub source_replica: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyncMaintained")]
        pub sync_maintained: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UndiscoveredElement"
        )]
        pub undiscovered_element:
            Option<crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoUndiscoveredElement>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoConsistencyState {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ConsistencyState),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyStateN1),
    }
    impl Default for ReplicaInfoConsistencyState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoConsistencyStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoConsistencyStatus {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ConsistencyStatus),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyStatusN1),
    }
    impl Default for ReplicaInfoConsistencyStatus {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoConsistencyStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoConsistencyType {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ConsistencyType),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoConsistencyTypeN1),
    }
    impl Default for ReplicaInfoConsistencyType {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoConsistencyTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaFaultDomain {
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaFaultDomainN1),
        StorageReplicaInfoReplicaFaultDomain(
            crate::swordfish::storage_replica_info::ReplicaFaultDomain,
        ),
    }
    impl Default for ReplicaInfoReplicaFaultDomain {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaFaultDomainN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaPriority {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaPriority),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaPriorityN1),
    }
    impl Default for ReplicaInfoReplicaPriority {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaPriorityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaProgressStatus {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaProgressStatus),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaProgressStatusN1),
    }
    impl Default for ReplicaInfoReplicaProgressStatus {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaProgressStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaReadOnlyAccess {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaReadOnlyAccess),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaReadOnlyAccessN1),
    }
    impl Default for ReplicaInfoReplicaReadOnlyAccess {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaReadOnlyAccessN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaRecoveryMode {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaRecoveryMode),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaRecoveryModeN1),
    }
    impl Default for ReplicaInfoReplicaRecoveryMode {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaRecoveryModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaRole {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaRole),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaRoleN1),
    }
    impl Default for ReplicaInfoReplicaRole {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaRoleN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaState {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaState),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaStateN1),
    }
    impl Default for ReplicaInfoReplicaState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaType {
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaTypeN1),
        StorageReplicaInfoReplicaType(crate::swordfish::storage_replica_info::ReplicaType),
    }
    impl Default for ReplicaInfoReplicaType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoReplicaUpdateMode {
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoReplicaUpdateModeN1),
        StorageReplicaInfoReplicaUpdateMode(
            crate::swordfish::storage_replica_info::ReplicaUpdateMode,
        ),
    }
    impl Default for ReplicaInfoReplicaUpdateMode {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoReplicaUpdateModeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoRequestedReplicaState {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::ReplicaState),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoRequestedReplicaStateN1),
    }
    impl Default for ReplicaInfoRequestedReplicaState {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoRequestedReplicaStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ReplicaInfoUndiscoveredElement {
        V010400(crate::swordfish::storage_replica_info::v1_4_0::UndiscoveredElement),
        V000001(crate::swordfish::storage_replica_info::v1_4_0::ReplicaInfoUndiscoveredElementN1),
    }
    impl Default for ReplicaInfoUndiscoveredElement {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReplicaInfoUndiscoveredElementN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub description:
            Option<crate::swordfish::storage_replica_info::v1_4_0::StorageReplicaInfoDescription>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageReplicaInfoDescription {
        V000001(crate::swordfish::storage_replica_info::v1_4_0::StorageReplicaInfoDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageReplicaInfoDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageReplicaInfoDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
