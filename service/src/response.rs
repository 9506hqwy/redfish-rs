// TODO: generate from schema
use axum::{
    http::header::HeaderMap,
    http::status::{InvalidStatusCode, StatusCode},
    response::{self, IntoResponse},
};
use redfish_model::{RedfishError, RedfishErrorError};
use serde::Serialize;
use std::convert::From;

pub struct Response<T>
where
    T: Serialize,
{
    status: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl Response<RedfishError> {
    // TODO: DSP8011
    pub fn general_error_client() -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.GeneralError".to_string(),
                message: "A general error has occurred.  See Resolution for information on how to resolve the error, or @Message.ExtendedInfo if Resolution is not provided.".to_string(),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::BAD_REQUEST,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn general_error_server() -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.GeneralError".to_string(),
                message: "A general error has occurred.  See Resolution for information on how to resolve the error, or @Message.ExtendedInfo if Resolution is not provided.".to_string(),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn invalid_json(line: u32) -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.InvalidJSON".to_string(),
                message: format!("The request body submitted is invalid JSON starting at line {line} and could not be parsed by the receiving service."),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::BAD_REQUEST,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn invalid_uri(arg1: &str) -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.InvalidURI".to_string(),
                message: format!("The URI {arg1} was not found."),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn malformed_json() -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.MalformedJSON".to_string(),
                message: "The request body submitted was malformed JSON and could not be parsed by the receiving service.".to_string(),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn operation_not_allowed() -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.OperationNotAllowed".to_string(),
                message: "The HTTP method is not allowed on this resource.".to_string(),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::METHOD_NOT_ALLOWED,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn property_value_error(name: &str) -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.PropertyValueError".to_string(),
                message: format!("The value provided for the property {name} is not valid."),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn resource_already_exists(ty: &str, name: &str, value: &str) -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.ResourceAlreadyExists".to_string(),
                message: format!("The requested resource of type {ty} with the property {name} with the value {value} already exists."),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::CONFLICT,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn resource_not_found(ty: &str, id: &str) -> Self {
        let body = RedfishError {
            error: RedfishErrorError {
                code: "Base.1.17.ResourceNotFound".to_string(),
                message: format!("The requested resource of type {ty} named '{id}' was not found."),
                message_extended_info: None,
            },
        };

        Response {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            body,
        }
    }
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn status_mut(&mut self, code: u16) -> Result<(), InvalidStatusCode> {
        self.status = StatusCode::from_u16(code)?;
        Ok(())
    }
}

impl<T> From<T> for Response<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        Response {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: value,
        }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> response::Response {
        (self.status, self.headers, response::Json(self.body)).into_response()
    }
}
