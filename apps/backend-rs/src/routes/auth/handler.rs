use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::Validate;

use crate::{
    app::AppState,
    models::dtos::{ErrorResponse, MessageResponse},
    routes::auth::dto::{LoginRequest, RegisterRequest},
    services::{self, users::UserError},
};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("bad request")]
    BadRequest,
    #[error("user not found")]
    UserNotFound,
    #[error("user does not have password")]
    UserDoesNotHavePassword,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("incorrect password")]
    WrongPassword,
    #[error("unknown")]
    Unknown,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => {
                (StatusCode::BAD_REQUEST, ErrorResponse::json("bad request")).into_response()
            }
            Self::WrongPassword => {
                (StatusCode::FORBIDDEN, ErrorResponse::json("wrong password")).into_response()
            }
            Self::UserNotFound => {
                (StatusCode::NOT_FOUND, ErrorResponse::json("unknown user")).into_response()
            }
            Self::UserDoesNotHavePassword => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::json("does not use password"),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::json("internal server error"),
            )
                .into_response(),
        }
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<MessageResponse>, AuthError> {
    body.validate().map_err(|_| AuthError::BadRequest)?;

    services::users::check_user_credentials(&state.db, &body.email, &body.password)
        .await
        .map_err(|e| match e {
            UserError::NotFound => AuthError::UserNotFound,
            UserError::DoesNotHavePassword => AuthError::UserDoesNotHavePassword,
            UserError::WrongPassword => AuthError::WrongPassword,
            _ => AuthError::Unknown,
        })?;

    Ok(MessageResponse::json("ok"))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<MessageResponse>, AuthError> {
    body.validate().map_err(|_| AuthError::BadRequest)?;

    services::users::register_user(&state.db, &body.name, &body.email, &body.password)
        .await
        .map_err(|e| match e {
            UserError::AlreadyExists => AuthError::UserAlreadyExists,
            _ => AuthError::Unknown,
        })?;

    Ok(MessageResponse::json("yay"))
}
