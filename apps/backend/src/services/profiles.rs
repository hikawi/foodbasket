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
}
