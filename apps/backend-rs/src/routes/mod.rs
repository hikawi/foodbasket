use axum::Router;

use crate::app::AppState;

pub mod auth;
pub mod health;

pub fn main_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .nest("/auth", auth::routes())
}
