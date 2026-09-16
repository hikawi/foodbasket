use fred::prelude::Client as CacheClient;
use sqlx::PgPool;
use tokio::try_join;
use uuid::Uuid;

use crate::{
    models::{Policy, PolicyDocument},
    repos,
};

#[derive(Debug, thiserror::Error)]
pub enum PolicyServiceError {
    #[error("Invalid parameters")]
    InvalidParameters,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct PolicyService {
    pool: PgPool,
}

impl PolicyService {
    pub fn new(pool: PgPool, _cache: CacheClient) -> Self {
        Self { pool }
    }

    pub async fn get_system_policies(
        &self,
        system_profile_id: &Uuid,
    ) -> Result<Vec<Policy>, PolicyServiceError> {
        repos::policies::get_system_policies(&self.pool, system_profile_id)
            .await
            .map_err(PolicyServiceError::from)
    }

    pub async fn get_customer_policies(
        &self,
        customer_profile_id: &Uuid,
        tenant_id: &Uuid,
        branch_id: Option<&Uuid>,
    ) -> Result<Vec<Policy>, PolicyServiceError> {
        match branch_id {
            Some(branch_id) => {
                repos::policies::get_branch_customer_policies(
                    &self.pool,
                    customer_profile_id,
                    tenant_id,
                    branch_id,
                )
                .await
            }
            None => {
                repos::policies::get_tenant_customer_policies(
                    &self.pool,
                    customer_profile_id,
                    tenant_id,
                )
                .await
            }
        }
        .map_err(PolicyServiceError::from)
    }

    /// Retrieves the policies assigned to a staff profile within a tenant.
    pub async fn get_staff_policies(
        &self,
        staff_profile_id: &Uuid,
        tenant_id: &Uuid,
        branch_id: Option<&Uuid>,
    ) -> Result<Vec<Policy>, PolicyServiceError> {
        match branch_id {
            Some(branch_id) => {
                repos::policies::get_branch_staff_policies(
                    &self.pool,
                    staff_profile_id,
                    tenant_id,
                    branch_id,
                )
                .await
            }
            None => {
                repos::policies::get_tenant_staff_policies(&self.pool, staff_profile_id, tenant_id)
                    .await
            }
        }
        .map_err(PolicyServiceError::from)
    }

    /// Retrieves the policies for a branch or a tenant level.
    ///
    /// If `branch_id` is not specified, then provides ALL policies.
    /// If `branch_id` is specified, provides the policies scoped to that branch and tenant-scoped.
    pub async fn get_tenant_policies(
        &self,
        tenant_id: &Uuid,
        branch_id: Option<&Uuid>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Policy>, i64), PolicyServiceError> {
        let policies_fut =
            repos::policies::get_tenant_policies(&self.pool, tenant_id, branch_id, offset, limit);
        let count_fut = repos::policies::count_tenant_policies(&self.pool, tenant_id, branch_id);

        let (policies, count) =
            try_join!(policies_fut, count_fut).map_err(PolicyServiceError::from)?;

        Ok((policies, count.unwrap_or(0)))
    }

