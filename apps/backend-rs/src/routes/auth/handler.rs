use std::sync::Arc;

use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
};
use chrono::Utc;
use http::StatusCode;
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use validator::Validate;

use crate::{
    api::responses::MessageResponse,
    app::AppConfig,
    error::AppError,
    routes::auth::{
        AuthError,
        dtos::{GetMeResponse, PostLoginRequest, PostRegisterRequest},
    },
    services::{SessionService, UserService, sessions::Session},
};

const SESSION_COOKIE_NAME: &str = "session_id";

pub async fn login(
    cookies: Cookies,
    State(cfg): State<Arc<AppConfig>>,
    State(user_service): State<Arc<UserService>>,
    State(session_service): State<Arc<SessionService>>,
    body: Result<Json<PostLoginRequest>, JsonRejection>,
) -> Result<Json<MessageResponse>, AppError> {
    let Json(body) = body.map_err(|e| AuthError::ValidationFailed(e.body_text()))?;

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

pub async fn register(
    cookies: Cookies,
    State(cfg): State<Arc<AppConfig>>,
    State(user_service): State<Arc<UserService>>,
    State(session_service): State<Arc<SessionService>>,
    body: Result<Json<PostRegisterRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<MessageResponse>), AppError> {
    let Json(body) = body.map_err(|e| AuthError::ValidationFailed(e.body_text()))?;

    body.validate()
        .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

    let user = user_service
        .register_user(&body.name, &body.email, Some(&body.password))
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

pub async fn get_me(
    cookies: Cookies,
    State(session_service): State<Arc<SessionService>>,
) -> Result<Json<GetMeResponse>, AppError> {
    let sess_id = cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string()) // Map to String so we don't hold a reference
        .ok_or(AuthError::Unauthenticated(
            "Session cookie not found".into(),
        ))?;

    let session = session_service
        .get(&sess_id)
        .await
        .map_err(|_| AuthError::Unauthenticated("Session not found".into()))?;

    Ok(Json(GetMeResponse {
        id: session
            .user_id
            .ok_or(AuthError::Unknown(anyhow::anyhow!("No user name")))?
            .to_string(),
        email: session
            .user_email
            .ok_or(AuthError::Unknown(anyhow::anyhow!("No user email")))?,
    }))
}
