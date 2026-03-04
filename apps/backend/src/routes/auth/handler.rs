use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{State, rejection::JsonRejection},
};
use chrono::Utc;
use http::StatusCode;
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use validator::Validate;

use crate::{
    api::responses::{ErrorResponse, MessageResponse},
    app::AppConfig,
    error::AppError,
    routes::{
        auth::{
            AuthError,
            dtos::{GetMeResponse, PostLoginRequest, PostRegisterRequest},
        },
        extract::SessionContext,
    },
    services::{Session, SessionService, UserService},
};

const SESSION_COOKIE_NAME: &str = "session_id";

/// Logins to an existing account.
///
/// This endpoint is only available to those using login with password method. For OAuth related
/// accounts, a separate endpoint should be used instead (currently OOS).
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    responses(
        (status = 200, description = "Successful login", body = MessageResponse, headers(("set-cookie" = String, description = "Sets a new cookie"))),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Wrong password", body = ErrorResponse),
        (status = 404, description = "User is not found", body = ErrorResponse),
        (status = 422, description = "User uses a different method of authentication", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn login(
    cookies: Cookies,
    State(cfg): State<Arc<AppConfig>>,
    State(user_service): State<Arc<UserService>>,
    State(session_service): State<Arc<SessionService>>,
    body: Result<Json<PostLoginRequest>, JsonRejection>,
) -> Result<Json<MessageResponse>, AppError> {
    let Json(body) = body.map_err(|e| AuthError::BindingFailed(e.body_text()))?;

    body.validate()
        .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

    let user = user_service
        .check_user_credentials(&body.email, &body.password)
        .await
        .map_err(AuthError::from)?;

    let session = Session {
        user_id: Some(user.id),
        user_email: Some(user.email.clone()),
        created_at: Utc::now(),
    };
    let session_id = session_service
        .create(&session)
        .await
        .map_err(AuthError::from)?;

    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_domain(cfg.cookie_domain.clone());
    cookie.set_secure(cfg.cookie_secure);
    cookies.add(cookie);

    tracing::info!(status = 200, email = %user.email);
    Ok(Json(MessageResponse {
        message: "ok".into(),
    }))
}

/// Registers an account with password.
///
/// This endpoint is only available to those using register with password method. For OAuth related
/// accounts, a separate endpoint should be used instead (currently OOS).
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    responses(
        (status = 201, description = "Successful register", body = MessageResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 409, description = "Email already in use", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn register(
    cookies: Cookies,
    State(cfg): State<Arc<AppConfig>>,
    State(user_service): State<Arc<UserService>>,
    State(session_service): State<Arc<SessionService>>,
    body: Result<Json<PostRegisterRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<MessageResponse>), AppError> {
    let Json(body) = body.map_err(|e| AuthError::BindingFailed(e.body_text()))?;

    body.validate()
        .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

    let user = user_service
        .register_user(&body.email, &body.password)
        .await
        .map_err(AuthError::from)?;

    let session = Session {
        user_id: Some(user.id),
        user_email: Some(user.email.clone()),
        created_at: Utc::now(),
    };
    let session_id = session_service
        .create(&session)
        .await
        .map_err(AuthError::from)?;

    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_domain(cfg.cookie_domain.clone());
    cookie.set_secure(cfg.cookie_secure);
    cookies.add(cookie);

    Ok((
        StatusCode::CREATED,
        Json(MessageResponse {
            message: "ok".into(),
        }),
    ))
}

/// Logouts the existing account and clears the session.
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Successful logout", body = MessageResponse),
        (status = 401, description = "No ssession cookie", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(("session_id" = [])),
)]
pub async fn logout(
    cookies: Cookies,
    State(session_service): State<Arc<SessionService>>,
) -> Result<Json<MessageResponse>, AppError> {
    let sess_id = cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string()) // Map to String so we don't hold a reference
        .ok_or(AuthError::Unauthenticated("Session not found".into()))?;

    session_service
        .delete(&sess_id)
        .await
        .map_err(AuthError::from)?;

    let mut cookie = Cookie::from(SESSION_COOKIE_NAME);
    cookie.set_path("/");
    cookies.remove(cookie);

    Ok(Json(MessageResponse {
        message: "ok".into(),
    }))
}

/// Retrieves the current logged in user.
///
/// This mainly applies for frontend to fetch the current context's profiles.
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Successful retrieval", body = GetMeResponse),
        (status = 401, description = "Unauthenticated", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(("session_id" = [])),
)]
pub async fn get_me(
    Extension(session): Extension<SessionContext>,
) -> Result<Json<GetMeResponse>, AppError> {
    match session {
        SessionContext::Authenticated(sess) => Ok(Json(GetMeResponse {
            id: sess
                .user_id
                .ok_or(AuthError::Unknown(anyhow::anyhow!("No user name")))?
                .to_string(),
            email: sess
                .user_email
                .clone()
                .ok_or(AuthError::Unknown(anyhow::anyhow!("No user email")))?
                .to_string(),
        })),
        SessionContext::Anonymous => Err(AuthError::Unauthenticated("Session not found".into()))?,
    }
}
