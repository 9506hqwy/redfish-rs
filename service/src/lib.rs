// TODO: generate from schema
pub mod model;
pub mod response;
pub mod service;

use crate::response::Response;
use crate::service::{
    Service, SessionService, SessionServiceSessions, SessionServiceSessionsSessionId,
};
use axum::{Router, extract, http::header::HeaderMap, routing};
use std::collections::HashMap;
use std::sync::Arc;

pub fn routes(state: Arc<dyn Service>) -> Router {
    async fn get(
        extract::State(redfish): extract::State<Arc<dyn Service>>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::service_root::v1_17_0::ServiceRoot>,
        Response<redfish_model::RedfishError>,
    > {
        redfish.get(queries, headers).await
    }

    // TODO: AuthZ
    // TODO: 7 Service requests
    // TODO: 8 Service responses
    let mut router = Router::new()
        .route("/redfish/v1", routing::get(get))
        .route("/redfish/v1/", routing::get(get));

    if let Some(session_service) = state.session_service() {
        router = router.nest(
            "/redfish/v1/SessionService",
            session_service_router(session_service),
        );
    }

    router.with_state(state)
}

fn session_service_router(state: Arc<dyn SessionService>) -> Router<Arc<dyn Service>> {
    async fn get(
        extract::State(redfish): extract::State<Arc<dyn SessionService>>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_service::v1_1_9::SessionService>,
        Response<redfish_model::RedfishError>,
    > {
        redfish.get(queries, headers).await
    }

    let mut router = Router::new().route("/", routing::get(get));

    if let Some(sessions) = state.sessions() {
        router = router.nest("/Sessions", session_service_sessions_router(sessions));
    }

    router.with_state(state)
}

fn session_service_sessions_router(
    state: Arc<dyn SessionServiceSessions>,
) -> Router<Arc<dyn SessionService>> {
    async fn get(
        extract::State(redfish): extract::State<Arc<dyn SessionServiceSessions>>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session_collection::SessionCollection>,
        Response<redfish_model::RedfishError>,
    > {
        redfish.get(queries, headers).await
    }

    async fn post(
        extract::State(redfish): extract::State<Arc<dyn SessionServiceSessions>>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
        payload: Result<
            extract::Json<redfish_model::session::v1_7_2::Session>,
            extract::rejection::JsonRejection,
        >,
    ) -> Result<
        Response<model::PostSessionServiceSessionsResponse>,
        Response<redfish_model::RedfishError>,
    > {
        match payload {
            Ok(extract::Json(payload)) => redfish.post(queries, headers, payload).await,
            Err(err) => match err {
                extract::rejection::JsonRejection::JsonDataError(_) => {
                    Err(Response::malformed_json())
                }
                extract::rejection::JsonRejection::JsonSyntaxError(_) => {
                    Err(Response::invalid_json(0))
                }
                extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                    Err(Response::general_error_client())
                }
                _ => Err(Response::general_error_server()),
            },
        }
    }

    let mut router = Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(post));

    if let Some(session_id) = state.session_id() {
        router = router.nest(
            "/:session_id",
            session_service_sessions_session_router(session_id),
        );
    }

    router.with_state(state)
}

fn session_service_sessions_session_router(
    state: Arc<dyn SessionServiceSessionsSessionId>,
) -> Router<Arc<dyn SessionServiceSessions>> {
    async fn delete(
        extract::State(redfish): extract::State<Arc<dyn SessionServiceSessionsSessionId>>,
        extract::Path(session_id): extract::Path<String>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> Result<
        Response<model::DeleteSessionServiceSessionsSessionIdResponse>,
        Response<redfish_model::RedfishError>,
    > {
        redfish.delete(session_id, queries, headers).await
    }

    async fn get(
        extract::State(redfish): extract::State<Arc<dyn SessionServiceSessionsSessionId>>,
        extract::Path(session_id): extract::Path<String>,
        extract::Query(queries): extract::Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> Result<
        Response<redfish_model::session::v1_7_2::Session>,
        Response<redfish_model::RedfishError>,
    > {
        redfish.get(session_id, queries, headers).await
    }

    Router::new()
        .route("/", routing::delete(delete))
        .route("/", routing::get(get))
        .with_state(state)
}
