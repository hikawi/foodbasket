use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::{Policy, PolicyDocument};
use sqlx::types::Json;

pub async fn get_system_policies(
    executor: impl PgExecutor<'_>,
    system_profile_id: &Uuid,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT 
            p.id, 
            p.tenant_id, 
            p.branch_id, 
            p.name, 
            p.statements AS "statements: Json<PolicyDocument>", 
            p.created_at, 
            p.updated_at, 
            p.deleted_at
        FROM policies p
        INNER JOIN assignments a ON a.policy_id = p.id
        WHERE a.system_profile_id = $1
          AND p.deleted_at IS NULL 
          AND a.scope_type = 'system'::scope_type
        "#,
        system_profile_id,
    )
    .fetch_all(executor)
    .await
}

pub async fn get_tenant_staff_policies(
    executor: impl PgExecutor<'_>,
    staff_profile_id: &Uuid,
    tenant_id: &Uuid,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT 
            p.id, 
            p.tenant_id, 
            p.branch_id, 
            p.name, 
            p.statements AS "statements: Json<PolicyDocument>", 
            p.created_at, 
            p.updated_at, 
            p.deleted_at
        FROM policies p
        INNER JOIN assignments a ON a.policy_id = p.id
        WHERE a.staff_profile_id = $1 
          AND p.deleted_at IS NULL 
          AND (
            (a.scope_type = 'system'::scope_type)
            OR (a.scope_type = 'tenant'::scope_type AND a.scope_id = $2)
          )
        "#,
        staff_profile_id,
        tenant_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_branch_staff_policies(
    executor: impl PgExecutor<'_>,
    staff_profile_id: &Uuid,
    tenant_id: &Uuid,
    branch_id: &Uuid,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT 
            p.id, p.tenant_id, p.branch_id, p.name, 
            p.statements AS "statements: Json<PolicyDocument>", 
            p.created_at, p.updated_at, p.deleted_at
        FROM policies p
        INNER JOIN assignments a ON a.policy_id = p.id
        WHERE a.staff_profile_id = $1 
          AND p.deleted_at IS NULL 
          AND (
            (a.scope_type = 'system'::scope_type)
            OR (a.scope_type = 'tenant'::scope_type AND a.scope_id = $2)
            OR (a.scope_type = 'branch'::scope_type AND a.scope_id = $3)
          )
        "#,
        staff_profile_id,
        tenant_id,
        branch_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_tenant_customer_policies(
    executor: impl PgExecutor<'_>,
    customer_profile_id: &Uuid,
    tenant_id: &Uuid,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT 
            p.id, p.tenant_id, p.branch_id, p.name, 
            p.statements AS "statements: Json<PolicyDocument>", 
            p.created_at, p.updated_at, p.deleted_at
        FROM policies p
        INNER JOIN assignments a ON a.policy_id = p.id
        WHERE a.customer_profile_id = $1 
          AND p.deleted_at IS NULL 
          AND (
            a.scope_type = 'system'::scope_type
            OR (a.scope_type = 'tenant'::scope_type AND a.scope_id = $2)
          )  
        "#,
        customer_profile_id,
        tenant_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_branch_customer_policies(
    executor: impl PgExecutor<'_>,
    customer_profile_id: &Uuid,
    tenant_id: &Uuid,
    branch_id: &Uuid,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT 
            p.id, p.tenant_id, p.branch_id, p.name, 
            p.statements AS "statements: Json<PolicyDocument>", 
            p.created_at, p.updated_at, p.deleted_at
        FROM policies p
        INNER JOIN assignments a ON a.policy_id = p.id
        WHERE a.customer_profile_id = $1 
          AND p.deleted_at IS NULL 
          AND (
            a.scope_type = 'system'::scope_type
            OR (a.scope_type = 'tenant'::scope_type AND a.scope_id = $2)
            OR (a.scope_type = 'branch'::scope_type AND a.scope_id = $3)
          )  
        "#,
        customer_profile_id,
        tenant_id,
        branch_id,
    )
    .fetch_all(executor)
    .await
}

/// Counts policies bound to a tenant.
///
/// Parameters:
/// - `tenant_id`: The tenant ID.
/// - `branch_id`: Specified branch ID to scope policies, otherwise get everything.
pub async fn count_tenant_policies(
    executor: impl PgExecutor<'_>,
    tenant_id: &Uuid,
    branch_id: Option<&Uuid>,
) -> Result<Option<i64>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT count(p.*)
        FROM policies p
        WHERE tenant_id = $1 AND (branch_id = $2 OR branch_id IS NULL)
        "#,
        tenant_id,
        branch_id,
    )
    .fetch_one(executor)
    .await
}

/// Retrieves policies bound to a tenant.
pub async fn get_tenant_policies(
    executor: impl PgExecutor<'_>,
    tenant_id: &Uuid,
    branch_id: Option<&Uuid>,
    offset: i64,
    limit: i64,
) -> Result<Vec<Policy>, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
        SELECT id, tenant_id, branch_id, name, statements AS "statements: Json<PolicyDocument>", created_at, updated_at, deleted_at
        FROM policies
        WHERE tenant_id = $1 AND (branch_id = $2 OR branch_id IS NULL)
        OFFSET $3
        LIMIT $4
        "#,
        tenant_id,
        branch_id,
        offset,
        limit,
    )
    .fetch_all(executor)
    .await
}

pub async fn insert_policy(
    executor: impl PgExecutor<'_>,
    tenant_id: Option<&Uuid>,
    branch_id: Option<&Uuid>,
    name: &str,
    policy_document: &PolicyDocument,
) -> Result<Policy, sqlx::Error> {
    sqlx::query_as!(
        Policy,
        r#"
    INSERT INTO policies (tenant_id, branch_id, name, statements)
    VALUES ($1, $2, $3, $4)
    RETURNING id, tenant_id, branch_id, name, statements as "statements: Json<PolicyDocument>", created_at, updated_at, deleted_at
    "#,
        tenant_id,
        branch_id,
        name,
        Json(policy_document) as _,
    )
    .fetch_one(executor)
    .await
}
