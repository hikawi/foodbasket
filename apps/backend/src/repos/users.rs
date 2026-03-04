use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE lower(email) = $1 AND deleted_at IS NULL",
        email
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
        email,
        password
    )
    .fetch_one(pool)
    .await
}

/// Gets a list of permissions for the user that belongs to a tenant.
/// This defaults to using a staff profile
pub async fn get_tenant_permissions(pool: &PgPool, user_id: &Uuid, tenant_id: Option<&Uuid>) {}
