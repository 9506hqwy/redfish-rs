pub mod capacity;
pub mod capacity_source_collection;
pub mod class_of_service;
pub mod class_of_service_collection;
pub mod consistency_group;
pub mod consistency_group_collection;
pub mod data_protection_line_of_service;
pub mod data_protection_los_capabilities;
pub mod data_security_line_of_service;
pub mod data_security_los_capabilities;
pub mod data_storage_line_of_service;
pub mod data_storage_los_capabilities;
pub mod file_share;
pub mod file_share_collection;
pub mod file_system;
pub mod file_system_collection;
pub mod file_system_metrics;
pub mod hosted_storage_services;
pub mod io_connectivity_line_of_service;
pub mod io_connectivity_los_capabilities;
pub mod io_performance_line_of_service;
pub mod io_performance_los_capabilities;
pub mod io_statistics;
pub mod line_of_service;
pub mod line_of_service_collection;
pub mod nvme_domain;
pub mod nvme_domain_collection;
pub mod nvme_firmware_image;
pub mod storage_group;
pub mod storage_group_collection;
pub mod storage_pool;
pub mod storage_pool_collection;
pub mod storage_pool_metrics;
pub mod storage_replica_info;
pub mod storage_service;
pub mod storage_service_collection;
pub mod storage_service_metrics;
pub mod storage_system_collection;
pub mod volume;
pub mod volume_collection;
pub mod volume_metrics;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetOdata200Response {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "value")]
    pub value: Vec<crate::swordfish::GetOdata200ResponseValue>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetOdata200ResponseValue {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedfishError {
    #[serde(rename = "error")]
    pub error: crate::swordfish::RedfishErrorError,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedfishErrorError {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "@Message.ExtendedInfo"
    )]
    pub message_extended_info: Option<Vec<crate::message::v1_1_2::Message>>,
}
