use std::sync::Arc;

use axum::{Router, routing::post};

use crate::app::AppState;

mod dto;
mod handler;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(handler::login))
        .route("/register", post(handler::register))
}
