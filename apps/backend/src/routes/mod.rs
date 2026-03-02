use axum::{Router, middleware};

use crate::app::AppState;

pub mod auth;
pub mod debug;
pub mod extract;
pub mod health;
pub mod middlewares;

pub fn main_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .nest("/auth", auth::routes())
        .nest("/debug", debug::routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::permission_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::session_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::host_hydrate,
        ))
        .layer(middleware::from_fn(middlewares::dynamic_cors))
}
