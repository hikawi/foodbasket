use axum::{Router, routing::get, routing::post};
use http::StatusCode;

use crate::{
    app::AppState,
    services::{sessions::SessionServiceError, users::UserServiceError},
};

mod dtos;
mod handler;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("VALIDATION_FAILED")]
    ValidationFailed(String),

    #[error("WRONG_PASSWORD")]
    WrongPassword(String),

    #[error("USER_NOT_FOUND")]
    UserNotFound(String),

    #[error("USER_NO_PASSWORD")]
    UserDoesNotUsePassword(String),

    #[error("USER_ALREADY_EXISTS")]
    UserAlreadyExists(String),

    #[error("UNAUTHENTICATED")]
    Unauthenticated(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl AuthError {
    pub fn extract(self) -> (StatusCode, String, String) {
        let code = self.to_string();
        match self {
            Self::ValidationFailed(s) => (StatusCode::BAD_REQUEST, code.clone(), s),
            Self::WrongPassword(s) => (StatusCode::FORBIDDEN, code.clone(), s),
            Self::UserNotFound(s) => (StatusCode::NOT_FOUND, code.clone(), s),
            Self::UserDoesNotUsePassword(s) => (StatusCode::UNPROCESSABLE_ENTITY, code.clone(), s),
            Self::UserAlreadyExists(s) => (StatusCode::CONFLICT, code.clone(), s),
            Self::Unauthenticated(s) => (StatusCode::UNAUTHORIZED, code.clone(), s),
            Self::Unknown(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR".into(),
                e.to_string(),
            ),
        }
    }
}

impl From<SessionServiceError> for AuthError {
    fn from(value: SessionServiceError) -> Self {
        match value {
            SessionServiceError::UnknownError(e) => AuthError::Unknown(e),
            _ => AuthError::Unknown(anyhow::anyhow!(value)),
        }
    }
}

impl From<UserServiceError> for AuthError {
    fn from(value: UserServiceError) -> Self {
        match value {
            UserServiceError::UserNotFound => AuthError::UserNotFound("User not found".into()),
            UserServiceError::WrongPassword => AuthError::WrongPassword("Wrong password".into()),
            UserServiceError::UserDoesNotUsePassword => {
                AuthError::UserDoesNotUsePassword("User does not use password".into())
            }
            UserServiceError::UserAlreadyExists => {
                AuthError::UserAlreadyExists("User already exists".into())
            }
            UserServiceError::UnknownError(e) => AuthError::Unknown(e),
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/register", post(handler::register))
        .route("/me", get(handler::get_me))
}
