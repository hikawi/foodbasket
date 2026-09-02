use std::{sync::Arc, time::Duration};

use axum::Router;
use fred::{
    clients::SubscriberClient,
    interfaces::{EventInterface, PubsubInterface},
    prelude::{ClientLike, TcpConfig},
    types::Builder,
};
use http::{Method, header};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    event::ScopedEvent,
    services::{PolicyService, ProfileService, SessionService, TenantService, UserService},
};

mod api;
mod app;
mod cache_keys;
mod docs;
mod error;
mod event;
mod models;
mod permissions;
mod repos;
mod routes;
mod services;

pub async fn start_valkey_listener(
    subscriber: SubscriberClient,
    tx: broadcast::Sender<ScopedEvent>,
) {
    tokio::spawn(async move {
        if let Err(e) = subscriber.init().await {
            tracing::error!("Failed to initialize Valkey pubsub: {:?}", e);
            return;
        }

        if let Err(e) = subscriber.psubscribe("foodbasket:tenants:*:sse").await {
            tracing::error!("Failed to subscribe to tenants: {:?}", e);
            return;
        }

        tracing::info!("starting valkey listener loop");
        let mut message_stream = subscriber.message_rx();
        while let Ok(message) = message_stream.recv().await {
            if let Ok(payload_str) = message.value.convert::<String>() {
                tracing::debug!(type = "event", event = %payload_str);
                if let Ok(event) = serde_json::from_str::<ScopedEvent>(&payload_str) {
                    let _ = tx.send(event);
                }
            }
        }
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Envs
    let _ = dotenvy::dotenv();

    // Logging
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Setup app state.
    let cfg = app::AppConfig::load()?;
    let pool = sqlx::PgPool::connect(&cfg.db_url).await?;
    let cache_config = fred::prelude::Config::from_url(&cfg.cache_url)?;
    let cache_client = fred::prelude::Builder::from_config(cache_config.clone())
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .build()?;
    cache_client.init().await?;

    // Migration, maybe.
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Setup list of services.
    let session_service = SessionService::new(cache_client.clone());
    let user_service = UserService::new(pool.clone());
    let tenant_service = TenantService::new(pool.clone(), cache_client.clone());
    let profile_service = ProfileService::new(pool.clone(), cache_client.clone());
    let policy_service = PolicyService::new(pool.clone(), cache_client.clone());

    // SSE holders
    let (sse_sender, _) = broadcast::channel::<ScopedEvent>(100);

    let state = app::AppState {
        config: Arc::new(cfg),
        db: pool,
        broadcast_sender: sse_sender.clone(),
        cache: cache_client,
        services: app::AppServices {
            tenants: Arc::new(tenant_service),
            sessions: Arc::new(session_service),
            users: Arc::new(user_service),
            profiles: Arc::new(profile_service),
            policies: Arc::new(policy_service),
        },
    };

    // Setup CORS layer.
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-tenant-slug"),
            header::HeaderName::from_static("x-branch-id"),
            header::HeaderName::from_static("x-app-context"),
        ])
        .allow_origin(AllowOrigin::predicate(
            move |origin: &header::HeaderValue, _config: &axum::http::request::Parts| {
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.ends_with(".foodbasket.app")
                    || origin_str == "https://foodbasket.app"
                    || origin_str.ends_with(".foodbasket.localhost")
                    || origin_str == "http://foodbasket.localhost"
            },
        ));

    let app = Router::new()
        .nest("/v1", routes::main_routes(state.clone()))
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDocs::openapi()))
        .layer(cors)
        .with_state(state);

    // Spawn background stuff for valkey thing.
    let subscriber_client = Builder::from_config(cache_config)
        .build_subscriber_client()
        .expect("Failed to build Valkey subscriber client");
    start_valkey_listener(subscriber_client, sse_sender).await;

    tracing::info!("server started on port 8080");
    axum::serve(TcpListener::bind("0.0.0.0:8080").await?, app).await?;

    Ok(())
}
