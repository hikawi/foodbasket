use std::{collections::HashSet, sync::Arc};

use serde::Serialize;
use uuid::Uuid;

use crate::services::Session;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum HostContext {
    TenantPos(Uuid),
    TenantHome(Uuid),
    Pos,
    Admin,
    Anonymous,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum SessionContext {
    Authenticated(Arc<Session>),
    Anonymous,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum PermissionsContext {
    Authenticated(Arc<HashSet<String>>),
    Anonymous,
}
