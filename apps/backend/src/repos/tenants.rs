use sqlx::{Executor, Postgres};
use uuid::Uuid;

use crate::models::{Branch, Tenant};

pub async fn find_by_id(
    executor: impl Executor<'_, Database = Postgres>,
    id: &Uuid,
) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
SELECT id, name, slug, created_at, updated_at, deleted_at FROM tenants
WHERE deleted_at IS NULL AND id = $1
LIMIT 1
        "#,
        id
    )
    .fetch_optional(executor)
    .await
}

pub async fn find_by_slug(
    executor: impl Executor<'_, Database = Postgres>,
    slug: &str,
) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
SELECT id, name, slug, created_at, updated_at, deleted_at FROM tenants
WHERE deleted_at IS NULL AND LOWER(slug) = LOWER($1)
LIMIT 1
        "#,
        slug,
    )
    .fetch_optional(executor)
    .await
}

pub async fn get_branches(
    executor: impl Executor<'_, Database = Postgres>,
    tenant_id: &Uuid,
) -> Result<Vec<Branch>, sqlx::Error> {
    sqlx::query_as!(
        Branch,
        r#"
        SELECT id, tenant_id, name, created_at, updated_at, deleted_at
        FROM branches
        WHERE tenant_id = $1 AND deleted_at IS NULL"#,
        tenant_id
    )
    .fetch_all(executor)
    .await
}

pub async fn count_staff_tenants(
    executor: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
) -> Result<Option<i64>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
    SELECT count(t.*)
    FROM tenants t
    INNER JOIN staff_profiles sp ON sp.tenant_id = t.id
    WHERE sp.user_id = $1 AND t.deleted_at IS NULL AND sp.deleted_at IS NULL
    "#,
        user_id,
    )
    .fetch_one(executor)
    .await
}

pub async fn get_staff_tenants(
    executor: impl Executor<'_, Database = Postgres>,
    user_id: &Uuid,
    offset: i64,
    limit: i64,
) -> Result<Vec<Tenant>, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
    SELECT t.id, t.name, t.slug, t.created_at, t.updated_at, t.deleted_at
    FROM tenants t
    INNER JOIN staff_profiles sp ON sp.tenant_id = t.id
    WHERE sp.user_id = $1 AND t.deleted_at IS NULL AND sp.deleted_at IS NULL
    ORDER BY name
    OFFSET $2
    LIMIT $3
    "#,
        user_id,
        offset,
        limit,
    )
    .fetch_all(executor)
    .await
}

pub async fn insert_tenant(
    executor: impl Executor<'_, Database = Postgres>,
    name: &str,
    slug: &str,
) -> Result<Tenant, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
        INSERT INTO tenants (name, slug) VALUES ($1, $2)
        RETURNING id, name, slug, created_at, updated_at, deleted_at
        "#,
        name,
        slug,
    )
    .fetch_one(executor)
    .await
}
