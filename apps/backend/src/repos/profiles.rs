use chrono::{DateTime, Utc};
use sqlx::{Executor, PgExecutor, Postgres};
use uuid::Uuid;

use crate::models::{CustomerProfile, StaffProfile, SystemProfile};

#[derive(Debug)]
struct StaffProfileRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub pin_code: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub total_count: i64,
}

impl From<StaffProfileRow> for StaffProfile {
    fn from(value: StaffProfileRow) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            tenant_id: value.tenant_id,
            avatar_url: value.avatar_url,
            pin_code: value.pin_code,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

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

pub async fn find_all_staff_by_tenant_id(
    pool: impl PgExecutor<'_>,
    tenant_id: &Uuid,
    offset: i64,
    limit: i64,
) -> Result<(Vec<StaffProfile>, i64), sqlx::Error> {
    let rows = sqlx::query_as!(
        StaffProfileRow,
        r#"
        SELECT sp.*, COUNT(*) OVER() as "total_count!"
        FROM staff_profiles sp
        WHERE sp.tenant_id = $1 AND sp.deleted_at IS NULL
        ORDER BY sp.id
        OFFSET $2
        LIMIT $3
        "#,
        tenant_id,
        offset,
        limit
    )
    .fetch_all(pool)
    .await?;

    let count = rows.get(0).map(|v| v.total_count).unwrap_or(0);
    let entities = rows
        .into_iter()
        .map(StaffProfile::from)
        .collect::<Vec<StaffProfile>>();

    Ok((entities, count))
}

pub async fn find_all_staff_by_tenant_id_and_branch_id(
    pool: impl PgExecutor<'_>,
    tenant_id: &Uuid,
    branch_id: &Uuid,
    offset: i64,
    limit: i64,
) -> Result<(Vec<StaffProfile>, i64), sqlx::Error> {
    let rows = sqlx::query_as!(
        StaffProfileRow,
        r#"
        SELECT sp.*, COUNT(*) OVER() as "total_count!"
        FROM staff_profiles sp
        WHERE sp.tenant_id = $1 AND sp.deleted_at IS NULL AND EXISTS (
              SELECT 1 FROM staff_profiles_branches spb
              WHERE spb.staff_profile_id = sp.id
              AND spb.branch_id = $2
        )
        ORDER BY sp.id
        OFFSET $3
        LIMIT $4
        "#,
        tenant_id,
        branch_id,
        offset,
        limit,
    )
    .fetch_all(pool)
    .await?;

    let count = rows.get(0).map(|v| v.total_count).unwrap_or(0);
    let entities = rows
        .into_iter()
        .map(StaffProfile::from)
        .collect::<Vec<StaffProfile>>();

    Ok((entities, count))
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
