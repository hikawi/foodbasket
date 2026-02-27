use axum::Router;

use crate::app::AppState;

mod health;

pub fn main_routes() -> Router<AppState> {
    Router::new().merge(health::routes())
}
