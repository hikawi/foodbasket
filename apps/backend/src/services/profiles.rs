use fred::prelude::Client as CacheClient;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{CustomerProfile, StaffProfile, SystemProfile},
    repos,
};

#[derive(thiserror::Error, Debug)]
pub enum ProfileServiceError {
    #[error("Profile could not be found")]
    ProfileNotFound,

    #[error("Invalid parameters passed by developer")]
    InvalidParameters,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct ProfileService {
    pool: PgPool,
}

impl ProfileService {
    pub fn new(pool: PgPool, _cache: CacheClient) -> Self {
        Self { pool }
    }

    pub async fn get_customer_profile(
        &self,
        user_id: &Uuid,
        tenant_id: &Uuid,
    ) -> Result<CustomerProfile, ProfileServiceError> {
        match repos::profiles::find_customer(&self.pool, user_id, tenant_id).await? {
            Some(profile) => Ok(profile),
            None => Err(ProfileServiceError::ProfileNotFound),
        }
    }

    pub async fn get_staff_profile(
        &self,
        user_id: &Uuid,
        tenant_id: &Uuid,
    ) -> Result<StaffProfile, ProfileServiceError> {
        match repos::profiles::find_staff(&self.pool, user_id, tenant_id).await? {
            Some(profile) => Ok(profile),
            None => Err(ProfileServiceError::ProfileNotFound),
        }
    }

    pub async fn get_system_profile(
        &self,
        user_id: &Uuid,
    ) -> Result<SystemProfile, ProfileServiceError> {
        match repos::profiles::find_system(&self.pool, user_id).await? {
            Some(profile) => Ok(profile),
            None => Err(ProfileServiceError::ProfileNotFound),
        }
    }

    pub async fn get_staff_by_tenant(
        &self,
        tenant_id: &Uuid,
        branch_id: Option<&Uuid>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<StaffProfile>, i64), ProfileServiceError> {
        if page <= 0 || per_page <= 0 {
            return Err(ProfileServiceError::InvalidParameters);
        }

        let offset = (page - 1) * per_page;
        Ok(match branch_id {
            Some(branch_id) => {
                repos::profiles::find_all_staff_by_tenant_id_and_branch_id(
                    &self.pool, tenant_id, branch_id, offset, per_page,
                )
                .await?
            }
            None => {
                repos::profiles::find_all_staff_by_tenant_id(
                    &self.pool, tenant_id, offset, per_page,
                )
                .await?
            }
        })
    }
}
