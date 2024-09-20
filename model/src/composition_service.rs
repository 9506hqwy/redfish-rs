pub type CompositionService = crate::composition_service::v1_2_3::CompositionService;
pub mod v1_2_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#CompositionService.Compose"
        )]
        pub composition_service_compose: Option<crate::composition_service::v1_2_3::Compose>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::composition_service::v1_2_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Compose {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComposeRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manifest")]
        pub manifest: Option<crate::manifest::Manifest>,
        #[serde(rename = "RequestFormat")]
        pub request_format: crate::composition_service::v1_2_3::ComposeRequestFormat,
        #[serde(rename = "RequestType")]
        pub request_type: crate::composition_service::v1_2_3::ComposeRequestType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReservationId")]
        pub reservation_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComposeRequestFormat {
        #[default]
        #[serde(rename = "Manifest")]
        Manifest,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComposeRequestType {
        #[default]
        #[serde(rename = "Apply")]
        Apply,
        #[serde(rename = "Preview")]
        Preview,
        #[serde(rename = "PreviewReserve")]
        PreviewReserve,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComposeResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manifest")]
        pub manifest: Option<crate::manifest::Manifest>,
        #[serde(rename = "RequestFormat")]
        pub request_format: crate::composition_service::v1_2_3::ComposeRequestFormat,
        #[serde(rename = "RequestType")]
        pub request_type: crate::composition_service::v1_2_3::ComposeRequestType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReservationId")]
        pub reservation_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::composition_service::v1_2_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActivePool")]
        pub active_pool: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowOverprovisioning"
        )]
        pub allow_overprovisioning: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowZoneAffinity")]
        pub allow_zone_affinity: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CompositionReservations"
        )]
        pub composition_reservations: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::composition_service::v1_2_3::CompositionServiceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FreePool")]
        pub free_pool: Option<crate::odata_v4::IdRef>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReservationDuration"
        )]
        pub reservation_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceZones")]
        pub resource_zones: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CompositionServiceDescription {
        V000001(crate::composition_service::v1_2_3::CompositionServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CompositionServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
