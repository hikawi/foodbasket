use std::time::Duration;

use axum::Router;
use fred::prelude::{ClientLike, TcpConfig};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;

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

    let state = app::AppState {
        config: cfg,
        db: pool,
        cache: cache_client,
    };

    let app = Router::new()
        .nest("/v1", routes::main_routes())
        .layer(CookieManagerLayer::new())
        .with_state(state);

    tracing::info!("server started on port 8080");
    axum::serve(TcpListener::bind("127.0.0.1:8080").await?, app).await?;

    Ok(())
}
