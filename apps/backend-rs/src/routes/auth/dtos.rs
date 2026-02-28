use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct PostLoginRequest {
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct PostRegisterRequest {
    #[validate(length(min = 2, message = "Must be at least 2 characters"))]
    pub name: String,
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters"))]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct GetMeResponse {
    pub id: String,
    pub email: String,
}
