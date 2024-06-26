pub type DataProtectionLineOfService =
    crate::swordfish::data_protection_line_of_service::v1_3_0::DataProtectionLineOfService;
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#DataProtectionLineOfService.CreateReplicas"
        )]
        pub data_protection_line_of_service_create_replicas:
            Option<crate::swordfish::data_protection_line_of_service::v1_3_0::CreateReplicas>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_protection_line_of_service::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicas {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CreateReplicasRequestBody {
        #[serde(rename = "ReplicaLineOfService")]
        pub replica_line_of_service: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaRequests")]
        pub replica_requests:
            Option<Vec<crate::swordfish::data_protection_line_of_service::v1_3_0::ReplicaRequest>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataProtectionLineOfService { # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceDescription > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "IsIsolated")] pub is_isolated : Option < bool > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MinLifetime")] pub min_lifetime : Option < String > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "RecoveryGeographicObjective")] pub recovery_geographic_objective : Option < crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceRecoveryGeographicObjective > , # [serde (skip_serializing_if = "Option::is_none" , rename = "RecoveryPointObjectiveTime")] pub recovery_point_objective_time : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "RecoveryTimeObjective")] pub recovery_time_objective : Option < crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceRecoveryTimeObjective > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReplicaAccessLocation")] pub replica_access_location : Option < crate :: resource :: v1_3_0 :: Location > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReplicaClassOfService")] pub replica_class_of_service : Option < crate :: odata_v4 :: IdRef > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ReplicaType")] pub replica_type : Option < crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceReplicaType > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Schedule")] pub schedule : Option < crate :: schedule :: Schedule > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataProtectionLineOfServiceDescription {
        V000001 (crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceDescriptionN1) , ResourceDescription (String) }
    impl Default for DataProtectionLineOfServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataProtectionLineOfServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataProtectionLineOfServiceRecoveryGeographicObjective {
        V000001 (crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceRecoveryGeographicObjectiveN1) , DataProtectionLoSCapabilitiesFailureDomainScope (crate :: swordfish :: data_protection_los_capabilities :: FailureDomainScope) }
    impl Default for DataProtectionLineOfServiceRecoveryGeographicObjective {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataProtectionLineOfServiceRecoveryGeographicObjectiveN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataProtectionLineOfServiceRecoveryTimeObjective {
        V000001 (crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceRecoveryTimeObjectiveN1) , DataProtectionLoSCapabilitiesRecoveryAccessScope (crate :: swordfish :: data_protection_los_capabilities :: RecoveryAccessScope) }
    impl Default for DataProtectionLineOfServiceRecoveryTimeObjective {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataProtectionLineOfServiceRecoveryTimeObjectiveN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataProtectionLineOfServiceReplicaType {
        V000001 (crate :: swordfish :: data_protection_line_of_service :: v1_3_0 :: DataProtectionLineOfServiceReplicaTypeN1) , StorageReplicaInfoReplicaType (crate :: swordfish :: storage_replica_info :: ReplicaType) }
    impl Default for DataProtectionLineOfServiceReplicaType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataProtectionLineOfServiceReplicaTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplicaRequest {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaName")]
        pub replica_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicaSource")]
        pub replica_source: Option<crate::odata_v4::IdRef>,
    }
}
