use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// Basic response for just sending a status message, such as OK or healthy
/// that doesn't have any additional data from the body.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    /// The message to return.
    pub message: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    pub message: String,
}

/// Basic generic typed response for sending a paginated response.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub total_pages: i64,
    pub page: i64,
    pub per_page: i64,
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

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            data,
            total,
            total_pages,
            page,
            per_page,
        }
    }
}
