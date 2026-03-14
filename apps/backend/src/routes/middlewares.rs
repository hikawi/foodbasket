use std::sync::Arc;

use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{
    Method, StatusCode,
    header::{self, InvalidHeaderValue},
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    error::AppError,
    routes::extract::{
        BranchContext, OriginContext, OriginUrl, PolicyContext, ProfileContext, RequestContext,
        SessionContext,
    },
    services::{PolicyService, ProfileService, SessionService, TenantService, TenantServiceError},
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
    let host = url.host_str().ok_or(MiddlewareError::InvalidOrigin)?;

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
                    "Content-Type, X-Branch-ID".into(),
                ),
                (header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".into()),
                (header::VARY, "Origin".into()),
            ],
        )
            .into_response());
    }

    // Inject the origin, since we know it's a valid origin.
    req.extensions_mut()
        .insert(OriginUrl::Valid(host.to_owned()));

    // Otherwise, just attach headers and move on.
    let mut res = next.run(req).await;

    // Attach headers to response.
    let headers = res.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        origin.parse().map_err(MiddlewareError::from)?,
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, X-Branch-ID".parse().unwrap(),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        "true".parse().map_err(MiddlewareError::from)?,
    );
    headers.insert(header::VARY, "Origin".parse().unwrap());

    Ok(res)
}

