use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use regex::Regex;
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

static RE_SLUG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenantRequest {
    #[validate(length(min = 3, message = "Name must be longer than 3 characters"))]
    pub name: String,
    #[validate(
        length(min = 3, message = "Slug must be longer than 3 characters"),
        regex(path = *RE_SLUG, message = "Wee"),
    )]
    pub slug: String,
    #[validate(length(min = 1, message = "Policy name must be longer than 1 character"))]
    pub policy_name: String,
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
