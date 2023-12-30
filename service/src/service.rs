// TODO: generate from schema
use crate::model;
use crate::response::Response;
use axum::{async_trait, http::header::HeaderMap};
use std::collections::HashMap;

#[allow(unused_variables)]
#[async_trait]
pub trait Service: Clone + Send + Sync + 'static {
    async fn get_service_root(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::service_root::v1_16_0::ServiceRoot>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn get_session_service(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_service::v1_1_8::SessionService>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn get_session_service_sessions(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_collection::SessionCollection>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn post_session_service_sessions(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
        payload: redfish_model::session::v1_6_0::Session,
    ) -> Result<
        Response<model::PostSessionServiceSessionsResponse>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn delete_session_service_sessions_sessionid(
        self,
        session_id: String,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<model::DeleteSessionServiceSessionsSessionIdResponse>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn get_session_service_sessions_sessionid(
        self,
        session_id: String,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session::v1_6_0::Session>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }
}
