use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum PolicyEffect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyStatement {
    pub effect: PolicyEffect,
    /// Actions like "pos:orders:create" or "inventory:*"
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyDocument {
    pub version: String,
    pub statements: Vec<PolicyStatement>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Policy {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub name: String,
    pub statements: Json<PolicyDocument>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
