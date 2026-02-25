use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::Validate;

use crate::{
    app::AppState,
    models::dtos::{ErrorResponse, MessageResponse},
    routes::auth::dto::{LoginRequest, RegisterRequest},
    services::users::{self, UserError},
};

pub async fn login(State(state): State<Arc<AppState>>, Json(body): Json<LoginRequest>) -> Response {
    if let Err(e) = body.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response();
    }

    match users::check_user_credentials(&state.db, &body.email, &body.password).await {
        Ok(_) => (
            StatusCode::OK,
            Json(MessageResponse {
                message: "ok".into(),
            }),
        )
            .into_response(),
        Err(UserError::NotFound) => (StatusCode::NOT_FOUND, "User is not found").into_response(),
        Err(UserError::DoesNotHavePassword) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "User does not have password",
        )
            .into_response(),
        Err(UserError::WrongPassword) => (StatusCode::FORBIDDEN, "Wrong password").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown Error").into_response(),
    }
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Response {
    if let Err(e) = body.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response();
    }

    (StatusCode::OK, "test").into_response()
}
