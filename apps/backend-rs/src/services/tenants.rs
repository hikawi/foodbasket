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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use fred::{mocks::SimpleMap, prelude::ClientLike};

    use crate::models::Tenant;

    use super::*;

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

    #[sqlx::test]
    async fn test_get_id_by_slug_not_found(pool: PgPool) -> anyhow::Result<()> {
        let cache_client = setup_mock_client();

        let _ = cache_client
            .set::<(), _, _>(cache_keys::tenant_slug("test"), "NF", None, None, false)
            .await?;

        let service = TenantService::new(pool, cache_client);
        let uuid = service.get_id_by_slug("test").await;
        assert!(matches!(uuid, Ok(None)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_id_by_slug_bad_uuid(pool: PgPool) -> anyhow::Result<()> {
        let cache_client = setup_mock_client();

        let _ = cache_client
            .set::<(), _, _>(cache_keys::tenant_slug("test"), "lol", None, None, false)
            .await?;

        let service = TenantService::new(pool, cache_client);
        let uuid = service.get_id_by_slug("test").await;
        assert!(matches!(uuid, Err(TenantServiceError::UuidError)));

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_id_by_slug_cache_hit(pool: PgPool) -> anyhow::Result<()> {
        let cache_client = setup_mock_client();

        let uuid = Uuid::new_v4();
        let _ = cache_client
            .set::<(), _, _>(
                cache_keys::tenant_slug("test"),
                uuid.to_string(),
                None,
                None,
                false,
            )
            .await?;

        let service = TenantService::new(pool, cache_client);
        let result = service.get_id_by_slug("test").await;
        assert!(matches!(result, Ok(Some(_))));
        assert_eq!(result.unwrap().unwrap(), uuid);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_id_by_slug_cache_miss(pool: PgPool) -> anyhow::Result<()> {
        let cache_client = setup_mock_client();
        let service = TenantService::new(pool.clone(), cache_client);

        let test_tenant = Tenant {
            id: Uuid::new_v4(),
            name: "test".into(),
            slug: "test".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        sqlx::query!(
            "INSERT INTO tenants (id, name, slug) VALUES ($1, $2, $3)",
            &test_tenant.id,
            &test_tenant.name,
            &test_tenant.slug
        )
        .execute(&pool)
        .await?;

        let opt = service.get_id_by_slug("test").await?;
        assert!(opt.is_some());
        assert_eq!(opt.unwrap(), test_tenant.id);

        Ok(())
    }

    #[sqlx::test]
    async fn test_is_tenant_cache_hit(pool: PgPool) -> anyhow::Result<()> {
        let test_tenant = Tenant {
            id: Uuid::new_v4(),
            name: "test".into(),
            slug: "test".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let cache_client = setup_mock_client();
        let _ = cache_client
            .set::<(), _, _>(
                cache_keys::tenant_uuid(&test_tenant.id.to_string()),
                "true",
                None,
                None,
                false,
            )
            .await?;

        let service = TenantService::new(pool.clone(), cache_client);

        let opt = service.is_tenant(test_tenant.id).await?;
        assert!(opt);

        Ok(())
    }

    #[sqlx::test]
    async fn test_is_tenant_cache_miss(pool: PgPool) -> anyhow::Result<()> {
        let test_tenant = Tenant {
            id: Uuid::new_v4(),
            name: "test".into(),
            slug: "test".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let cache_client = setup_mock_client();
        let service = TenantService::new(pool.clone(), cache_client);

        let opt = service.is_tenant(test_tenant.id).await?;
        assert!(!opt);

        Ok(())
    }
}
