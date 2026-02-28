use axum::{http::StatusCode, response::IntoResponse};

use crate::{api::responses::ErrorResponse, routes::auth::AuthError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::Auth(e) => e.extract(),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        tracing::error!(status = %status.as_u16(), message);
        ErrorResponse::new(status, &message).into_response()
    }
}
