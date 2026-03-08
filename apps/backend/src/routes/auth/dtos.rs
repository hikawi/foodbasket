use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

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
pub enum ProfileContextResponse {
    Staff,
    Customer,
    System,
    Anonymous,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponse {
    pub user_id: Uuid,
    pub user_email: String,
    pub tenant_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub profile_context: ProfileContextResponse,
}
