use fred::{
    prelude::{Client as CacheClient, KeysInterface, SetsInterface},
    types::Expiration,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    cache_keys,
    models::{
        AssignmentProfile, PolicyDocument, PolicyEffect, PolicyStatement, ScopeType, StaffProfile,
        Tenant,
    },
    repos,
};

const TENANT_CACHE_TTL: i64 = 5 * 60;
const BLOCKED_SLUGS: [&str; 3] = ["pos", "admin", "api"];

#[derive(Debug)]
pub struct TenantService {
    pool: PgPool,
    cache: CacheClient,
}

#[derive(Debug, thiserror::Error)]
pub enum TenantServiceError {
    #[error("Uuid could not be parsed")]
    UuidError,

    #[error("That slug has been taken")]
    SlugTaken,

    #[error("That slug is not available")]
    SlugForbidden,

    #[error("There was a conflict in creating a profile")]
    Conflict,

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

    /// Checks if the provided branch belongs to the tenant.
    pub async fn is_branch_of_tenant(
        &self,
        branch_id: &Uuid,
        tenant_id: &Uuid,
    ) -> Result<bool, TenantServiceError> {
        let branch_id_str = branch_id.to_string();
        let tenant_id_str = tenant_id.to_string();
        let key = cache_keys::tenant_branches(&tenant_id_str);

        let pipeline = self.cache.pipeline();
        let _: () = pipeline.exists(&key).await?;
        let _: () = pipeline.sismember(&key, &branch_id_str).await?;
        let results = pipeline.all::<Vec<i64>>().await?;

        let group_exists = results[0] == 1;
        let is_member = results[1] == 1;

        if group_exists {
            return Ok(is_member);
        }

        // Welp. Cache miss. DB has to be hit regardless.
        let branches = repos::tenants::get_branches(&self.pool, tenant_id).await?;
        let moving_cache = self.cache.clone();
        let is_member = branches.iter().any(|v| &v.id == branch_id);

        // Set and forget.
        tokio::spawn(async move {
            let mut members: Vec<String> = branches.into_iter().map(|b| b.id.to_string()).collect();

            // ALWAYS add a sentinel so the key 'EXISTS' even if branches are empty.
            // Thank you Gemini!
            members.push("SENTINEL".into());

            let pipe = moving_cache.pipeline();
            let _: () = pipe.sadd(&key, members).await.unwrap_or_default();
            let _: () = pipe.expire(&key, 3600, None).await.unwrap_or_default();
            let _: () = pipe.all::<()>().await.unwrap_or_default();
        });

        Ok(is_member)
    }

    /// Retrieves a list of tenants for a user.
    pub async fn get_staff_tenants(
        &self,
        user_id: &Uuid,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<Tenant>, i64), TenantServiceError> {
        let offset = limit * (page - 1);
        let tenants = repos::tenants::get_staff_tenants(&self.pool, user_id, offset, limit)
            .await
            .map_err(TenantServiceError::from)?;
        let count = repos::tenants::count_staff_tenants(&self.pool, user_id)
            .await
            .map_err(TenantServiceError::from)?
            .unwrap_or(0);

        Ok((tenants, count))
    }

