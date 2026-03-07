use std::sync::Arc;

use axum::extract::FromRef;
use fred::prelude::Client as CacheClient;
use sqlx::PgPool;

use crate::services::{PolicyService, ProfileService, SessionService, TenantService, UserService};

pub struct AppConfig {
    pub db_url: String,
    pub cache_url: String,
    pub cookie_domain: String,
    pub cookie_secure: bool,
}

#[derive(Clone)]
pub struct AppServices {
    pub sessions: Arc<SessionService>,
    pub tenants: Arc<TenantService>,
    pub users: Arc<UserService>,
    pub profiles: Arc<ProfileService>,
    pub policies: Arc<PolicyService>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
    pub cache: CacheClient,
    pub services: AppServices,
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

impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}

impl FromRef<AppState> for CacheClient {
    fn from_ref(input: &AppState) -> Self {
        input.cache.clone()
    }
}

impl FromRef<AppState> for Arc<TenantService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.tenants.clone()
    }
}

impl FromRef<AppState> for Arc<SessionService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.sessions.clone()
    }
}
impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.users.clone()
    }
}

impl FromRef<AppState> for Arc<PolicyService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.policies.clone()
    }
}

impl FromRef<AppState> for Arc<ProfileService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.profiles.clone()
    }
}
