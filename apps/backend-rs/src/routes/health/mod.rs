use std::sync::Arc;

use axum::{Router, routing::get};

use crate::app::AppState;

mod handler;

pub fn health_routes() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(handler::health_check))
}
