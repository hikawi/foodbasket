use axum::{Router, routing::get};

use crate::app::AppState;

mod handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(handler::health_check))
}