    /// Creates a new policy.
    ///
    /// - `tenant_id`: Specified to scope to a tenant, otherwise system.
    /// - `branch_id`: Specified to scope to a branch, otherwise tenant.
    pub async fn create_policy(
        &self,
        tenant_id: Option<&Uuid>,
        branch_id: Option<&Uuid>,
        name: &str,
        document: &PolicyDocument,
    ) -> Result<Policy, PolicyServiceError> {
        if tenant_id.is_none() && branch_id.is_some() {
            return Err(PolicyServiceError::InvalidParameters);
        }

        repos::policies::insert_policy(&self.pool, tenant_id, branch_id, name, document)
            .await
            .map_err(PolicyServiceError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use fred::{interfaces::ClientLike, mocks::SimpleMap};
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{
        models::PolicyDocument,
        services::{PolicyService, PolicyServiceError},
    };

    fn setup_mock_client() -> fred::prelude::Client {
        let mock_cache = SimpleMap::new();

        let config = fred::prelude::Config {
            mocks: Some(Arc::new(mock_cache)),
            ..fred::prelude::Config::default()
        };
        let cache_client = fred::prelude::Client::new(config, None, None, None);
        cache_client.connect();

        cache_client
    }

    async fn populate_test_policies(
        pool: &PgPool,
        tenant_id: &Uuid,
        branch1_id: &Uuid,
        branch2_id: &Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO tenants (id, name, slug)
            VALUES
              ($1, 'Yahoo', 'yahoo');
            "#,
            tenant_id,
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO branches (id, tenant_id, name)
            VALUES
              ($2, $1, 'Branch 1'),
              ($3, $1, 'Branch 2');
            "#,
            tenant_id,
            branch1_id,
            branch2_id,
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO policies (tenant_id, branch_id, name, statements)
            VALUES
              ($1, NULL, 'Policy 1', '{"version":"v1","statements":[{"actions":["pos:menus:read"],"effect":"allow"}]}'),
              ($1, $2, 'Policy 2', '{"version":"v1","statements":[{"actions":["pos:menus:read"],"effect":"deny"}]}'::jsonb),
              ($1, $3, 'Policy 2', '{"version":"v1","statements":[{"actions":["pos:menus:read"],"effect":"deny"}]}'::jsonb);
            "#,
            tenant_id,
            branch1_id,
            branch2_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    #[sqlx::test]
    pub async fn get_policies_test_empty(pool: PgPool) -> anyhow::Result<()> {
        let mock_client = setup_mock_client();

        // Setup.
        let tenant_id = Uuid::new_v4();

        // Test.
        let policy_service = PolicyService::new(pool, mock_client.clone());
        let (policies, count) = policy_service
            .get_tenant_policies(&tenant_id, None, 0, 20)
            .await?;

        assert_eq!(policies.len(), 0);
        assert_eq!(count, 0);

        Ok(())
    }

    #[sqlx::test]
    pub async fn get_policies_test(pool: PgPool) -> anyhow::Result<()> {
        let mock_client = setup_mock_client();

        // Setup.
        let tenant_id = Uuid::new_v4();
        let branch1_id = Uuid::new_v4();
        let branch2_id = Uuid::new_v4();
        populate_test_policies(&pool, &tenant_id, &branch1_id, &branch2_id).await?;

        // Test.
        let policy_service = PolicyService::new(pool, mock_client.clone());
        let (_, tenant_count) = policy_service
            .get_tenant_policies(&tenant_id, None, 0, 20)
            .await?;
        let (_, branch_count) = policy_service
            .get_tenant_policies(&tenant_id, Some(&branch1_id), 0, 20)
            .await?;

        assert_eq!(tenant_count, 3); // Should yield ALL policies for ALL branches.
        assert_eq!(branch_count, 2); // Should just be for BRANCH_1 AND tenant.

        Ok(())
    }

    #[sqlx::test]
    pub async fn create_policy_invalid_parameters(pool: PgPool) -> anyhow::Result<()> {
        let mock_client = setup_mock_client();

        // Setup.
        let branch1_id = Uuid::new_v4();

        // Test.
        let policy_service = PolicyService::new(pool, mock_client.clone());
        let res = policy_service
            .create_policy(
                None,
                Some(&branch1_id),
                "test",
                &PolicyDocument {
                    version: "v1".into(),
                    statements: vec![],
                },
            )
            .await;

        assert!(matches!(res, Err(PolicyServiceError::InvalidParameters)));
        Ok(())
    }

    #[sqlx::test]
    pub async fn create_policy_success(pool: PgPool) -> anyhow::Result<()> {
        let mock_client = setup_mock_client();

        // Setup.
        let tenant1_id = Uuid::new_v4();
        let branch1_id = Uuid::new_v4();
        let branch2_id = Uuid::new_v4();
        populate_test_policies(&pool, &tenant1_id, &branch1_id, &branch2_id).await?;

        // Test.
        let policy_service = PolicyService::new(pool, mock_client.clone());
        let res = policy_service
            .create_policy(
                Some(&tenant1_id),
                Some(&branch1_id),
                "test",
                &PolicyDocument {
                    version: "v1".into(),
                    statements: vec![],
                },
            )
            .await;

        assert!(matches!(res, Ok(_)));
        assert_eq!(res.unwrap().tenant_id.unwrap(), tenant1_id);

        Ok(())
    }
}
