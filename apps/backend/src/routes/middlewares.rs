use std::sync::Arc;

use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use http::{StatusCode, header::InvalidHeaderValue};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    error::AppError,
    routes::extract::{
        AppContext, BranchContext, PolicyContext, ProfileContext, RequestContext, SessionContext,
        TenantContext,
    },
    services::{PolicyService, ProfileService, SessionService, TenantService, TenantServiceError},
};

#[derive(Debug, thiserror::Error)]
pub enum MiddlewareError {
    #[error("FAILED_TO_SET_HEADERS")]
    FailedToSetHeaders,

    #[error("INVALID_TENANT")]
    InvalidTenant,

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
            Self::InvalidTenant => (
                StatusCode::BAD_REQUEST,
                code.clone(),
                "Malformed tenant slug".into(),
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

/// Applies a layer that extracts the X-Tenant-Slug header out of the request, and check whether it's a
/// valid host in our infrastructure.
pub async fn tenant_hydrate(
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let tenant_context = match req.headers().get("x-tenant-slug") {
        Some(header) => {
            let val = header
                .to_str()
                .map_err(|_| MiddlewareError::InvalidTenant)?;
            match val {
                "admin" => TenantContext::Admin,
                slug => TenantContext::Tenant(
                    tenant_service
                        .get_id_by_slug(&slug.to_lowercase())
                        .await
                        .map_err(MiddlewareError::from)?
                        .ok_or(MiddlewareError::UnknownTenant)?,
                ),
            }
        }
        None => TenantContext::Anonymous,
    };

    req.extensions_mut().insert(tenant_context);

    // We injected the origin context.
    // We can now move forward.
    Ok(next.run(req).await)
}

/// A layer to hydrate the AppContext for the request based on the x-app-context header value and the previous
/// layer's extension. This middleware is guaranteed to run after tenant_hydrate.
pub async fn app_hydrate(mut req: Request, next: Next) -> Result<Response, AppError> {
    let app_context = match req.extensions().get::<TenantContext>() {
        Some(&TenantContext::Tenant(_)) => match req.headers().get("x-app-context") {
            Some(val) => match val.to_str() {
                Ok(s) if s.to_lowercase() == "pos" => AppContext::Pos,
                Ok(s) if s.to_lowercase() == "store" => AppContext::Storefront,
                _ => AppContext::None,
            },
            _ => AppContext::None,
        },
        _ => AppContext::None,
    };

    req.extensions_mut().insert(app_context);

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
            Ok(session) => SessionContext(Some(Arc::new(session))),
            Err(_) => SessionContext(None),
        },
        None => SessionContext(None),
    };

    req.extensions_mut().insert(session_ctx);
    Ok(next.run(req).await)
}

/// Applies a layer that reads the extension for SessionContext. Depending on the user and the
/// context, reads the profile. There are 3 separations for best isolation, as staff_profiles,
/// customer_profiles, system_profiles are all separate, but link to the same user.
pub async fn profile_hydrate(
    State(profile_service): State<Arc<ProfileService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Attempt to read the profile based on the session and origin.
    // SessionContext holds an Arc<Session>.
    let sess = match req.extensions().get::<SessionContext>() {
        Some(SessionContext(Some(sess))) => sess.clone(),
        _ => {
            // If not authenticated. Why would we hydrate profiles?
            req.extensions_mut().insert(ProfileContext::Anonymous);
            return Ok(next.run(req).await);
        }
    };

    let tenant_ctx = req.extensions().get::<TenantContext>();
    let app_ctx = req.extensions().get::<AppContext>();

    let ctx = match (tenant_ctx, app_ctx, sess.user_id) {
        // If it is in tenant context and authenticated, parse as customer profiles.
        (Some(TenantContext::Tenant(tenant_id)), Some(AppContext::Storefront), Some(user_id)) => {
            match profile_service
                .get_customer_profile(&user_id, &tenant_id)
                .await
            {
                Ok(profile) => ProfileContext::Customer(Arc::new(profile)),
                _ => ProfileContext::Anonymous,
            }
        }
        // If it is in POS context and authenticated, parse as staff profiles.
        (Some(TenantContext::Tenant(tenant_id)), Some(AppContext::Pos), Some(user_id)) => {
            match profile_service
                .get_staff_profile(&user_id, &tenant_id)
                .await
            {
                Ok(profile) => ProfileContext::Staff(Arc::new(profile)),
                _ => ProfileContext::Anonymous,
            }
        }
        // If it is in admin context and authenticated, parse as system profiles.
        (Some(TenantContext::Admin), _, Some(user_id)) => {
            match profile_service.get_system_profile(&user_id).await {
                Ok(profile) => ProfileContext::System(Arc::new(profile)),
                _ => ProfileContext::Anonymous,
            }
        }
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
    State(tenant_service): State<Arc<TenantService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let branch_id = req
        .headers()
        .get("x-branch-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok());

    let tenant_ctx = req.extensions().get::<TenantContext>();
    let branch_ctx = match (branch_id, tenant_ctx) {
        (Some(branch_id), Some(&TenantContext::Tenant(tenant_id))) => {
            match tenant_service
                .is_branch_of_tenant(&branch_id, &tenant_id)
                .await
            {
                Ok(true) => BranchContext(Some(branch_id)),
                _ => BranchContext(None),
            }
        }
        _ => BranchContext(None),
    };

    req.extensions_mut().insert(branch_ctx);
    Ok(next.run(req).await)
}

/// The innermost required layer to pull permissions in the form of policies. This will actually
/// run a calculate IF AND ONLY IF all other middlewares succeeded in querying or extracting the
/// context needed. Otherwise, this injects an empty PolicyContext.
pub async fn policy_hydrate(
    State(policy_service): State<Arc<PolicyService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let tenant_ctx = req.extensions().get::<TenantContext>();
    let profile_ctx = req.extensions().get::<ProfileContext>();
    let branch_ctx = req.extensions().get::<BranchContext>();

    let branch_id = match branch_ctx {
        Some(BranchContext(Some(uuid))) => Some(uuid),
        _ => None,
    };

    // We can safely extract extensions because it always has a fallback option (Anonymous).
    // We start extracting based on the combination of the three.
    let policies = match (tenant_ctx, profile_ctx, branch_id) {
        // A customer is looking at the tenant's webpage.
        (
            Some(TenantContext::Tenant(tenant_id)),
            Some(ProfileContext::Customer(customer_profile)),
            branch_id,
        ) => policy_service
            .get_customer_policies(&customer_profile.id, &tenant_id, branch_id)
            .await
            .ok(),
        // A staff is looking at the tenant's staff system.
        (
            Some(TenantContext::Tenant(tenant_id)),
            Some(ProfileContext::Staff(staff_profile)),
            branch_id,
        ) => policy_service
            .get_staff_policies(&staff_profile.id, &tenant_id, branch_id)
            .await
            .ok(),
        // A system user looking at admin.
        (Some(TenantContext::Admin), Some(ProfileContext::System(system_profile)), _) => {
            policy_service
                .get_system_policies(&system_profile.id)
                .await
                .ok()
        }
        // For other cases, they are invalid and should be ignored.
        _ => {
            // Should do nothing here.
            None
        }
    };

    req.extensions_mut().insert(match policies {
        Some(policies) => PolicyContext(Some(Arc::new(policies))),
        _ => PolicyContext(None),
    });

    Ok(next.run(req).await)
}

/// Just a final layer that compacts everything into one object for handlers to handle easily.
pub async fn context_solidify(
    Extension(origin_ctx): Extension<TenantContext>,
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
