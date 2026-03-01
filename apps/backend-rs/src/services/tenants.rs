use fred::{
    prelude::{Client as CacheClient, KeysInterface},
    types::Expiration,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{cache_keys, repos};

const TENANT_CACHE_TTL: i64 = 5 * 60;

#[derive(Debug)]
pub struct TenantService {
    pool: PgPool,
    cache: CacheClient,
}

#[derive(Debug, thiserror::Error)]
pub enum TenantServiceError {
    #[error("Uuid could not be parsed")]
    UuidError,

    #[error(transparent)]
    DatabaseError(anyhow::Error),
}

impl From<sqlx::Error> for TenantServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(anyhow::Error::new(value))
    }
}

impl From<fred::error::Error> for TenantServiceError {
    fn from(value: fred::error::Error) -> Self {
        Self::DatabaseError(anyhow::Error::new(value))
    }
}

impl From<uuid::Error> for TenantServiceError {
    fn from(_: uuid::Error) -> Self {
        Self::UuidError
    }
}

impl TenantService {
    pub fn new(pool: PgPool, cache: CacheClient) -> Self {
        Self { pool, cache }
    }

    /// Retrieves the Uuid of a tenant from their slug. Returns `Ok(Some(uuid))` if
    /// the slug is a valid tenant, or `Ok(None)` if the slug is an invalid tenant.
    ///
    /// # Errors
    ///
    /// - `TenantServiceError::UuidError`: The cache hits, but the value is not parsable.
    /// - `TenantServiceError::DatabaseError`: The cache or the database failed.
    pub async fn get_id_by_slug(&self, slug: &str) -> Result<Option<Uuid>, TenantServiceError> {
        let key = cache_keys::tenant_slug(slug);
        let val: Option<String> = self.cache.get(&key).await?;

        match val {
            Some(s) if s == "NF" => {
                return Ok(None);
            }
            Some(s) => {
                let uuid = Uuid::try_parse(&s)?;
                return Ok(Some(uuid));
            }
            None => (),
        }

        // Cache miss fr.
        let tenant = repos::tenants::find_by_slug(&self.pool, slug).await?;

        let cache = self.cache.clone();
        let cache_key = key.clone();
        let cache_value = match &tenant {
            Some(t) => t.id.to_string(),
            None => "NF".to_string(),
        };

        tokio::spawn(async move {
            let _ = cache
                .set::<(), _, _>(
                    cache_key,
                    cache_value,
                    Some(Expiration::EX(TENANT_CACHE_TTL)),
                    None,
                    false,
                )
                .await;
        });

        Ok(tenant.map(|t| t.id))
    }

    /// Checks if the provided Uuid is a valid tenant.
    ///
    /// Returns `Ok(true)` if the UUID maps to a valid tenant, `Ok(false)` otherwise.
    ///
    /// # Errors
    ///
    /// - `TenantServiceError::DatabaseError` if the cache failed.
    pub async fn is_tenant(&self, id: Uuid) -> Result<bool, TenantServiceError> {
        let id_str = id.to_string();

        let key = cache_keys::tenant_uuid(&id_str);

        // Cache hit.
        if let Some(s) = self.cache.get::<Option<String>, _>(&key).await? {
            return Ok(s == "true");
        }

        // Cache miss, check database.
        let tenant = repos::tenants::find_by_id(&self.pool, &id).await?;
        let exists = tenant.is_some();

        // Save negative/positive result in cache.
        // Prepare stuff to move to tokio.
        let cache_client = self.cache.clone();
        let cache_key = key.clone(); // Must clone the string to 'move' it
        let cache_val = if exists { "true" } else { "false" };

        // Fire and forget thingy
        tokio::spawn(async move {
            let _ = cache_client
                .set::<(), _, _>(
                    cache_key,
                    cache_val,
                    Some(Expiration::EX(TENANT_CACHE_TTL)),
                    None,
                    false,
                )
                .await;
        });

        Ok(exists)
    }
}
