use serde::Serialize;
use utoipa::ToSchema;

use crate::routes::extract::{HostContext, PermissionsContext, SessionContext};

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DebugContextResponse {
    pub host: HostContext,
    pub session: SessionContext,
    pub permissions: PermissionsContext,
}
