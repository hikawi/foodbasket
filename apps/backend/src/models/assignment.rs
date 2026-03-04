use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::ScopeType;

#[derive(Debug)]
pub struct Assignment {
    pub id: Uuid,
    pub staff_profile_id: Uuid,
    pub policy_id: Uuid,
    pub scope_type: ScopeType,
    pub scope_id: Uuid,
    pub created_at: DateTime<Utc>,
}
