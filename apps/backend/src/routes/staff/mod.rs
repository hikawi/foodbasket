use axum::{
    Router,
    extract::rejection::{JsonRejection, QueryRejection},
    routing::*,
};
use http::StatusCode;
use validator::ValidationErrors;

use crate::app::AppState;

mod dto;
pub mod handler;

#[derive(thiserror::Error, Debug)]
pub enum StaffError {
    #[error("BINDING_FAILED")]
    BindingFailed(String),

    #[error("VALIDATION_FAILED")]
    ValidationFailed(String),

    #[error("BAD_CONTEXT")]
    BadContext(String),

    #[error("UNAUTHORIZED")]
    Unauthorized(String),

    #[error("INTERNAL_SERVER_ERROR")]
    Internal(String),
}

impl StaffError {
    pub fn extract(self) -> (StatusCode, String, String) {
        let code = self.to_string();
        match self {
            Self::BindingFailed(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::ValidationFailed(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::BadContext(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::Unauthorized(s) => (StatusCode::UNAUTHORIZED, code.clone(), s),
            Self::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                code.clone(),
                e.to_string(),
            ),
        }
    }
}

impl From<QueryRejection> for StaffError {
    fn from(value: QueryRejection) -> Self {
        Self::BindingFailed(value.to_string())
    }
}

impl From<JsonRejection> for StaffError {
    fn from(value: JsonRejection) -> Self {
        Self::BindingFailed(value.to_string())
    }
}

impl From<ValidationErrors> for StaffError {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidationFailed(value.to_string())
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler::get_staff))
}
