pub type ServiceRoot = crate::service_root::v1_19_0::ServiceRoot;
pub mod v1_19_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DeepOperations {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepPATCH")]
        pub deep_patch: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepPOST")]
        pub deep_post: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLevels")]
        pub max_levels: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Expand {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandAll")]
        pub expand_all: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Levels")]
        pub levels: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxLevels")]
        pub max_levels: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NoLinks")]
        pub no_links: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagerProvidingService"
        )]
        pub manager_providing_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_19_0::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_19_0::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FilterQueryComparisonOperations"
        )]
        pub filter_query_comparison_operations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FilterQueryCompoundOperations"
        )]
        pub filter_query_compound_operations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IncludeOriginOfConditionQuery"
        )]
        pub include_origin_of_condition_query: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipleHTTPRequests"
        )]
        pub multiple_http_requests: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TopSkipQuery")]
        pub top_skip_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutomationNodes")]
        pub automation_nodes: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComponentIntegrity")]
        pub component_integrity: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::service_root::v1_19_0::ServiceRootDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facilities")]
        pub facilities: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobService")]
        pub job_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyService")]
        pub key_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseService")]
        pub license_service: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_19_0::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeDomains")]
        pub nvme_domains: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEquipment")]
        pub power_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_19_0::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegisteredClients")]
        pub registered_clients: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceConditions")]
        pub service_conditions: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceIdentification"
        )]
        pub service_identification: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageSystems")]
        pub storage_systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Systems")]
        pub systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TelemetryService")]
        pub telemetry_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThermalEquipment")]
        pub thermal_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ServiceRootDescription {
        V000001(crate::service_root::v1_19_0::ServiceRootDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ServiceRootDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ServiceRootDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
