use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use osl_db::error::StorageError;
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum WebError {
    Storage(StorageError),
    BadRequest(String),
    NotFound,
    InternalServerError(String),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(e) => write!(f, "Storage error: {e}"),
            Self::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            Self::NotFound => write!(f, "Resource not found"),
            Self::InternalServerError(msg) => write!(f, "Internal server error: {msg}"),
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Storage(StorageError::NotFound) | Self::NotFound => StatusCode::NOT_FOUND,
            Self::Storage(_) | Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        let body = match &self {
            Self::Storage(StorageError::NotFound) | Self::NotFound => {
                json!({"error": "Resource not found"})
            }
            Self::Storage(e) => {
                tracing::error!(error = ?e, "Request failed during a database operation");
                json!({"error": "An internal error occurred"})
            }
            Self::BadRequest(msg) => json!({"error": msg}),
            Self::InternalServerError(msg) => {
                tracing::error!(error = %msg, "Request failed with an internal error");
                json!({"error": "An internal error occurred"})
            }
        };

        (status, Json(body)).into_response()
    }
}

impl From<StorageError> for WebError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

pub type WebResult<T> = Result<T, WebError>;
