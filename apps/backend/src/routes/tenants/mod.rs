use axum::{
    Router,
    extract::rejection::{JsonRejection, QueryRejection},
    routing::{get, post},
};
use http::StatusCode;
use validator::ValidationErrors;

use crate::{app::AppState, services::TenantServiceError};

mod dtos;
pub mod handler;

#[derive(thiserror::Error, Debug)]
pub enum TenantError {
    #[error("BINDING_FAILED")]
    Binding(String),

    #[error("VALIDATION_FAILED")]
    Validation(String),

    #[error("UNAUTHORIZED")]
    Unauthorized(String),

    #[error("SLUG_TAKEN")]
    SlugTaken(String),

    #[error("SLUG_FORBIDDEN")]
    SlugForbidden(String),

    #[error("INTERNAL_SERVER_ERROR")]
    InternalServer(#[from] TenantServiceError),
}

impl From<QueryRejection> for TenantError {
    fn from(value: QueryRejection) -> Self {
        Self::Binding(value.to_string())
    }
}

impl From<JsonRejection> for TenantError {
    fn from(value: JsonRejection) -> Self {
        Self::Binding(value.to_string())
    }
}

impl From<ValidationErrors> for TenantError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value.to_string())
    }
}

impl TenantError {
    pub fn extract(self) -> (StatusCode, String, String) {
        let code = self.to_string();
        match self {
            Self::Binding(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::Validation(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::Unauthorized(s) => (StatusCode::UNAUTHORIZED, code.clone(), s),
            Self::SlugTaken(s) => (StatusCode::CONFLICT, code.clone(), s),
            Self::SlugForbidden(s) => (StatusCode::FORBIDDEN, code.clone(), s),
            Self::InternalServer(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                code.clone(),
                e.to_string(),
            ),
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::get_tenants))
        .route("/", post(handler::create_tenant))
}
