use std::sync::Arc;

use axum::Router;

use crate::app;

mod health;

pub fn create_router() -> Router<Arc<app::AppState>> {
    Router::new().merge(health::health_routes())
}
