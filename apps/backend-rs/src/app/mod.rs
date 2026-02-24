use anyhow::Context;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub postgres_url: String,
}

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::PgPool,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let postgres_url =
            env::var("POSTGRES_URL").context("POSTGRES_URL must be set in the environment")?;
        Ok(Self { postgres_url })
    }
}
