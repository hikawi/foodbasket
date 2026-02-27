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
    pub code: u16,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, error: &str) -> Self {
        Self {
            code: code.as_u16(),
            error: error.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        return (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response();
    }
}
