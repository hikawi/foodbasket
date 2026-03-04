use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "scope_type", rename_all = "lowercase")]
pub enum ScopeType {
    System,
    Tenant,
    Branch,
}
