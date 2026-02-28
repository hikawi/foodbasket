use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, code: &str, message: &str) -> Self {
        Self {
            status: status.as_u16(),
            code: code.into(),
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        if let Ok(status) = StatusCode::from_u16(self.status) {
            (status, Json(self)).into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    "Failed to serialize error response",
                ),
            )
                .into_response()
        }
    }
}
