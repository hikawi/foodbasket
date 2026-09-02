use axum::{
    Router,
    routing::{get, post},
};
use http::StatusCode;

use crate::app::AppState;

pub mod handler;

#[derive(thiserror::Error, Debug)]
pub enum SseError {
    // #[error("BINDING_FAILED")]
    // BindingFailed(String),

    // #[error("VALIDATION_FAILED")]
    // ValidationFailed(String),

    // #[error("BAD_CONTEXT")]
    // BadContext(String),
    #[error("UNAUTHORIZED")]
    Unauthorized(String),

    #[error("INTERNAL_SERVER_ERROR")]
    Internal(String),
}

impl SseError {
    pub fn extract(self) -> (StatusCode, String, String) {
        let code = self.to_string();
        match self {
            // Self::BindingFailed(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            // Self::ValidationFailed(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            // Self::BadContext(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::Unauthorized(s) => (StatusCode::UNAUTHORIZED, code.clone(), s),
            Self::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                code.clone(),
                e.to_string(),
            ),
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::sse_handler))
        .route("/ping", post(handler::sse_ping))
}
