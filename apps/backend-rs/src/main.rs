use std::{sync::Arc, time::Duration};

use axum::Router;
use fred::prelude::{ClientLike, TcpConfig};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;

use crate::services::{SessionService, TenantService, UserService};

mod api;
mod app;
mod cache_keys;
mod error;
mod models;
mod repos;
mod routes;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Envs
    dotenvy::dotenv()?;

    // Logging
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format::json()) // The magic line
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .init();

    // Setup app state.
    let cfg = app::AppConfig::load()?;
    let pool = sqlx::PgPool::connect(&cfg.db_url).await?;
    let cache_config = fred::prelude::Config::from_url(&cfg.cache_url)?;
    let cache_client = fred::prelude::Builder::from_config(cache_config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .build()?;
    cache_client.init().await?;

    // Setup list of services.
    let session_service = SessionService::new(cache_client.clone());
    let user_service = UserService::new(pool.clone());
    let tenant_service = TenantService::new(pool.clone(), cache_client.clone());

    let state = app::AppState {
        config: Arc::new(cfg),
        db: pool,
        cache: cache_client,
        tenant_service: Arc::new(tenant_service),
        session_service: Arc::new(session_service),
        user_service: Arc::new(user_service),
    };

    let app = Router::new()
        .nest("/v1", routes::main_routes())
        .layer(CookieManagerLayer::new())
        .with_state(state);

    tracing::info!("server started on port 8080");
    axum::serve(TcpListener::bind("127.0.0.1:8080").await?, app).await?;

    Ok(())
}
