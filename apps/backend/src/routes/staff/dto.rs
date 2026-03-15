use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::StaffProfile;

#[derive(Serialize, ToSchema)]
pub struct StaffProfileDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub pin_code: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateStaffRequest {
    pub user_id: Uuid,
}

impl From<StaffProfile> for StaffProfileDTO {
    fn from(value: StaffProfile) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            tenant_id: value.tenant_id,
            pin_code: value.pin_code,
            avatar_url: value.avatar_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
