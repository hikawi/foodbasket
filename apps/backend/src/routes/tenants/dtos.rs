use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::models::Tenant;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TenantDTO {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenantRequest {
    #[validate(length(min = 3, message = "Name must be longer than 3 characters"))]
    pub name: String,
    #[validate(length(min = 3, message = "Slug must be longer than 3 characters"))]
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenantResponse {
    pub tenant_id: Uuid,
    pub tenant_slug: String,
}

impl From<Tenant> for TenantDTO {
    fn from(value: Tenant) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
