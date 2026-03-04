use fred::prelude::Client as CacheClient;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PermissionService {
    pool: PgPool,
    cache: CacheClient,
}

impl PermissionService {
    pub fn new(pool: PgPool, cache: CacheClient) -> Self {
        Self { pool, cache }
    }
}
