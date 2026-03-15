use axum::{Router, middleware};
use tower_cookies::CookieManagerLayer;

use crate::app::AppState;

pub mod auth;
pub mod extract;
pub mod health;
pub mod middlewares;
pub mod staff;
pub mod tenants;

pub fn main_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/health", health::routes())
        .nest("/tenants", tenants::routes())
        .nest("/staff", staff::routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::context_solidify,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::policy_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::branch_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::profile_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::session_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::app_hydrate,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::tenant_hydrate,
        ))
        .layer(CookieManagerLayer::new())
}
