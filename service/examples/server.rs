use axum::{
    async_trait, extract,
    http::header::{HeaderMap, HeaderValue, CONTENT_TYPE, LOCATION},
    http::status::StatusCode,
    middleware,
    response::IntoResponse,
};
use redfish_model::{
    odata_v4::IdRef,
    service_root::v1_16_0::{Links, ServiceRoot},
    session::v1_6_0::Session,
    session_collection::SessionCollection,
    session_service::v1_1_8::SessionService,
    RedfishError,
};
use redfish_service::{model, response::Response, routes, service::Service};
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Box::<Redfish>::default();

    let app = routes(state).layer(middleware::from_fn(handle_error));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_error(req: extract::Request, next: middleware::Next) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    let res = next.run(req).await;

    let content_type = res.headers().get(CONTENT_TYPE);
    if let Some(ty) = content_type {
        if ty.to_str().unwrap() == "application/json" {
            return res;
        }
    }

    match res.status() {
        StatusCode::METHOD_NOT_ALLOWED => Response::operation_not_allowed().into_response(),
        StatusCode::NOT_FOUND => Response::invalid_uri(&path).into_response(),
        _ => res,
    }
}

#[derive(Clone)]
struct Redfish {
    service_root: ServiceRoot,
    session_service: SessionService,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

#[allow(unused_variables)]
#[async_trait]
impl Service for Redfish {
    async fn get_service_root(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<Response<ServiceRoot>, Response<RedfishError>> {
        Ok(Response::<ServiceRoot>::from(self.service_root))
    }

    async fn get_session_service(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<Response<SessionService>, Response<RedfishError>> {
        Ok(Response::<SessionService>::from(self.session_service))
    }

    async fn get_session_service_sessions(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<Response<SessionCollection>, Response<RedfishError>> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|_| Response::general_error_server())?;

        let ids = sessions
            .iter()
            .map(|s| IdRef {
                odata_id: Some(s.1.odata_id.clone()),
            })
            .collect();

        let m = session_service_sessions(ids);

        Ok(Response::<SessionCollection>::from(m))
    }

    async fn post_session_service_sessions(
        self,
        queries: HashMap<String, String>,
        headers: HeaderMap,
        mut payload: Session,
    ) -> Result<Response<model::PostSessionServiceSessionsResponse>, Response<RedfishError>> {
        // TODO: OpenAPI schema does not represent `requiredOnCreate` on JSON Schema.
        session_init(&mut payload)?;

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| Response::general_error_server())?;

        sessions.get(&payload.id).map_or(Ok(()), |s| {
            Err(Response::resource_already_exists(
                "#Session.v1_6_0.Session",
                "members",
                &s.id,
            ))
        })?;

        sessions.insert(payload.id.clone(), payload.clone());

        let path = payload.odata_id.clone();
        let m = model::PostSessionServiceSessionsResponse::R201(payload);

        let mut res = Response::<model::PostSessionServiceSessionsResponse>::from(m);
        res.status_mut(201).unwrap();
        res.headers_mut()
            .append(LOCATION, HeaderValue::from_str(&path).unwrap());

        Ok(res)
    }

    async fn delete_session_service_sessions_sessionid(
        self,
        session_id: String,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<
        Response<model::DeleteSessionServiceSessionsSessionIdResponse>,
        Response<RedfishError>,
    > {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| Response::general_error_server())?;

        sessions
            .get(&session_id)
            .ok_or_else(|| Response::resource_not_found("#Session.v1_6_0.Session", &session_id))?;

        let session = sessions.remove(&session_id).unwrap();

        let m = model::DeleteSessionServiceSessionsSessionIdResponse::R200(session);
        let res = Response::<model::DeleteSessionServiceSessionsSessionIdResponse>::from(m);
        Ok(res)
    }

    async fn get_session_service_sessions_sessionid(
        self,
        session_id: String,
        queries: HashMap<String, String>,
        headers: HeaderMap,
    ) -> Result<Response<Session>, Response<RedfishError>> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|_| Response::general_error_server())?;

        sessions
            .get(&session_id)
            .map(|s| Response::<Session>::from(s.clone()))
            .ok_or_else(|| Response::resource_not_found("#Session.v1_6_0.Session", &session_id))
    }
}

impl Default for Redfish {
    fn default() -> Self {
        let sessions = session_service_sessions(vec![]);
        let session_service = session_service(&sessions);
        let service_root = service_root(&session_service);

        Redfish {
            service_root,
            session_service,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn service_root(session_service: &SessionService) -> ServiceRoot {
    ServiceRoot {
        odata_id: "/redfish/v1".to_string(),
        odata_type: "#ServiceRoot.v1_16_0.ServiceRoot".to_string(),
        id: "0".to_string(),
        name: "root".to_string(),
        links: Links {
            sessions: IdRef {
                odata_id: Some(session_service.odata_id.clone()),
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn session_service(sessoins: &SessionCollection) -> SessionService {
    SessionService {
        odata_id: "/redfish/v1/SessionService".to_string(),
        odata_type: "#SessionService.v1_1_8.SessionService".to_string(),
        id: "0".to_string(),
        name: "SessionService".to_string(),
        sessions: Some(IdRef {
            odata_id: Some(sessoins.odata_id.clone()),
        }),
        ..Default::default()
    }
}

fn session_service_sessions(members: Vec<IdRef>) -> SessionCollection {
    let members_odata_count = members.len() as i64;
    SessionCollection {
        odata_id: "/redfish/v1/SessionService/Sessions".to_string(),
        odata_type: "#SessionCollection.SessionCollection".to_string(),
        name: "SessionCollection".to_string(),
        members,
        members_odata_count,
        ..Default::default()
    }
}

fn session_init(session: &mut Session) -> Result<(), Response<RedfishError>> {
    if session.id.is_empty() {
        return Err(Response::property_value_error("id"));
    }

    let id = &session.id;

    session.odata_id = format!("/redfish/v1/SessionService/Sessions/{id}");
    session.odata_type = "#Session.v1_6_0.Session".to_string();

    Ok(())
}
