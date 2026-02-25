use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must have at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, message = "Must have at least 2 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must have at least 8 characters"))]
    pub password: String,
}
