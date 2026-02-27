#[derive(Clone)]
pub struct AppConfig {
    pub db_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::PgPool,
}

impl AppConfig {
    /// Loads the AppConfig from environment variables and fails if it can't be loaded.
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            db_url: std::env::var("DATABASE_URL")?,
        })
    }
}
