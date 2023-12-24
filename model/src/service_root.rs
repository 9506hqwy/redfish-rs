use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ServiceRoot {
    V011600(crate::service_root::v1_16_0::ServiceRoot),
    V011501(crate::service_root::v1_15_1::ServiceRoot),
    V011402(crate::service_root::v1_14_2::ServiceRoot),
    V011302(crate::service_root::v1_13_2::ServiceRoot),
    V011202(crate::service_root::v1_12_2::ServiceRoot),
    V011102(crate::service_root::v1_11_2::ServiceRoot),
    V011002(crate::service_root::v1_10_2::ServiceRoot),
    V010902(crate::service_root::v1_9_2::ServiceRoot),
    V010802(crate::service_root::v1_8_2::ServiceRoot),
    V010702(crate::service_root::v1_7_2::ServiceRoot),
    V010602(crate::service_root::v1_6_2::ServiceRoot),
    V010504(crate::service_root::v1_5_4::ServiceRoot),
    V010407(crate::service_root::v1_4_7::ServiceRoot),
    V010307(crate::service_root::v1_3_7::ServiceRoot),
    V010206(crate::service_root::v1_2_6::ServiceRoot),
    V010108(crate::service_root::v1_1_8::ServiceRoot),
    V010011(crate::service_root::v1_0_11::ServiceRoot),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_0_11::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Systems")]
        pub systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
}
pub mod v1_1_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_1_8::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageSystems")]
        pub storage_systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Systems")]
        pub systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
}
pub mod v1_2_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_2_6::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageSystems")]
        pub storage_systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Systems")]
        pub systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
}
pub mod v1_3_7 {
    use serde::{Deserialize, Serialize};
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_3_7::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_3_7::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_3_7::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageServices")]
        pub storage_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StorageSystems")]
        pub storage_systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Systems")]
        pub systems: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tasks")]
        pub tasks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
}
pub mod v1_4_7 {
    use serde::{Deserialize, Serialize};
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_4_7::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobService")]
        pub job_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_4_7::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_4_7::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
}
pub mod v1_5_4 {
    use serde::{Deserialize, Serialize};
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_5_4::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventService")]
        pub event_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fabrics")]
        pub fabrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JobService")]
        pub job_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "JsonSchemas")]
        pub json_schemas: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_5_4::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_5_4::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_6_2 {
    use serde::{Deserialize, Serialize};
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_6_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_6_2::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEquipment")]
        pub power_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_6_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_7_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_7_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_7_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_7_2::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEquipment")]
        pub power_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_7_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_8_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_8_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_8_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_8_2::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEquipment")]
        pub power_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_8_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionService")]
        pub session_service: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_9_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_9_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_9_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_9_2::Links,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Managers")]
        pub managers: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEquipment")]
        pub power_equipment: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Product")]
        pub product: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProtocolFeaturesSupported"
        )]
        pub protocol_features_supported:
            Option<crate::service_root::v1_9_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_10_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_10_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_10_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_10_2::Links,
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
            Option<crate::service_root::v1_10_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_11_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_11_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_11_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(rename = "Links")]
        pub links: crate::service_root::v1_11_2::Links,
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
            Option<crate::service_root::v1_11_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_12_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_12_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_12_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateService")]
        pub certificate_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CompositionService")]
        pub composition_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        pub links: crate::service_root::v1_12_2::Links,
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
            Option<crate::service_root::v1_12_2::ProtocolFeaturesSupported>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedfishVersion")]
        pub redfish_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Registries")]
        pub registries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_13_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_13_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_13_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
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
        pub description: Option<String>,
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
        pub links: crate::service_root::v1_13_2::Links,
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
            Option<crate::service_root::v1_13_2::ProtocolFeaturesSupported>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_14_2 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "Sessions")]
        pub sessions: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProtocolFeaturesSupported {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeepOperations")]
        pub deep_operations: Option<crate::service_root::v1_14_2::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_14_2::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipleHTTPRequests"
        )]
        pub multiple_http_requests: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
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
        pub description: Option<String>,
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
        pub links: crate::service_root::v1_14_2::Links,
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
            Option<crate::service_root::v1_14_2::ProtocolFeaturesSupported>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_15_1 {
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
        pub deep_operations: Option<crate::service_root::v1_15_1::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_15_1::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipleHTTPRequests"
        )]
        pub multiple_http_requests: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
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
        pub description: Option<String>,
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
        pub links: crate::service_root::v1_15_1::Links,
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
            Option<crate::service_root::v1_15_1::ProtocolFeaturesSupported>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpdateService")]
        pub update_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
}
pub mod v1_16_0 {
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
        pub deep_operations: Option<crate::service_root::v1_16_0::DeepOperations>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcerptQuery")]
        pub excerpt_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpandQuery")]
        pub expand_query: Option<crate::service_root::v1_16_0::Expand>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FilterQuery")]
        pub filter_query: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipleHTTPRequests"
        )]
        pub multiple_http_requests: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OnlyMemberQuery")]
        pub only_member_query: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SelectQuery")]
        pub select_query: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ServiceRoot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountService")]
        pub account_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationService")]
        pub aggregation_service: Option<crate::odata_v4::IdRef>,
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
        pub description: Option<String>,
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
        pub links: crate::service_root::v1_16_0::Links,
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
            Option<crate::service_root::v1_16_0::ProtocolFeaturesSupported>,
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
}
