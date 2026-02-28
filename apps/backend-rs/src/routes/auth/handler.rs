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
    app::AppState,
    error::AppError,
    routes::auth::{
        AuthError,
        dtos::{GetMeResponse, PostLoginRequest, PostRegisterRequest},
    },
    services::{self, sessions::Session},
};

const SESSION_COOKIE_NAME: &'static str = "session_id";

pub async fn login(
    cookies: Cookies,
    State(state): State<AppState>,
    body: Result<Json<PostLoginRequest>, JsonRejection>,
) -> Result<Json<MessageResponse>, AppError> {
    let Json(body) = body.map_err(|e| AuthError::ValidationFailed(e.body_text()))?;

    body.validate()
        .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

    let user = services::users::check_user_credentials(&state.db, &body.email, &body.password)
        .await
        .map_err(AuthError::from)?;

    let session = Session {
        user_id: Some(user.id),
        user_email: Some(user.email),
        created_at: Utc::now(),
    };
    let session_id = services::sessions::create(&state.cache, &session)
        .await
        .map_err(AuthError::from)?;

    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_domain(state.config.cookie_domain.clone());
    cookie.set_secure(state.config.cookie_secure);
    cookies.add(cookie);

    Ok(Json(MessageResponse {
        message: "ok".into(),
    }))
}

pub async fn register(
    cookies: Cookies,
    State(state): State<AppState>,
    body: Result<Json<PostRegisterRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<MessageResponse>), AppError> {
    let Json(body) = body.map_err(|e| AuthError::ValidationFailed(e.body_text()))?;

    body.validate()
        .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

    let user =
        services::users::register_user(&state.db, &body.name, &body.email, Some(&body.password))
            .await
            .map_err(AuthError::from)?;

    let session = Session {
        user_id: Some(user.id),
        user_email: Some(user.email.clone()),
        created_at: Utc::now(),
    };
    let session_id = services::sessions::create(&state.cache, &session)
        .await
        .map_err(AuthError::from)?;

    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_domain(state.config.cookie_domain.clone());
    cookie.set_secure(state.config.cookie_secure);
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
    State(state): State<AppState>,
) -> Result<Json<MessageResponse>, AppError> {
    let sess_id = cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string()) // Map to String so we don't hold a reference
        .ok_or(AuthError::Unauthenticated)?;

    services::sessions::delete(&state.cache, &sess_id)
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
    State(state): State<AppState>,
) -> Result<Json<GetMeResponse>, AppError> {
    let sess_id = cookies
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string()) // Map to String so we don't hold a reference
        .ok_or(AuthError::Unauthenticated)?;

    let session = services::sessions::get(&state.cache, &sess_id)
        .await
        .map_err(|_| AuthError::Unauthenticated)?;

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
