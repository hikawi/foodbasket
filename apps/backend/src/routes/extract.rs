use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    models::{CustomerProfile, Policy, PolicyEffect, StaffProfile, SystemProfile},
    services::Session,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OriginUrl {
    Valid(String),
    Invalid,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OriginContext {
    TenantPos(Uuid),
    TenantHome(Uuid),
    Pos,
    Admin,
    Anonymous,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BranchContext {
    Branch(Uuid),
    Anonymous,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SessionContext {
    Authenticated(Arc<Session>),
    Anonymous,
}

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
#[allow(dead_code)]
pub enum PolicyContext {
    Authenticated(Arc<Vec<Policy>>),
    Anonymous,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RequestContext {
    pub origin: OriginContext,
    pub session: SessionContext,
    pub profile: ProfileContext,
    pub branch: BranchContext,
    pub policies: PolicyContext,

    policies_set: HashMap<String, bool>,
}

impl RequestContext {
    pub fn new(
        origin: OriginContext,
        session: SessionContext,
        profile: ProfileContext,
        branch: BranchContext,
        policies: PolicyContext,
    ) -> Self {
        let mut map = HashMap::new();

        if let PolicyContext::Authenticated(ref policies) = policies {
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

    /// Checks if the current context has permission to perform a specific action.
    /// Follows the logic: Explicit Deny > Explicit Allow > Implicit Deny.
    #[allow(dead_code)]
    pub fn has_permission(&self, action: &str) -> bool {
        // 1. Look up the action in our pre-calculated map.
        // .get() returns Option<&bool>.
        // We map that to a simple bool, defaulting to 'false' if the key doesn't exist.
        match self.policies_set.get(action) {
            Some(allowed) => *allowed, // Returns true if Allow, false if Deny
            None => {
                // 2. Optional: Check for global wildcards if the specific action wasn't found.
                // This allows a "pos:*" permission to cover "pos:orders:create".
                self.check_wildcards(action)
            }
        }
    }

    /// Helper to handle pattern matching (e.g., "pos:*" or "*")
    fn check_wildcards(&self, action: &str) -> bool {
        let mut allowed = false;

        for (perm, effect) in self.policies_set.iter() {
            if perm.ends_with('*') {
                let prefix = &perm[..perm.len() - 1];
                if action.starts_with(prefix) {
                    // If we find an explicit Deny wildcard, return false immediately (highest priority)
                    if !*effect {
                        return false;
                    }
                    // If we find an Allow wildcard, note it but keep looking for potential Denies
                    allowed = true;
                }
            }
        }

        allowed
    }
}
