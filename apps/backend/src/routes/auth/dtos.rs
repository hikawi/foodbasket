use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::models::{CustomerProfile, StaffProfile, SystemProfile, User};

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostLoginRequest {
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostRegisterRequest {
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters"))]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDTO {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponse {
    pub user: UserDTO,
    pub tenant_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub customer_profile: Option<ProfileDTO>,
    pub staff_profile: Option<ProfileDTO>,
    pub system_profile: Option<ProfileDTO>,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Arc<SystemProfile>> for ProfileDTO {
    fn from(value: Arc<SystemProfile>) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            avatar_url: value.avatar_url.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Arc<CustomerProfile>> for ProfileDTO {
    fn from(value: Arc<CustomerProfile>) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            avatar_url: value.avatar_url.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Arc<StaffProfile>> for ProfileDTO {
    fn from(value: Arc<StaffProfile>) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            avatar_url: value.avatar_url.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
