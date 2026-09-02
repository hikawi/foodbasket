use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    Extension, Json,
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use fred::interfaces::PubsubInterface;
use futures_util::stream::Stream;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::{
    api::responses::{ErrorResponse, MessageResponse},
    app::AppState,
    cache_keys,
    error::AppError,
    event::{AppEvent, EventTarget, ScopedEvent},
    routes::{
        extract::{AppContext, ProfileContext, RequestContext, TenantContext},
        sse::SseError,
    },
};

/// Subscribess to the app's broadcast receiver for receiving future updates.
#[utoipa::path(
    get,
    path = "/sse",
    tag = "sse",
    security(("session_id" = []), ("branch_id" = []), ("tenant_slug" = []), ("app_context" = [])),
    responses(
        (status = 200, description = "Successful connection"),
    ),
)]
pub async fn sse_handler(
    State(state): State<AppState>,
    Extension(context): Extension<Arc<RequestContext>>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // Make sure they're authenticated.
    if context.session.0.is_none()
        || context
            .session
            .0
            .clone()
            .is_some_and(|s| s.user_id.is_none())
    {
        Err(SseError::Unauthorized("Unauthorized".into()))?
    }

    let rx = state.broadcast_sender.subscribe();

    // Convert Tokio broadcast receiver into a Stream
    let stream_ctx = context.clone();
    let stream = BroadcastStream::new(rx).filter_map(move |res| match res {
        Ok(event) => {
            // Check if correct tenant.
            let tenant_id = match stream_ctx.origin {
                TenantContext::Anonymous => "anonymous".into(),
                TenantContext::Admin => "admin".into(),
                TenantContext::Tenant(uuid) => uuid.to_string(),
            };
            if &tenant_id != &event.target.tenant_id {
                return None;
            }

            // Check if it needs a branch match.
            // If the current context is a branch, then another branch will fail.
            // (Some, None) => Works (a broadcast to all will match regardless)
            // (None, Some) => Always (a catch all will match regardless)
            // (Some, Some) => if a != b
            let current_branch = stream_ctx.branch.0.map(|u| u.to_string());
            let target_branch = &event.target.branch_id;
            if let (Some(b), Some(t)) = (&current_branch, target_branch) {
                if b != t {
                    return None;
                }
            }

            // Check for app context matches.
            let required_app_context = match stream_ctx.profile {
                ProfileContext::System(_) => AppContext::None,
                ProfileContext::Customer(_) => AppContext::Storefront,
                ProfileContext::Staff(_) => AppContext::Pos,
                ProfileContext::Anonymous => return None,
            };
            let is_app_context_allowed = match (&required_app_context, &event.target.app_context) {
                // If either listener OR event is AppContext::None, allow it
                (AppContext::None, _) | (_, AppContext::None) => true,
                (user_ctx, event_ctx) => user_ctx == event_ctx,
            };

            if !is_app_context_allowed {
                return None;
            }

            // Can't see if no permissions. Most time-consuming check.
            if let Some(perm) = &event.target.permission_matcher {
                if !stream_ctx.has_permission(perm) {
                    return None;
                }
            }

            let json = serde_json::to_string(&event).ok()?;
            Some(Ok(Event::default().data(json)))
        }
        Err(_) => None,
    });

    tracing::info!(route = "/sse", status = 200);
    Ok(Sse::new(stream).keep_alive(KeepAlive::default().interval(Duration::from_secs(15))))
}

/// Makes a request to the SSE sender to send a PONG message. Fires the PONG with the
/// exact same target as the provided session.
#[utoipa::path(
    post,
    path = "/sse/ping",
    tag = "sse",
    security(("session_id" = []), ("branch_id" = []), ("tenant_slug" = []), ("app_context" = [])),
    responses(
        (status = 200, description = "Successfully published an SSE event", body = MessageResponse),
        (status = 500, description = "Failed to publish to Valkey", body = ErrorResponse),
    ),
)]
pub async fn sse_ping(
    State(state): State<AppState>,
    Extension(context): Extension<Arc<RequestContext>>,
) -> Result<Json<MessageResponse>, AppError> {
    let tenant_id = match context.origin {
        TenantContext::Admin => "admin".into(),
        TenantContext::Anonymous => "anonymous".into(),
        TenantContext::Tenant(id) => id.to_string(),
    };
    let branch_id = context.branch.0.map(|u| u.to_string());

    let e = ScopedEvent {
        target: EventTarget {
            tenant_id: tenant_id.clone(),
            branch_id,
            app_context: context.app,
            permission_matcher: Some("*".into()),
        },
        payload: AppEvent::Pong,
    };

    let channel = cache_keys::tenant_sse(&tenant_id);
    state
        .cache
        .publish::<(), _, _>(
            channel,
            serde_json::to_vec(&e).map_err(|e| SseError::Internal(e.to_string()))?,
        )
        .await
        .map_err(|e| SseError::Internal(e.to_string()))?;

    Ok(Json(MessageResponse {
        message: "Pong!".into(),
    }))
}
