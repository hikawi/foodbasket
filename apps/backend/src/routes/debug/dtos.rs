use serde::Serialize;

use crate::routes::extract::{HostContext, PermissionsContext, SessionContext};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebugContextResponse {
    pub host: HostContext,
    pub session: SessionContext,
    pub permissions: PermissionsContext,
}
