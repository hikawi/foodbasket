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
        staff::{StaffError, dto::StaffProfileDTO},
    },
    services::{ProfileService, ProfileServiceError},
};

/// Retrieves a list of staff members belonging to a tenant.
#[utoipa::path(
    get,
    path = "/staff",
    tag = "staff",
    params(
        ("page" = i64, Query, minimum = 1),
        ("per_page" = i64, Query, minimum = 1),
    ),
    security(("session_id" = []), ("branch_id" = []), ("tenant_slug" = []), ("app_context" = [])),
    responses(
        (status = 200, description = "Successful retrieval", body = PaginatedResponse<StaffProfileDTO>),
        (status = 400, description = "Invalid query or invalid branch ID", body = ErrorResponse),
        (status = 401, description = "Unauthenticated or not enough permissions", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn get_staff(
    State(profile_service): State<Arc<ProfileService>>,
    Extension(ctx): Extension<Arc<RequestContext>>,
    query: Result<Query<PaginationQuery>, QueryRejection>,
) -> Result<Json<PaginatedResponse<StaffProfileDTO>>, AppError> {
    if !ctx.has_permission(permissions::pos::staff::READ) {
        return Err(StaffError::Unauthorized("Unauthorized".into()))?;
    }

    let tenant_id = match &ctx.origin {
        TenantContext::Tenant(uuid) => uuid,
        _ => Err(StaffError::BadContext("Expected POS context".into()))?,
    };

    let Query(query) = query.map_err(StaffError::from)?;
    query.validate().map_err(StaffError::from)?;

    let branch_id = ctx.branch.0;

    let (profiles, total) = profile_service
        .get_staff_by_tenant(tenant_id, branch_id.as_ref(), query.page, query.per_page)
        .await
        .map_err(|e| match e {
            ProfileServiceError::InvalidParameters => {
                StaffError::ValidationFailed("Bad request".into())
            }
            _ => StaffError::Internal(e.to_string()),
        })?;

    Ok(Json(PaginatedResponse::new(
        profiles.into_iter().map(StaffProfileDTO::from).collect(),
        total,
        query.page,
        query.per_page,
    )))
}
