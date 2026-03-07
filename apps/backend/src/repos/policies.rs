use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Policy, PolicyDocument};
use sqlx::types::Json;

pub async fn get_system_policies(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

pub async fn get_tenant_staff_policies(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

pub async fn get_branch_staff_policies(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

pub async fn get_tenant_customer_policies(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

pub async fn get_branch_customer_policies(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}
