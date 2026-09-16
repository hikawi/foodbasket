use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Query, State, rejection::QueryRejection},
};

use validator::Validate;

use crate::{
    api::{
        requests::PaginationQuery,
        responses::{ErrorResponse, PaginatedResponse},
    },
    error::AppError,
    permissions,
    routes::{
        extract::{RequestContext, TenantContext},
        policies::{PolicyError, dto::PolicyDto},
    },
    services::PolicyService,
};

/// Retrieves a list of roles currently belonging to a tenant or a branch of one.
#[utoipa::path(
    get,
    path = "/policies",
    tag = "policies",
    params(
        ("page" = i64, Query, minimum = 1),
        ("per_page" = i64, Query, minimum = 1),
    ),
    security(("session_id" = []), ("branch_id" = []), ("tenant_slug" = []), ("app_context" = [])),
    responses(
        (status = 200, description = "Successful retrieval", body = PaginatedResponse<PolicyDto>),
        (status = 400, description = "Invalid query or invalid branch ID", body = ErrorResponse),
        (status = 401, description = "Unauthenticated or not enough permissions", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn get_policies(
    State(policy_service): State<Arc<PolicyService>>,
    Extension(ctx): Extension<Arc<RequestContext>>,
    query: Result<Query<PaginationQuery>, QueryRejection>,
) -> Result<Json<PaginatedResponse<PolicyDto>>, AppError> {
    if !ctx.has_permission(permissions::POS_ROLE_READ) {
        Err(PolicyError::Unauthorized("Unauthorized".into()))?;
    }

    let tenant_id = match &ctx.origin {
        TenantContext::Tenant(uuid) => uuid,
        _ => Err(PolicyError::BadContext("Expected POS context".into()))?,
    };

    let Query(query) = query.map_err(PolicyError::from)?;
    query.validate().map_err(PolicyError::from)?;

    let branch_id = ctx.branch.0;

    let (policies, total) = policy_service
        .get_tenant_policies(tenant_id, branch_id.as_ref(), query.page, query.per_page)
        .await
        .map_err(|_| PolicyError::Internal("Database error".into()))?;

    Ok(Json(PaginatedResponse::new(
        policies.into_iter().map(PolicyDto::from).collect(),
        total,
        query.page,
        query.per_page,
    )))
}
