use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Query, State, rejection::{JsonRejection, QueryRejection}},
};
use validator::Validate;

use crate::{
    api::{requests::PaginationQuery, responses::{ErrorResponse, PaginatedResponse}},
    error::AppError,
    routes::{
        extract::{RequestContext, SessionContext},
        tenants::{TenantError, dtos::{CreateTenantRequest, CreateTenantResponse, TenantDTO}},
    },
    services::{TenantService, TenantServiceError},
};

/// Retrieves a list of staff tenants for the POS system.
///
/// This only retrieves a paginated response for staff profiles. For user profiles ordering from
/// the tenants, please refer to the `customers` tag routes.
#[utoipa::path(
    get, 
    path = "/tenants", 
    tag = "pos", 
    responses(
        (status = 200, description = "Successful retrieval", body = PaginatedResponse<TenantDTO>),
        (status = 400, description = "Invalid query", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    params(
        ("page" = i64, Query, minimum = 1),
        ("per_page" = i64, Query, minimum = 1),
    ),
    security(("session_id" = [])),
)]
pub async fn get_tenants(
    State(tenant_service): State<Arc<TenantService>>,
    Extension(ctx): Extension<Arc<RequestContext>>,
    query_res: Result<Query<PaginationQuery>, QueryRejection>,
) -> Result<Json<PaginatedResponse<TenantDTO>>, AppError> {
    let Query(query) = query_res.map_err(TenantError::from)?;
    query.validate().map_err(TenantError::from)?;

    let user_id = match &ctx.session {
        SessionContext::Authenticated(sess) if sess.user_id.is_some() => sess.user_id.unwrap(),
        _ => Err(TenantError::Unauthorized(
            "User is not authenticated".into(),
        ))?,
    };

    let (tenants, count) = tenant_service
        .get_staff_tenants(&user_id, query.page, query.per_page)
        .await
        .map_err(TenantError::from)?;

    Ok(Json(PaginatedResponse::new(
        tenants.into_iter().map(TenantDTO::from).collect(),
        count,
        query.page,
        query.per_page,
    )))
}

/// Creates a new tenant with the requesting user as the first administrator.
#[utoipa::path(
    post,
    path = "/tenants",
    tag = "pos", 
    security(("session_id" = [])),
    responses(
        (status = 201, description = "Successful creation", body = CreateTenantResponse),
        (status = 400, description = "Invalid body", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "The requested slug is forbidden", body = ErrorResponse),
        (status = 409, description = "Slug is already taken", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn create_tenant(
    State(tenant_service): State<Arc<TenantService>>, 
    Extension(ctx): Extension<Arc<RequestContext>>,
    body: Result<Json<CreateTenantRequest>, JsonRejection>,
) -> Result<Json<CreateTenantResponse>, AppError> {
    let Json(body) = body.map_err(TenantError::from)?;
    body.validate().map_err(TenantError::from)?;

    let user_id = match &ctx.session {
        SessionContext::Authenticated(sess) if sess.user_id.is_some() => sess.user_id.unwrap(),
        _ => Err(TenantError::Unauthorized("User is not authenticated".into()))?,
    };

    let slug = &body.slug;
    let (tenant, _) = tenant_service.create_tenant(&user_id, &body.name, &body.slug).await.map_err(|e| match e {
        TenantServiceError::SlugTaken => TenantError::SlugTaken(format!("{slug} is already taken!")),
        TenantServiceError::SlugForbidden => TenantError::SlugForbidden(format!("{slug} is not allowed")),
        _ => TenantError::InternalServer(e),
    })?;

    Ok(Json(CreateTenantResponse { tenant_id: tenant.id, tenant_slug: tenant.slug.clone() }))
}
