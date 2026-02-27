use axum::Router;
use tokio::net::TcpListener;

mod api;
mod app;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Envs
    dotenvy::dotenv()?;

    // Logging
    tracing_subscriber::fmt::init();

    // Setup app state.
    let cfg = app::AppConfig::load()?;
    let pool = sqlx::PgPool::connect(&cfg.db_url).await?;

    let state = app::AppState {
        config: cfg,
        db: pool,
    };

    let app = Router::new()
        .nest("/v1", routes::main_routes())
        .with_state(state);

    tracing::info!("Server started on hehe :8080");
    axum::serve(TcpListener::bind("127.0.0.1:8080").await?, app).await?;

    Ok(())
}
