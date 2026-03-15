use axum::{Router, routing::get};

use crate::app::AppState;

pub mod handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler::health_check))
}
