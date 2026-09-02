use serde::{Deserialize, Serialize};

use crate::routes::extract::AppContext;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum AppEvent {
    Pong,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventTarget {
    pub tenant_id: String,
    pub branch_id: Option<String>,
    pub app_context: AppContext,
    pub permission_matcher: Option<String>,
}

/// Provides an event that is scoped to a tenant or a branch.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScopedEvent {
    pub target: EventTarget,
    pub payload: AppEvent,
}
