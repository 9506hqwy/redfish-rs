// TODO: generate from schema
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PostSessionServiceSessionsResponse {
    R201(redfish_model::session::v1_6_0::Session),
    R202(redfish_model::task::v1_7_1::Task),
    R204,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DeleteSessionServiceSessionsSessionIdResponse {
    R200(redfish_model::session::v1_6_0::Session),
    R202(redfish_model::task::v1_7_1::Task),
    R204,
}
