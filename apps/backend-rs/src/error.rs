use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::api::responses::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unknown error: {0}")]
    UnknownError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UnknownError(e) => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
                    .into_response()
            }
        }
    }
}
