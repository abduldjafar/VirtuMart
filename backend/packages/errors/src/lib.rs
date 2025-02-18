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
    DataNotValidate(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        log_error!("{}", error);
        Error::DatabaseErrorExecution(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        log_error!("{}", error);
        Error::TokenError(error.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        log_error!("{}", error);
        Error::DecodeError(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        log_error!("{}", error);
        Error::StringError(error.to_string())
    }
}

impl From<RedisError> for Error {
    fn from(error: RedisError) -> Self {
        log_error!("{}", error);
        Error::DatabaseErrorExecution(error.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Self {
        log_error!("{}", error);
        Error::StringError(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        log_error!("{}", error);
        Error::DatabaseErrorExecution(error.to_string())
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        log_error!("{}", error);
        Error::SmtpProcessingError(error.to_string())
    }
}

impl From<google_cloud_storage::http::Error> for Error {
    fn from(error: google_cloud_storage::http::Error) -> Self {
        log_error!("{}", error);
        Error::UploadProcessingError(error.to_string())
    }
}

impl From<google_cloud_storage::client::google_cloud_auth::error::Error> for Error {
    fn from(error: google_cloud_storage::client::google_cloud_auth::error::Error) -> Self {
        log_error!("{}", error);
        Error::CloudAuthError(error.to_string())
    }
}

impl From<ValidationErrors> for Error {
    fn from(error: ValidationErrors) -> Self {
        log_error!("{}", error);
        Error::DataNotValidate(error.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match &self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed".to_string()),
            Error::DatabaseErrorExecution(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("There was a problem with the database: {}", error),
            ),
            Error::DataExist(id) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("{} already registered", id),
            ),
            Error::DataNotAvaliable(id) => (StatusCode::NOT_FOUND, format!("{} Not Available", id)),
            Error::TokenError(message) => (StatusCode::UNAUTHORIZED, message.to_string()),
            Error::DecodeError(message) => (StatusCode::FORBIDDEN, message.to_string()),
            Error::StringError(message) => (StatusCode::FORBIDDEN, message.to_string()),
            Error::UserUnauthorized(message) => (StatusCode::UNAUTHORIZED, message.to_string()),
            Error::SmtpProcessingError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            Error::UserNotVerified(message) => (StatusCode::NOT_ACCEPTABLE, message.to_string()),
            Error::UploadProcessingError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            Error::CloudAuthError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            Error::InvalidUserRole(message) => (StatusCode::FORBIDDEN, message.to_string()),
            Error::UnsupportedEngine(message) => (StatusCode::NOT_ACCEPTABLE, message.to_string()),
            Error::TcpErrorConnection(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            Error::DataDuplicationError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            Error::DataNotValidate(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
        };

        let body = Body::from(
            json!({
                "status": "failed",
                "error": error_message
            })
            .to_string(),
        );

        let mut response = Response::new(body);
        *response.status_mut() = status;
        response
    }
}
