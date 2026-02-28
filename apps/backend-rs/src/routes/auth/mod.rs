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
    #[error("Bad Request")]
    ValidationFailed(String),

    #[error("Wrong password")]
    WrongPassword,

    #[error("User not found")]
    UserNotFound,

    #[error("User does not use password")]
    UserDoesNotUsePassword,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User does not have an active session")]
    Unauthenticated,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl AuthError {
    pub fn extract(self) -> (StatusCode, String) {
        match self {
            Self::ValidationFailed(s) => (StatusCode::BAD_REQUEST, s),
            Self::WrongPassword => (StatusCode::FORBIDDEN, self.to_string()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::UserDoesNotUsePassword => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            Self::Unauthenticated => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".into(),
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
            UserServiceError::UserNotFound => AuthError::UserNotFound,
            UserServiceError::WrongPassword => AuthError::WrongPassword,
            UserServiceError::UserDoesNotUsePassword => AuthError::UserDoesNotUsePassword,
            UserServiceError::UserAlreadyExists => AuthError::UserAlreadyExists,
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
