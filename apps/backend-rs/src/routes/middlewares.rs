use std::{collections::HashSet, sync::Arc};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{
    Method, StatusCode,
    header::{self, InvalidHeaderValue},
};
use tower_cookies::Cookies;

use crate::{
    error::AppError,
    routes::extract::{HostContext, PermissionsContext, SessionContext},
    services::{SessionService, TenantService, TenantServiceError},
};

#[derive(Debug, thiserror::Error)]
pub enum MiddlewareError {
    #[error("INVALID_ORIGIN")]
    InvalidOrigin,

    #[error("INVALID_HOST")]
    InvalidHost,

    #[error("SERVICE_UNAVAILABLE")]
    ServiceUnavailable,

    #[error("FAILED_TO_SET_HEADERS")]
    FailedToSetHeaders,

    #[error("UNKNOWN_TENANT")]
    UnknownTenant,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<InvalidHeaderValue> for MiddlewareError {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::FailedToSetHeaders
    }
}

impl From<TenantServiceError> for MiddlewareError {
    fn from(value: TenantServiceError) -> Self {
        Self::Unknown(anyhow::Error::new(value))
    }
}

impl MiddlewareError {
    pub fn extract(self) -> (StatusCode, String, String) {
        let code = self.to_string();
        match self {
            Self::InvalidOrigin => (
                StatusCode::BAD_REQUEST,
                code.clone(),
                "Malformed origin".into(),
            ),
            Self::InvalidHost => (
                StatusCode::BAD_REQUEST,
                code.clone(),
                "Malformed or non-existent host header".into(),
            ),
            Self::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                code.clone(),
                "That service is currently down".into(),
            ),
            Self::UnknownTenant => (
                StatusCode::NOT_FOUND,
                code.clone(),
                "That tenant isn't available".into(),
            ),
            Self::FailedToSetHeaders => (
                StatusCode::INTERNAL_SERVER_ERROR,
                code.clone(),
                "Failed to setup headers".into(),
            ),
            Self::Unknown(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR".into(),
                e.to_string(),
            ),
        }
    }
}

/// Applies a dynamic CORS middleware or layer that checks the Origin header at runtime to return
/// the correct CORS headers for browsers. This is necessary due to multi-tenancy and not knowing
/// what users might register their slugs as.
pub async fn dynamic_cors(req: Request, next: Next) -> Result<Response, AppError> {
    let method = req.method().clone();
    let origin_header = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    // Not a browser request. Ignore.
    if origin_header.is_none() {
        return Ok(next.run(req).await);
    }

    let origin = origin_header.unwrap();
    let url = url::Url::parse(&origin).map_err(|_| MiddlewareError::InvalidOrigin)?;
    let _host = url.host_str().ok_or(MiddlewareError::InvalidOrigin)?;

    // TODO
    // Check if the host is a valid tenant

    // If it is preflight
    if method == Method::OPTIONS {
        return Ok((
            StatusCode::NO_CONTENT,
            [
                (header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone()),
                (
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET,POST,PUT,PATCH,DELETE,OPTIONS".into(),
                ),
                (
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    "Content-Type, Authorization".into(),
                ),
                (header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".into()),
            ],
        )
            .into_response());
    }

    // Otherwise, just attach headers and move on.
    let mut res = next.run(req).await;

    // Attach headers to response.
    let headers = res.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        origin.parse().map_err(MiddlewareError::from)?,
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        "true".parse().map_err(MiddlewareError::from)?,
    );

    Ok(res)
}

/// Applies a layer that extracts the Host header out of the request, and check whether it's a
/// valid host in our infrastructure.
pub async fn host_hydrate(
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let mut host = req
        .headers()
        .get(header::HOST)
        .and_then(|s| s.to_str().ok())
        .map(&str::to_owned)
        .ok_or(MiddlewareError::InvalidHost)?;

    // If it is a forwarded host, override.
    if let Some(forwarded_host) = req
        .headers()
        .get(header::FORWARDED)
        .and_then(|s| s.to_str().ok())
    {
        host = forwarded_host.to_owned();
    }

    // Parse subdomain (assume format: tenant.[pos.]foodbasket.app)
    let parts: Vec<&str> = host.split(".").collect();

    req.extensions_mut()
        .insert(match (parts.as_slice(), parts.len()) {
            (_, len) if len > 4 => Err(MiddlewareError::ServiceUnavailable)?,
            (["pos", ..], 3) => HostContext::Pos,
            (["admin", ..], 3) => HostContext::Admin,
            ([slug, ..], 3) => HostContext::TenantHome(
                // Need to check if it's valid by slug.
                tenant_service
                    .get_id_by_slug(slug)
                    .await
                    .map_err(MiddlewareError::from)?
                    .ok_or(MiddlewareError::UnknownTenant)?,
            ),
            ([slug, "pos", ..], 4) => HostContext::TenantPos(
                tenant_service
                    .get_id_by_slug(slug)
                    .await
                    .map_err(MiddlewareError::from)?
                    .ok_or(MiddlewareError::UnknownTenant)?,
            ),
            _ => HostContext::Anonymous,
        });

    // We injected the host context.
    // We can now move forward.
    Ok(next.run(req).await)
}

/// Applies a layer that extracts SessionID cookie out of the header, and checking if is a good
/// session cookie. This merely hydrates the session. This does not check for permissions or fails
/// if there is no SessionID cookie.
pub async fn session_hydrate(
    cookies: Cookies,
    State(session_service): State<Arc<SessionService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let session_ctx = match cookies.get("session_id") {
        Some(cookie) => match session_service.get(cookie.value()).await {
            Ok(session) => SessionContext::Authenticated(Arc::new(session)),
            Err(_) => SessionContext::Anonymous,
        },
        None => SessionContext::Anonymous,
    };

    req.extensions_mut().insert(session_ctx);

    Ok(next.run(req).await)
}

/// Applies a layer that hydrates a list of permissions for the current SessionContext
/// and the current HostContext.
pub async fn permission_hydrate(mut req: Request, next: Next) -> Result<Response, AppError> {
    // TODO:
    // Convert this to a permission service calling.
    let mut placeholder_perms = HashSet::new();
    placeholder_perms.insert("test".to_owned());
    placeholder_perms.insert("perms".to_owned());

    let perm_context = match req.extensions().get::<SessionContext>() {
        Some(_) => PermissionsContext::Authenticated(Arc::new(placeholder_perms)),
        None => PermissionsContext::Anonymous,
    };

    req.extensions_mut().insert(perm_context);

    Ok(next.run(req).await)
}
