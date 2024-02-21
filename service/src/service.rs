// TODO: generate from schema
use crate::model;
use crate::response::Response;
use axum::{async_trait, http::header::HeaderMap};
use std::collections::HashMap;
use std::sync::Arc;

#[allow(unused_variables)]
#[async_trait]
pub trait Service: Send + Sync + 'static {
    async fn get(
        &self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::service_root::v1_16_0::ServiceRoot>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    fn session_service(&self) -> Option<Arc<dyn SessionService>> {
        None
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait SessionService: Send + Sync + 'static {
    async fn get(
        &self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_service::v1_1_8::SessionService>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    fn sessions(&self) -> Option<Arc<dyn SessionServiceSessions>> {
        None
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait SessionServiceSessions: Send + Sync + 'static {
    async fn get(
        &self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_collection::SessionCollection>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn post(
        &self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
        payload: redfish_model::session::v1_6_0::Session,
    ) -> Result<
        Response<model::PostSessionServiceSessionsResponse>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    fn session_id(&self) -> Option<Arc<dyn SessionServiceSessionsSessionId>> {
        None
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait SessionServiceSessionsSessionId: Send + Sync + 'static {
    async fn delete(
        &self,
        session_id: String,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<model::DeleteSessionServiceSessionsSessionIdResponse>,
        Response<redfish_model::RedfishError>,
    > {
        Err(Response::operation_not_allowed())
    }

    async fn get(
        &self,
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
