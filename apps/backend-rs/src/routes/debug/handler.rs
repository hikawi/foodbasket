use axum::{Extension, Json};

use crate::routes::{
    debug::dtos::DebugContextResponse,
    extract::{HostContext, PermissionsContext, SessionContext},
};

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
