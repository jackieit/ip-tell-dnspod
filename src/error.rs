use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::Error as ReqwestError;
use serde_json::json;
use serde_json::Error as JsonError;
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error as Utf8Error;
use std::time::SystemTimeError as TimeError;

#[derive(Debug)]
pub struct ItdError(String, String);

impl ItdError {
    pub fn new(kind: String, message: String) -> ItdError {
        ItdError(kind, message)
    }
}
impl fmt::Display for ItdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IpTellDnspod Error: From {}, {}", self.0, self.1)
    }
}
impl std::error::Error for ItdError {}
impl From<IoError> for ItdError {
    fn from(err: IoError) -> Self {
        ItdError::new("StdIo".to_string(), err.to_string())
    }
}
impl From<Utf8Error> for ItdError {
    fn from(err: Utf8Error) -> Self {
        ItdError::new("FromUtf8".to_string(), err.to_string())
    }
}
impl From<TimeError> for ItdError {
    fn from(err: TimeError) -> Self {
        ItdError::new("SystemTime".to_string(), err.to_string())
    }
}
impl From<JsonError> for ItdError {
    fn from(err: JsonError) -> Self {
        ItdError::new("JsonConvert".to_string(), err.to_string())
    }
}
impl From<ReqwestError> for ItdError {
    fn from(err: ReqwestError) -> Self {
        ItdError::new("Reqwest".to_string(), err.to_string())
    }
}
impl From<argon2::password_hash::errors::Error> for ItdError {
    fn from(err: argon2::password_hash::errors::Error) -> Self {
        ItdError::new("PasswordHash".to_string(), err.to_string())
    }
}
impl From<jsonwebtoken::errors::Error> for ItdError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ItdError::new("Webtoken".to_string(), err.to_string())
    }
}
impl From<base64::DecodeError> for ItdError {
    fn from(err: base64::DecodeError) -> Self {
        ItdError::new("Base64Decode".to_string(), err.to_string())
    }
}
impl From<sqlx::Error> for ItdError {
    fn from(err: sqlx::Error) -> Self {
        ItdError::new("DbError".to_string(), err.to_string())
    }
}
impl From<std::net::AddrParseError> for ItdError {
    fn from(err: std::net::AddrParseError) -> Self {
        ItdError::new("AddrParse".to_string(), err.to_string())
    }
}
impl From<validator::ValidationErrors> for ItdError {
    fn from(err: validator::ValidationErrors) -> Self {
        ItdError::new("RequestValidation".to_string(), err.to_string())
    }
}
impl From<axum::extract::rejection::JsonRejection> for ItdError {
    fn from(err: axum::extract::rejection::JsonRejection) -> Self {
        ItdError::new("JsonRejection".to_string(), err.to_string())
    }
}
impl From<AuthenticateError> for ItdError {
    fn from(err: AuthenticateError) -> Self {
        ItdError::new("AuthError".to_string(), err.to_string())
    }
}

#[derive(Debug)]
pub enum AuthenticateError {
    WrongCredentials,
    UserNotExists,
    OperationForbidden,
}
impl fmt::Display for AuthenticateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthenticateError::WrongCredentials => {
                write!(f, "WrongCredentials")
            }
            AuthenticateError::UserNotExists => {
                write!(f, "UserNotExists")
            }
            AuthenticateError::OperationForbidden => {
                write!(f, "OperationForbidden")
            }
        }
    }
}

impl IntoResponse for ItdError {
    fn into_response(self) -> Response {
        let (status_code, code, message) = match self.0.as_str() {
            "AuthError" => match self.1.as_str() {
                "WrongCredentials" => (StatusCode::UNAUTHORIZED, 4010, "错误认证信息"),
                "UserNotExists" => (StatusCode::UNAUTHORIZED, 4011, "用户不存在"),
                "OperationForbidden" => (StatusCode::FORBIDDEN, 4030, "禁止访问"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, 5000, self.1.as_str()),
            },
            "RequestValidation" => (StatusCode::UNPROCESSABLE_ENTITY, 4220, self.1.as_str()),
            "ManError" => (StatusCode::UNPROCESSABLE_ENTITY, 4221, self.1.as_str()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, 5000, self.1.as_str()),
        };
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}
pub type ItdResult<T> = Result<T, ItdError>;

#[macro_export]
macro_rules! err {
    ( $msg:expr) => {
        Err(crate::error::ItdError::new(
            "__ERROR__".to_string(),
            $msg.to_string(),
        ))
    };
}
#[macro_export]
macro_rules! kerr {
    ( $kind:expr,$msg:expr) => {
        Err(crate::error::ItdError::new(
            $kind.to_string(),
            $msg.to_string(),
        ))
    };
}
