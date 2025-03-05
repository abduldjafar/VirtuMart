use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use std::string::FromUtf8Error;

use serde::Serialize;
use serde_json::json;

use argon2::password_hash::Error as ArgonHashError;
use base64::DecodeError;
use jsonwebtoken::errors::Error as JsonWebTokenError;
use redis::RedisError;
use surrealdb::Error as SurrealError;
use uuid::Error as UuidError;

use lettre::transport::smtp::Error as LettreError;

use google_cloud_storage::{
    client::google_cloud_auth::error::Error as GoogleCloudAuthError,
    http::Error as GoogleCloudStorageError,
};

/// Custom result type using the `Error` enum.
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    DatabaseErrorExecution(String),
    DataExist(String),
    DataNotAvaliable(String),
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
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[macro_export]
macro_rules! impl_error {
    ($($from:ty => $error_enum:ident),* $(,)?) => {
        $(
            impl From<$from> for Error {
                fn from(error: $from) -> Self {
                    Error::$error_enum(error.to_string())
                }
            }
        )*
    };
}

impl std::error::Error for Error {}

impl_error!(
    SurrealError => DatabaseErrorExecution,
    JsonWebTokenError => TokenError,
    DecodeError => DecodeError,
    FromUtf8Error => StringError,
    RedisError => DatabaseErrorExecution,
    UuidError => StringError,
    ArgonHashError => DatabaseErrorExecution,
    LettreError => SmtpProcessingError,
    GoogleCloudStorageError => UploadProcessingError,
    GoogleCloudAuthError => CloudAuthError,
);

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match &self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed".to_string()),

            Error::DatabaseErrorExecution(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),

            Error::DataExist(id) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("{} already registered", id),
            ),

            Error::DataNotAvaliable(id) => (StatusCode::NOT_FOUND, format!("{} Not Available", id)),

            Error::TokenError(msg)
            | Error::DecodeError(msg)
            | Error::StringError(msg)
            | Error::UserUnauthorized(msg)
            | Error::InvalidUserRole(msg) => (StatusCode::FORBIDDEN, msg.to_string()),

            Error::SmtpProcessingError(msg)
            | Error::UploadProcessingError(msg)
            | Error::CloudAuthError(msg)
            | Error::TcpErrorConnection(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
            }

            Error::UserNotVerified(msg) | Error::UnsupportedEngine(msg) => {
                (StatusCode::NOT_ACCEPTABLE, msg.to_string())
            }
        };

        let body = Body::from(json!({ "status": "failed", "error": error_message }).to_string());

        let mut response = Response::new(body);
        *response.status_mut() = status;
        response
    }
}