/// Applies a layer that extracts the Origin header out of the request, and check whether it's a
/// valid host in our infrastructure.
pub async fn origin_hydrate(
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let origin = match req.extensions().get::<OriginUrl>() {
        Some(OriginUrl::Valid(origin)) => origin.to_owned(),
        _ => {
            // Inject invalid hydration and ignore.
            req.extensions_mut().insert(OriginContext::Anonymous);
            return Ok(next.run(req).await);
        }
    };

    // Parse subdomain (assume format: https://tenant.[pos.]foodbasket.app)
    let parts: Vec<&str> = origin.split(".").collect();

    req.extensions_mut()
        .insert(match (parts.as_slice(), parts.len()) {
            (_, len) if len > 4 => Err(MiddlewareError::ServiceUnavailable)?,
            (["pos", "foodbasket", "app" | "localhost"], 3) => OriginContext::Pos,
            (["admin", "foodbasket", "app" | "localhost"], 3) => OriginContext::Admin,
            ([slug, "foodbasket", "app" | "localhost"], 3) => OriginContext::TenantHome(
                // Need to check if it's valid by slug.
                tenant_service
                    .get_id_by_slug(slug)
                    .await
                    .map_err(MiddlewareError::from)?
                    .ok_or(MiddlewareError::UnknownTenant)?,
            ),
            ([slug, "pos", "foodbasket", "app" | "localhost"], 4) => OriginContext::TenantPos(
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

/// Applies a layer that reads the extension for SessionContext. Depending on the user and the
/// context, reads the profile. There are 3 separations for best isolation, as staff_profiles,
/// customer_profiles, system_profiles are all separate, but link to the same user.
pub async fn profile_hydrate(
    Extension(origin): Extension<OriginContext>,
    Extension(session): Extension<SessionContext>,
    State(profile_service): State<Arc<ProfileService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Attempt to read the profile based on the session and origin.
    let sess = match session {
        SessionContext::Authenticated(sess) => sess,
        _ => {
            // If not authenticated. Why would we hydrate profiles?
            req.extensions_mut().insert(ProfileContext::Anonymous);
            return Ok(next.run(req).await);
        }
    };

    let ctx = match (origin, sess.user_id) {
        // If it is in tenant context and authenticated, parse as customer profiles.
        (OriginContext::TenantHome(tenant_id), Some(user_id)) => {
            match profile_service
                .get_customer_profile(&user_id, &tenant_id)
                .await
            {
                Ok(profile) => ProfileContext::Customer(Arc::new(profile)),
                _ => ProfileContext::Anonymous,
            }
        }
        // If it is in POS context and authenticated, parse as staff profiles.
        (OriginContext::TenantPos(tenant_id), Some(user_id)) => {
            match profile_service
                .get_staff_profile(&user_id, &tenant_id)
                .await
            {
                Ok(profile) => ProfileContext::Staff(Arc::new(profile)),
                _ => ProfileContext::Anonymous,
            }
        }
        // If it is in another context and authenticated, parse as system profiles.
        (_, Some(user_id)) => match profile_service.get_system_profile(&user_id).await {
            Ok(profile) => ProfileContext::System(Arc::new(profile)),
            _ => ProfileContext::Anonymous,
        },
        // Otherwise, that means user_id is probably None.
        _ => ProfileContext::Anonymous,
    };
    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

/// A layer to pull out the X-Branch-ID header for the current context of what branch are we
/// looking at. Basically just to not have the stupid query ?branch_id which is extremely ugly on
/// the frontend!
pub async fn branch_hydrate(
    Extension(origin_ctx): Extension<OriginContext>,
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let branch_id = req
        .headers()
        .get("X-Branch-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok());

    let branch_ctx = match (branch_id, origin_ctx) {
        (
            Some(branch_id),
            OriginContext::TenantPos(tenant_id) | OriginContext::TenantHome(tenant_id),
        ) => {
            match tenant_service
                .is_branch_of_tenant(&branch_id, &tenant_id)
                .await
            {
                Ok(true) => BranchContext::Branch(branch_id),
                _ => BranchContext::Anonymous,
            }
        }
        _ => BranchContext::Anonymous,
    };

    req.extensions_mut().insert(branch_ctx);
    Ok(next.run(req).await)
}

/// The innermost required layer to pull permissions in the form of policies. This will actually
/// run a calculate IF AND ONLY IF all other middlewares succeeded in querying or extracting the
/// context needed. Otherwise, this injects an empty PolicyContext.
pub async fn policy_hydrate(
    Extension(origin_ctx): Extension<OriginContext>,
    Extension(profile_ctx): Extension<ProfileContext>,
    Extension(branch_ctx): Extension<BranchContext>,
    State(policy_service): State<Arc<PolicyService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // We can safely extract extensions because it always has a fallback option (Anonymous).
    // We start extracting based on the combination of the three.
    let policies = match (origin_ctx, profile_ctx, branch_ctx) {
        // A customer is looking at the tenant's webpage.
        (
            OriginContext::TenantHome(tenant_id),
            ProfileContext::Customer(customer_id),
            branch_ctx,
        ) => {
            // Fetch policies for this customer for that tenant.
            let branch_id = match branch_ctx {
                BranchContext::Branch(ref branch_id) => Some(branch_id),
                BranchContext::Anonymous => None,
            };
            policy_service
                .get_customer_policies(&customer_id.id, &tenant_id, branch_id)
                .await
                .ok()
        }
        // A staff is looking at the tenant's staff system.
        (OriginContext::TenantPos(tenant_id), ProfileContext::Staff(staff_id), branch_ctx) => {
            let branch_id = match branch_ctx {
                BranchContext::Branch(ref branch_id) => Some(branch_id),
                BranchContext::Anonymous => None,
            };
            policy_service
                .get_staff_policies(&staff_id.id, &tenant_id, branch_id)
                .await
                .ok()
        }
        // A normal user looking at anything?
        (_, ProfileContext::System(system_id), _) => {
            policy_service.get_system_policies(&system_id.id).await.ok()
        }
        // For other cases, they are invalid and should be ignored.
        _ => {
            // Should do nothing here.
            None
        }
    };

    req.extensions_mut().insert(match policies {
        Some(policies) => PolicyContext::Authenticated(Arc::new(policies)),
        _ => PolicyContext::Anonymous,
    });

    Ok(next.run(req).await)
}

/// Just a final layer that compacts everything into one object for handlers to handle easily.
pub async fn context_solidify(
    Extension(origin_ctx): Extension<OriginContext>,
    Extension(session_ctx): Extension<SessionContext>,
    Extension(profile_ctx): Extension<ProfileContext>,
    Extension(branch_ctx): Extension<BranchContext>,
    Extension(policy_ctx): Extension<PolicyContext>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let request_ctx =
        RequestContext::new(origin_ctx, session_ctx, profile_ctx, branch_ctx, policy_ctx);

    req.extensions_mut().insert(Arc::new(request_ctx));
    Ok(next.run(req).await)
}
