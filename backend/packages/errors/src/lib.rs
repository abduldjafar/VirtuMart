use std::string::FromUtf8Error;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use redis::RedisError;
use serde::Serialize;
use serde_json::json;
use tracing::error as log_error;
use validator::ValidationErrors;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    DatabaseErrorExecution(String),
    DataDuplicationError(String),
    DataExist(String),
    DataNotAvailable(String),
    TokenError(String),
    DecodeError(String),
    StringError(String),
    UserUnauthorized(String),
    SmtpProcessingError(String),
    UserNotVerified(String),
    UploadProcessingError(String),
    CloudAuthError(String),
    InvalidUserRole(String),
    UnsupportedEngine(String),
    TcpErrorConnection(String),
    DataNotValidate(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Implement From<T> for common error conversions
macro_rules! impl_from_error {
    ($($type:ty => $variant:ident),* $(,)?) => {
        $(impl From<$type> for Error {
            fn from(error: $type) -> Self {
                log_error!("{}", error);
                Error::$variant(error.to_string())
            }
        })*
    };
}

impl_from_error!(
    surrealdb::Error => DatabaseErrorExecution,
    jsonwebtoken::errors::Error => TokenError,
    base64::DecodeError => DecodeError,
    FromUtf8Error => StringError,
    RedisError => DatabaseErrorExecution,
    uuid::Error => StringError,
    argon2::password_hash::Error => DatabaseErrorExecution,
    lettre::transport::smtp::Error => SmtpProcessingError,
    google_cloud_storage::http::Error => UploadProcessingError,
    google_cloud_storage::client::google_cloud_auth::error::Error => CloudAuthError,
    ValidationErrors => DataNotValidate
);

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match &self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed".to_string()),
            Error::DatabaseErrorExecution(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", error),
            ),
            Error::DataExist(id) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("{} already registered", id),
            ),
            Error::DataNotAvailable(message) => (StatusCode::NOT_FOUND, message.clone()),
            Error::TokenError(message) | Error::UserUnauthorized(message) => {
                (StatusCode::UNAUTHORIZED, message.clone())
            }
            Error::DecodeError(message)
            | Error::StringError(message)
            | Error::InvalidUserRole(message) => (StatusCode::FORBIDDEN, message.clone()),
            Error::SmtpProcessingError(message)
            | Error::UploadProcessingError(message)
            | Error::CloudAuthError(message)
            | Error::TcpErrorConnection(message)
            | Error::DataDuplicationError(message)
            | Error::DataNotValidate(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.clone())
            }
            Error::UserNotVerified(message) | Error::UnsupportedEngine(message) => {
                (StatusCode::NOT_ACCEPTABLE, message.clone())
            }
        };

        let body = Body::from(json!({ "status": "failed", "error": error_message }).to_string());

        let mut response = Response::new(body);
        *response.status_mut() = status;
        response
    }
}
