use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    models::{CustomerProfile, Policy, PolicyEffect, StaffProfile, SystemProfile},
    services::Session,
};

/// Represents the type of tenant we're scoping to in our chain of handlers and middlewares.
#[derive(Debug, Clone, Copy)]
pub enum TenantContext {
    /// The tenant's slug is captured and correctly resolved.
    Tenant(Uuid),
    /// The admin panel of the platform for managing it as a whole.
    Admin,
    /// The ANY type of Tenant Context. Default fallback.
    Anonymous,
}

/// Represents the type of application we're querying for.
#[derive(Debug, Clone, Copy)]
pub enum AppContext {
    /// The sale frontpage for our tenants.
    Storefront,
    /// The POS management system for our tenants.
    Pos,
    /// Default fallback. Represents the platform's homepage or admin panel.
    None,
}

/// Represents a branch if it was resolved.
#[derive(Debug, Clone, Copy)]
pub struct BranchContext(pub Option<Uuid>);

#[derive(Debug, Clone)]
pub struct SessionContext(pub Option<Arc<Session>>);

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ProfileContext {
    /// Contains a POS Context, the UUID inside is a staff_profile_id
    Staff(Arc<StaffProfile>),
    /// Contains a customer context. The UUID inside is a customer_profile_id
    Customer(Arc<CustomerProfile>),
    /// Contains a system context. The UUID inside is a staff_profile_id
    System(Arc<SystemProfile>),
    Anonymous,
}

#[derive(Debug, Clone)]
pub struct PolicyContext(pub Option<Arc<Vec<Policy>>>);

#[derive(Debug)]
#[allow(dead_code)]
pub struct RequestContext {
    pub origin: TenantContext,
    pub session: SessionContext,
    pub profile: ProfileContext,
    pub branch: BranchContext,
    pub policies: PolicyContext,

    policies_set: HashMap<String, bool>,
}

impl RequestContext {
    pub fn new(
        origin: TenantContext,
        session: SessionContext,
        profile: ProfileContext,
        branch: BranchContext,
        policies: PolicyContext,
    ) -> Self {
        let mut map = HashMap::new();

        if let PolicyContext(Some(ref policies)) = policies {
            for policy in policies.iter() {
                let policy_doc = &policy.statements.0;
                for statement in &policy_doc.statements {
                    // Assuming statement.actions is a Vec<String>
                    let effect = match statement.effect {
                        PolicyEffect::Allow => true,
                        PolicyEffect::Deny => false,
                    };
                    for action in &statement.actions {
                        map.entry(action.clone())
                            .and_modify(|existing| *existing &= effect) // Bitwise AND assignment
                            .or_insert(effect);
                    }
                }
            }
        }

        Self {
            origin,
            session,
            profile,
            branch,
            policies,
            policies_set: map,
        }
    }

    pub fn has_permission(&self, action: &str) -> bool {
        if self.policies_set.is_empty() {
            return false;
        }

        // 1. Check for ANY explicit deny first (Specific OR Wildcard)
        // If we find a specific Deny, or a wildcard that matches and is Deny, return false.
        if let Some(false) = self.policies_set.get(action) {
            return false;
        }

        // We can't return true yet! We must check if a wildcard Deny exists.
        let mut wildcard_allow = false;
        for (perm, effect) in self.policies_set.iter() {
            if perm.ends_with('*') {
                let prefix = &perm[..perm.len() - 1];
                if action.starts_with(prefix) {
                    if !*effect {
                        return false; // Explicit wildcard deny
                    }
                    wildcard_allow = true;
                }
            }
        }

        // 2. If we reach here, no Denies exist.
        // Return true if we had a specific Allow OR a wildcard Allow.
        let specific_allow = self.policies_set.get(action).copied().unwrap_or(false);

        specific_allow || wildcard_allow
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::types::Json;

    use crate::models::{PolicyDocument, PolicyStatement};

    use super::*;

    fn mock_request_context(policy_ctx: PolicyContext) -> RequestContext {
        RequestContext::new(
            TenantContext::Anonymous,
            SessionContext(None),
            ProfileContext::Anonymous,
            BranchContext(None),
            policy_ctx,
        )
    }

    fn mock_policy(statements: Vec<PolicyStatement>) -> Policy {
        Policy {
            id: Uuid::new_v4(),
            tenant_id: Some(Uuid::new_v4()),
            branch_id: Some(Uuid::new_v4()),
            name: "test".into(),
            statements: Json(PolicyDocument {
                version: "v1".into(),
                statements,
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    fn mock_statement(effect: PolicyEffect, actions: &[&str]) -> PolicyStatement {
        PolicyStatement {
            effect,
            actions: actions.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[tokio::test]
    async fn test_explicit_allow() -> anyhow::Result<()> {
        let policy1 = mock_policy(vec![
            mock_statement(
                PolicyEffect::Allow,
                &["permissions:read", "permissions:write"],
            ),
            mock_statement(PolicyEffect::Deny, &["permissions:delete"]),
        ]);

        let policies = vec![policy1];
        let policy_ctx = PolicyContext(Some(Arc::new(policies)));
        let request_ctx = mock_request_context(policy_ctx);

        assert!(request_ctx.has_permission("permissions:read"));
        assert!(!request_ctx.has_permission("permissions:update"));

        Ok(())
    }

    #[tokio::test]
    async fn test_prioritized_deny() -> anyhow::Result<()> {
        let policy1 = mock_policy(vec![
            mock_statement(
                PolicyEffect::Allow,
                &["permissions:read", "permissions:write"],
            ),
            mock_statement(PolicyEffect::Deny, &["permissions:read"]),
        ]);

        let policies = vec![policy1];
        let policy_ctx = PolicyContext(Some(Arc::new(policies)));
        let request_ctx = mock_request_context(policy_ctx);

        assert!(request_ctx.has_permission("permissions:write"));
        assert!(!request_ctx.has_permission("permissions:read"));

        Ok(())
    }

    #[tokio::test]
    async fn test_wildcard_allow_specific_deny() -> anyhow::Result<()> {
        let policy1 = mock_policy(vec![
            mock_statement(
                PolicyEffect::Deny,
                &["permissions:read", "permissions:write"],
            ),
            mock_statement(PolicyEffect::Allow, &["permissions:*"]),
        ]);

        let policies = vec![policy1];
        let policy_ctx = PolicyContext(Some(Arc::new(policies)));
        let request_ctx = mock_request_context(policy_ctx);

        assert!(request_ctx.has_permission("permissions:anything"));
        assert!(request_ctx.has_permission("permissions:delete:what"));
        assert!(!request_ctx.has_permission("permissions:read"));

        Ok(())
    }

    #[tokio::test]
    async fn test_wildcard_deny_specific_allow() -> anyhow::Result<()> {
        let policy1 = mock_policy(vec![
            mock_statement(
                PolicyEffect::Deny,
                &["permissions:read", "permissions:write:*"],
            ),
            mock_statement(
                PolicyEffect::Allow,
                &["permissions:write", "permissions:write:123"],
            ),
        ]);

        let policies = vec![policy1];
        let policy_ctx = PolicyContext(Some(Arc::new(policies)));
        let request_ctx = mock_request_context(policy_ctx);

        assert!(request_ctx.has_permission("permissions:write"));
        assert!(!request_ctx.has_permission("permissions:write:123"));
        assert!(!request_ctx.has_permission("permissions:read"));

        Ok(())
    }
}
