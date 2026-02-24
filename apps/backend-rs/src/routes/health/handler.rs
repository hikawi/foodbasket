use axum::{Json, response::IntoResponse};

use crate::models::dtos::MessageResponse;

pub async fn health_check() -> impl IntoResponse {
    Json(MessageResponse {
        message: "ok".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{Value, json};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = axum::Router::new().route("/health", axum::routing::get(health_check));
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, json!({ "message": "ok" }));
    }
}
