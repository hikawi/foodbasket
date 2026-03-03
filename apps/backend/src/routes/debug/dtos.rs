use serde::Serialize;
use utoipa::ToSchema;

use crate::routes::extract::{OriginContext, PermissionsContext, SessionContext};

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DebugContextResponse {
    pub origin: OriginContext,
    pub session: SessionContext,
    pub permissions: PermissionsContext,
}