    /// Creates a new tenant with the name or slug and adds the user in as the only staff.
    ///
    /// # Errors
    ///
    /// - `TenantServiceError::SlugTaken` if the slug has already been taken.
    /// - `TenantServiceError::SlugForbidden` if the slug is not available.
    pub async fn create_tenant(
        &self,
        user_id: &Uuid,
        name: &str,
        slug: &str,
    ) -> Result<(Tenant, StaffProfile), TenantServiceError> {
        let slug = slug.to_lowercase();
        if BLOCKED_SLUGS.iter().any(|s| s.to_lowercase() == slug) {
            return Err(TenantServiceError::SlugForbidden);
        }

        let mut tx = self.pool.begin().await?;

        // Create a tenant.
        let tenant = repos::tenants::insert_tenant(&mut *tx, name, &slug)
            .await
            .map_err(|e| match e.as_database_error() {
                Some(db_err) if db_err.is_unique_violation() => TenantServiceError::SlugTaken,
                _ => TenantServiceError::from(e),
            })?;

        // Add a staff profile.
        let staff_profile = repos::profiles::insert_staff(&mut *tx, user_id, &tenant.id).await?;

        // Add a superadmin policy.
        let policy_doc = PolicyDocument {
            version: "1".into(),
            statements: vec![PolicyStatement {
                effect: PolicyEffect::Allow,
                actions: vec!["*".into()],
            }],
        };
        let policy = repos::policies::insert_policy(
            &mut *tx,
            Some(&tenant.id),
            None,
            "Superadmin",
            &policy_doc,
        )
        .await?;

        // Hook the staff profile with the admin policy using an assignment.
        _ = repos::assignments::insert_assignment(
            &mut *tx,
            AssignmentProfile::Staff(staff_profile.id),
            &policy.id,
            ScopeType::Tenant,
            &Uuid::nil(),
        )
        .await?;

        // Hopefully, we're done.
        tx.commit().await?;
        Ok((tenant, staff_profile))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use fred::{mocks::SimpleMap, prelude::ClientLike};
    use testcontainers::{GenericImage, core::WaitFor, runners::AsyncRunner};

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

        cache_client
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

        cache_client
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
        cache_client
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
        cache_client
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

    #[sqlx::test]
    async fn test_is_branch_success(pool: PgPool) -> anyhow::Result<()> {
        let valkey_container = GenericImage::new("valkey/valkey", "9-alpine")
            .with_exposed_port(6379.into())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await?;
        let valkey_host = valkey_container.get_host().await?;
        let valkey_port = valkey_container.get_host_port_ipv4(6379).await?;

        let cache_config =
            fred::prelude::Config::from_url(&format!("redis://{valkey_host}:{valkey_port}"))?;
        let cache_client = CacheClient::new(cache_config, None, None, None);

        let tenant_id = Uuid::new_v4();
        let branch_id = Uuid::new_v4();
        let key = cache_keys::tenant_branches(&tenant_id.to_string());
        cache_client.connect();
        cache_client.wait_for_connect().await?;
        cache_client
            .sadd::<(), _, _>(&key, vec![branch_id.to_string(), "SENTINEL".into()])
            .await?;

        let service = TenantService::new(pool, cache_client);
        assert!(matches!(
            service.is_branch_of_tenant(&branch_id, &tenant_id).await,
            Ok(true),
        ));

        Ok(())
    }

    #[sqlx::test]
    async fn test_is_branch_cache_not_found(pool: PgPool) -> anyhow::Result<()> {
        let valkey_container = GenericImage::new("valkey/valkey", "9-alpine")
            .with_exposed_port(6379.into())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await?;
        let valkey_host = valkey_container.get_host().await?;
        let valkey_port = valkey_container.get_host_port_ipv4(6379).await?;

        let cache_config =
            fred::prelude::Config::from_url(&format!("redis://{valkey_host}:{valkey_port}"))?;
        let cache_client = CacheClient::new(cache_config, None, None, None);
        cache_client.connect();
        cache_client.wait_for_connect().await?;

        let tenant_id = Uuid::new_v4();
        let branch_id = Uuid::new_v4();
        let key = cache_keys::tenant_branches(&tenant_id.to_string());
        cache_client
            .sadd::<(), _, _>(&key, vec!["SENTINEL"])
            .await?;

        let service = TenantService::new(pool, cache_client);
        assert!(matches!(
            service.is_branch_of_tenant(&branch_id, &tenant_id).await,
            Ok(false),
        ));

        Ok(())
    }

    #[sqlx::test]
    async fn test_is_branch_cache_miss(pool: PgPool) -> anyhow::Result<()> {
        let valkey_container = GenericImage::new("valkey/valkey", "9-alpine")
            .with_exposed_port(6379.into())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await?;
        let valkey_host = valkey_container.get_host().await?;
        let valkey_port = valkey_container.get_host_port_ipv4(6379).await?;

        let cache_config =
            fred::prelude::Config::from_url(&format!("redis://{valkey_host}:{valkey_port}"))?;
        let cache_client = CacheClient::new(cache_config, None, None, None);
        cache_client.connect();
        cache_client.wait_for_connect().await?;

        let tenant_id = Uuid::new_v4();
        let branch_id = Uuid::new_v4();

        let service = TenantService::new(pool, cache_client);
        assert!(matches!(
            service.is_branch_of_tenant(&branch_id, &tenant_id).await,
            Ok(false),
        ));

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_staff_tenants(pool: PgPool) -> anyhow::Result<()> {
        let test_user_id = Uuid::new_v4();
        let test_tenant_id1 = Uuid::new_v4();
        let test_tenant_id2 = Uuid::new_v4();

        sqlx::query(
            r#"
        INSERT INTO tenants (id, name, slug) VALUES ($1, 'test1', 'test1'), ($2, 'test2', 'test2');
        "#,
        )
        .bind(test_tenant_id1)
        .bind(test_tenant_id2)
        .bind(test_user_id)
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        INSERT INTO users (id, email, password) VALUES ($3, 'test@foodbasket.app', 'idchash');
        "#,
        )
        .bind(test_tenant_id1)
        .bind(test_tenant_id2)
        .bind(test_user_id)
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        INSERT INTO staff_profiles (user_id, tenant_id) VALUES ($3, $1);
        "#,
        )
        .bind(test_tenant_id1)
        .bind(test_tenant_id2)
        .bind(test_user_id)
        .execute(&pool)
        .await?;

        let cache_client = setup_mock_client();
        let service = TenantService::new(pool.clone(), cache_client);

        let (tenants, count) = service.get_staff_tenants(&test_user_id, 1, 5).await?;
        assert_eq!(tenants.len(), 1);
        assert_eq!(tenants[0].name, "test1");
        assert_eq!(count, 1);

        Ok(())
    }
}
