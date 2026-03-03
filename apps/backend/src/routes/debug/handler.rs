use axum::{Extension, Json};

use crate::{
    api::responses::ErrorResponse,
    routes::{
        debug::dtos::DebugContextResponse,
        extract::{OriginContext, PermissionsContext, SessionContext},
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
    Extension(origin): Extension<OriginContext>,
    Extension(session): Extension<SessionContext>,
    Extension(permissions): Extension<PermissionsContext>,
) -> Json<DebugContextResponse> {
    Json(DebugContextResponse {
        origin,
        session,
        permissions,
    })
}
