use sqlx::PgPool;

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
