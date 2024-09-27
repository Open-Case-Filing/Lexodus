use http::status::StatusCode;
use cfg_if::cfg_if;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize, EnumString)]
pub enum LexodusAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Auth Error")]
    AuthError,
    #[error("DB Connection Not Found")]
    DBConnectionNotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Server Error: {0}")]
    ServerError(String),
    #[error("TomlError: {0}")]
    TomlError(String),
    #[error("Argon2Error: {0}")]
    Argon2Error(String),
    #[error("CompilationError: {0}")]
    CompilationError(String),
    #[error("SessionError: {0}")]
    SessionError(String),
    #[error("JsonError: {0}")]
    JsonError(String),
    #[error("DBError: {0}")]
    DBError(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("Missing or Invalid Frontmatter")]
    MissingOrInvalidFrontmatter,
    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl LexodusAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            LexodusAppError::NotFound => StatusCode::NOT_FOUND,
            LexodusAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::Argon2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::CompilationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::SessionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::JsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::DBConnectionNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            LexodusAppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            LexodusAppError::AuthError => StatusCode::BAD_REQUEST,
            LexodusAppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            LexodusAppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
impl From<serde_wasm_bindgen::Error> for LexodusAppError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        Self::CompilationError(error.to_string())
    }
}
}
}
//impl From<ServerFnError> for ServerFnError<LexodusAppError> {
//    fn from(err: LexodusAppError) -> Self {
//        server_fn_error!(err)
//    }
//}
//
cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl From<argon2::password_hash::Error> for LexodusAppError {
            fn from(error: argon2::password_hash::Error) -> Self {
                Self::Argon2Error(error.to_string())
            }
        }
        impl From<spin_sdk::sqlite::Error> for LexodusAppError{
            fn from(error: spin_sdk::sqlite::Error) -> Self {
                Self::DBError(error.to_string())
            }
        }
        impl From<async_session::Error> for LexodusAppError{
            fn from(error: async_session::Error) -> Self {
                Self::SessionError(error.to_string())
            }
        }
        impl From<serde_json::Error> for LexodusAppError{
            fn from(error: serde_json::Error) -> Self {
                Self::JsonError(error.to_string())
            }
        }
    }
}
