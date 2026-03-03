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
    routes::extract::{OriginContext, OriginUrl, PermissionsContext, SessionContext},
    services::{SessionService, TenantService, TenantServiceError},
};

#[derive(Debug, thiserror::Error)]
pub enum MiddlewareError {
    #[error("INVALID_ORIGIN")]
    InvalidOrigin,

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
pub async fn dynamic_cors(mut req: Request, next: Next) -> Result<Response, AppError> {
    let method = req.method().clone();
    let origin_header = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    // Not a browser request. Ignore.
    // Also inject an invalid origin.
    if origin_header.is_none() {
        req.extensions_mut().insert(OriginUrl::Invalid);
        return Ok(next.run(req).await);
    }

    let origin = origin_header.unwrap();
    let url = url::Url::parse(&origin).map_err(|_| MiddlewareError::InvalidOrigin)?;
    let _host = url.host_str().ok_or(MiddlewareError::InvalidOrigin)?;

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

    // Inject the origin, since we know it's a valid origin.
    req.extensions_mut()
        .insert(OriginUrl::Valid(url.origin().unicode_serialization()));

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

/// Applies a layer that extracts the Origin header out of the request, and check whether it's a
/// valid host in our infrastructure.
pub async fn origin_hydrate(
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let origin_url = req.extensions().get::<OriginUrl>();
    let origin = match origin_url {
        Some(OriginUrl::Valid(origin)) => origin.to_owned(),
        _ => {
            // Inject invalid hydration and ignore.
            req.extensions_mut().insert(OriginContext::Anonymous);
            return Ok(next.run(req).await);
        }
    };

    // Parse subdomain (assume format: tenant.[pos.]foodbasket.app)
    let parts: Vec<&str> = origin
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split(".")
        .collect();

    req.extensions_mut()
        .insert(match (parts.as_slice(), parts.len()) {
            (_, len) if len > 4 => Err(MiddlewareError::ServiceUnavailable)?,
            (["pos", ..], 3) => OriginContext::Pos,
            (["admin", ..], 3) => OriginContext::Admin,
            ([slug, ..], 3) => OriginContext::TenantHome(
                // Need to check if it's valid by slug.
                tenant_service
                    .get_id_by_slug(slug)
                    .await
                    .map_err(MiddlewareError::from)?
                    .ok_or(MiddlewareError::UnknownTenant)?,
            ),
            ([slug, "pos", ..], 4) => OriginContext::TenantPos(
                tenant_service
                    .get_id_by_slug(slug)
                    .await
                    .map_err(MiddlewareError::from)?
                    .ok_or(MiddlewareError::UnknownTenant)?,
            ),
            _ => OriginContext::Anonymous,
        });

    // We injected the origin context.
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
