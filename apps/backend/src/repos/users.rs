use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::User;

pub async fn find_by_id(
    executor: impl PgExecutor<'_>,
    id: &Uuid,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL LIMIT 1",
        id
    )
    .fetch_optional(executor)
    .await
}

pub async fn find_by_email(
    executor: impl PgExecutor<'_>,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE lower(email) = $1 AND deleted_at IS NULL LIMIT 1",
        email
    )
    .fetch_optional(executor)
    .await
}

pub async fn create_user(
    executor: impl PgExecutor<'_>,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
        email,
        password
    )
    .fetch_one(executor)
    .await
}
