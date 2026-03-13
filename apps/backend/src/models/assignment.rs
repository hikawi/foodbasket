use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::ScopeType;

#[allow(dead_code)]
pub enum AssignmentProfile {
    Staff(Uuid),
    Customer(Uuid),
    System(Uuid),
    Invalid,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Assignment {
    pub id: Uuid,
    pub staff_profile_id: Option<Uuid>,
    pub customer_profile_id: Option<Uuid>,
    pub system_profile_id: Option<Uuid>,
    pub policy_id: Uuid,
    pub scope_type: ScopeType,
    pub scope_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl Assignment {
    /// Calculates the type of the assignment. One of `staff_profile_id` or `customer_profile_id`
    /// or `system_profile_id` has to be non-null while others are null for a valid assignment
    /// profile.
    fn get_type(&self) -> AssignmentProfile {
        match (
            self.staff_profile_id,
            self.customer_profile_id,
            self.system_profile_id,
        ) {
            (Some(uuid), None, None) => AssignmentProfile::Staff(uuid),
            (None, Some(uuid), None) => AssignmentProfile::Customer(uuid),
            (None, None, Some(uuid)) => AssignmentProfile::System(uuid),
            _ => AssignmentProfile::Invalid,
        }
    }
}
