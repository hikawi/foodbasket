use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Tenant;

pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
SELECT id, name, slug, created_at, updated_at, deleted_at FROM tenants
WHERE deleted_at IS NULL AND id = $1
LIMIT 1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as!(
        Tenant,
        r#"
SELECT id, name, slug, created_at, updated_at, deleted_at FROM tenants
WHERE deleted_at IS NULL AND LOWER(slug) = LOWER($1)
LIMIT 1
        "#,
        slug,
    )
    .fetch_optional(pool)
    .await
}
