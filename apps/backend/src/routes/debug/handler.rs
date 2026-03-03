use axum::{Extension, Json};

use crate::{
    api::responses::ErrorResponse,
    routes::{
        debug::dtos::DebugContextResponse,
        extract::{HostContext, PermissionsContext, SessionContext},
    },
};

/// Retrieves and checks the current context injected into the request.
#[utoipa::path(
    get, 
    path = "/debug", 
    tags = ["debug"],
    responses(
        (status = 200, description = "Successful retrieval of the user context", body = DebugContextResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn debug_context(
    Extension(host): Extension<HostContext>,
    Extension(session): Extension<SessionContext>,
    Extension(permissions): Extension<PermissionsContext>,
) -> Json<DebugContextResponse> {
    Json(DebugContextResponse {
        host,
        session,
        permissions,
    })
}
