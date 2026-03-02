use axum::{Router, routing};

use crate::app::AppState;

mod dtos;
mod handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", routing::get(handler::debug_context))
}
