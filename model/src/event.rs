use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EventType {
    #[default]
    #[serde(rename = "Alert")]
    Alert,
    #[serde(rename = "MetricReport")]
    MetricReport,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "ResourceAdded")]
    ResourceAdded,
    #[serde(rename = "ResourceRemoved")]
    ResourceRemoved,
    #[serde(rename = "ResourceUpdated")]
    ResourceUpdated,
    #[serde(rename = "StatusChange")]
    StatusChange,
}
