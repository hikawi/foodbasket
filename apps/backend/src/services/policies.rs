use fred::prelude::Client as CacheClient;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::Policy, repos};

#[derive(Debug, thiserror::Error)]
pub enum PolicyServiceError {
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
}
