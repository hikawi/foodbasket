use fred::prelude::Client as CacheClient;

#[derive(Clone)]
pub struct AppConfig {
    pub db_url: String,
    pub cache_url: String,
    pub cookie_domain: String,
    pub cookie_secure: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::PgPool,
    pub cache: CacheClient,
}

impl AppConfig {
    /// Loads the AppConfig from environment variables and fails if it can't be loaded.
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL variable not set"),
            cache_url: std::env::var("CACHE_URL").expect("CACHE_URL variable not set"),
            cookie_domain: std::env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN variable not set"),
            cookie_secure: std::env::var("COOKIE_SECURE")
                .expect("COOKIE_SECURE variable not set")
                .parse::<bool>()?,
        })
    }
}
