use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::{Assignment, AssignmentProfile, ScopeType};

pub async fn insert_assignment(
    executor: impl PgExecutor<'_>,
    assignment_profile: AssignmentProfile,
    policy_id: &Uuid,
    scope_type: ScopeType,
    scope_id: &Uuid,
) -> Result<Assignment, sqlx::Error> {
    let (staff, customer, system) = match assignment_profile {
        AssignmentProfile::Staff(uuid) => (Some(uuid), None, None),
        AssignmentProfile::Customer(uuid) => (None, Some(uuid), None),
        AssignmentProfile::System(uuid) => (None, None, Some(uuid)),
        _ => Err(sqlx::Error::Decode(
            "Invalid AssignmentProfile variant".into(),
        ))?,
    };

    sqlx::query_as!(
        Assignment,
        r#"
    INSERT INTO assignments (staff_profile_id, customer_profile_id, system_profile_id, policy_id, scope_type, scope_id)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING id, staff_profile_id, customer_profile_id, system_profile_id, policy_id, scope_type as "scope_type: ScopeType", scope_id, created_at
    "#,
        staff,
        customer,
        system,
        policy_id,
        scope_type as _,
        scope_id
    ).fetch_one(executor).await
}
