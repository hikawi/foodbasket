mod app;
mod models;
mod repo;
mod routes;

use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv()?;

    // Load the stupid stuff up
    let config = app::AppConfig::load()?;
    let pool = sqlx::PgPool::connect(&config.postgres_url).await?;

    let state = Arc::new(app::AppState {
        config: config,
        db: pool,
    });
    let app = Router::new()
        .nest("/v1", routes::create_router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;

    Ok(())
}
