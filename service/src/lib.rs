// TODO: generate from schema
pub mod model;
pub mod response;
pub mod service;

use crate::response::Response;
use crate::service::Service;
use axum::{extract, http::header::HeaderMap, routing, Router};
use std::collections::HashMap;

pub fn routes(service: Box<impl Service>) -> Router {
    // TODO: AuthN, AuthZ
    // TODO: 7 Service requests
    // TODO: 8 Service responses
    Router::new()
        .route("/redfish/v1", routing::get(get_service_root))
        .route("/redfish/v1/", routing::get(get_service_root))
        .route(
            "/redfish/v1/SessionService",
            routing::get(get_session_service),
        )
        .route(
            "/redfish/v1/SessionService/Sessions",
            routing::get(get_session_service_sessions),
        )
        .route(
            "/redfish/v1/SessionService/Sessions",
            routing::post(post_session_service_sessions),
        )
        .route(
            "/redfish/v1/SessionService/Sessions/:session_id",
            routing::delete(delete_session_service_sessions_sessionid),
        )
        .route(
            "/redfish/v1/SessionService/Sessions/:session_id",
            routing::get(get_session_service_sessions_sessionid),
        )
        .with_state(service)
}

async fn get_service_root(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<
    Response<redfish_model::service_root::v1_16_0::ServiceRoot>,
    Response<redfish_model::RedfishError>,
> {
    redfish.get_service_root(queries, headers).await
}

async fn get_session_service(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<
    Response<redfish_model::session_service::v1_1_8::SessionService>,
    Response<redfish_model::RedfishError>,
> {
    redfish.get_session_service(queries, headers).await
}

async fn get_session_service_sessions(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<
    Response<redfish_model::session_collection::SessionCollection>,
    Response<redfish_model::RedfishError>,
> {
    redfish.get_session_service_sessions(queries, headers).await
}

async fn post_session_service_sessions(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
    payload: Result<
        extract::Json<redfish_model::session::v1_6_0::Session>,
        extract::rejection::JsonRejection,
    >,
) -> Result<
    Response<model::PostSessionServiceSessionsResponse>,
    Response<redfish_model::RedfishError>,
> {
    match payload {
        Ok(extract::Json(payload)) => {
            redfish
                .post_session_service_sessions(queries, headers, payload)
                .await
        }
        Err(err) => match err {
            extract::rejection::JsonRejection::JsonDataError(_) => Err(Response::malformed_json()),
            extract::rejection::JsonRejection::JsonSyntaxError(_) => Err(Response::invalid_json(0)),
            extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                Err(Response::general_error_client())
            }
            _ => Err(Response::general_error_server()),
        },
    }
}

async fn delete_session_service_sessions_sessionid(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Path(session_id): extract::Path<String>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<
    Response<model::DeleteSessionServiceSessionsSessionIdResponse>,
    Response<redfish_model::RedfishError>,
> {
    redfish
        .delete_session_service_sessions_sessionid(session_id, queries, headers)
        .await
}

async fn get_session_service_sessions_sessionid(
    extract::State(redfish): extract::State<Box<impl Service>>,
    extract::Path(session_id): extract::Path<String>,
    extract::Query(queries): extract::Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response<redfish_model::session::v1_6_0::Session>, Response<redfish_model::RedfishError>>
{
    redfish
        .get_session_service_sessions_sessionid(session_id, queries, headers)
        .await
}
