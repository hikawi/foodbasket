use axum::Json;

use crate::api::responses::{ErrorResponse, MessageResponse};

/// Checks if the server is healthy.
#[utoipa::path(get, path = "/health", tags = ["health"], responses(
        (status = 200, description = "Server is healthy", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
))]
pub async fn health_check() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "healthy".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing};
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", routing::get(health_check));
        let server = TestServer::new(app);

        let res = server.get("/health").await;

        res.assert_status_ok();
        res.assert_json(&json!({ "message": "healthy" }));
    }
}
