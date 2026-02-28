use sqlx::PgPool;

use crate::models::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE lower(email) = $1 AND deleted_at IS NULL",
        email
    )
    .fetch_one(pool)
    .await
}

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
        name,
        email,
        password
    )
    .fetch_one(pool)
    .await
}
