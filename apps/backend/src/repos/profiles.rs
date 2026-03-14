use sqlx::{Executor, Postgres};
use uuid::Uuid;

use crate::models::{CustomerProfile, StaffProfile, SystemProfile};

pub async fn find_customer(
    pool: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
    tenant_id: &Uuid,
) -> Result<Option<CustomerProfile>, sqlx::Error> {
    sqlx::query_as!(
        CustomerProfile,
        r#"
        SELECT id, user_id, tenant_id, avatar_url, created_at, updated_at, deleted_at
        FROM customer_profiles
        WHERE user_id = $1 AND tenant_id = $2 AND deleted_at IS NULL
        LIMIT 1
        "#,
        user_id,
        tenant_id,
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_staff(
    pool: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
    tenant_id: &Uuid,
) -> Result<Option<StaffProfile>, sqlx::Error> {
    sqlx::query_as!(
        StaffProfile,
        r#"
        SELECT id, user_id, tenant_id, avatar_url, pin_code, created_at, updated_at, deleted_at
        FROM staff_profiles
        WHERE user_id = $1 AND tenant_id = $2 AND deleted_at IS NULL
        LIMIT 1
        "#,
        user_id,
        tenant_id,
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_system(
    pool: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
) -> Result<Option<SystemProfile>, sqlx::Error> {
    sqlx::query_as!(
        SystemProfile,
        r#"
        SELECT id, user_id, avatar_url, created_at, updated_at, deleted_at
        FROM system_profiles
        WHERE user_id = $1 AND deleted_at IS NULL
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_staff(
    pool: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
    tenant_id: &Uuid,
) -> Result<StaffProfile, sqlx::Error> {
    sqlx::query_as!(
        StaffProfile,
        r#"
        INSERT INTO staff_profiles (user_id, tenant_id) VALUES ($1, $2)
        RETURNING *
        "#,
        user_id,
        tenant_id,
    )
    .fetch_one(pool)
    .await
}
