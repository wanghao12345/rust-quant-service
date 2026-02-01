use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    AuthorizationError(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    DbError(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceError::AuthorizationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ServiceError::DbError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        let body = json!({
            "code": status.as_u16() as i32,
            "msg": message,
            "data": null
        });
        
        (status, Json(body)).into_response()
    }
}


