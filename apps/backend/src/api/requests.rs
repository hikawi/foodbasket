use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
#[serde(default)]
pub struct PaginationQuery {
    #[validate(range(min = 1, message = "Page must be greater than zero"))]
    pub page: i64,
    #[validate(range(min = 1, message = "Per Page must be greater than zero"))]
    pub per_page: i64,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}
