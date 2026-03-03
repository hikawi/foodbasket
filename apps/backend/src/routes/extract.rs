use std::{collections::HashSet, sync::Arc};

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::services::Session;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum HostContext {
    TenantPos(Uuid),
    TenantHome(Uuid),
    Pos,
    Admin,
    Anonymous,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum SessionContext {
    #[schema(value_type = Session)]
    Authenticated(Arc<Session>),
    Anonymous,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum PermissionsContext {
    #[schema(value_type = Vec<String>)]
    Authenticated(Arc<HashSet<String>>),
    Anonymous,
}
