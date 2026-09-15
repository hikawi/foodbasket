use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{Policy, PolicyDocument, PolicyEffect, PolicyStatement};

#[derive(Debug, Serialize, ToSchema)]
pub enum PolicyEffectDto {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PolicyStatementDto {
    pub effect: PolicyEffectDto,
    /// Actions like "pos:orders:create" or "inventory:*"
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PolicyDocumentDto {
    pub version: String,
    pub statements: Vec<PolicyStatementDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PolicyDto {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub name: String,
    pub statements: PolicyDocumentDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<PolicyEffect> for PolicyEffectDto {
    fn from(value: PolicyEffect) -> Self {
        match value {
            PolicyEffect::Allow => Self::Allow,
            PolicyEffect::Deny => Self::Deny,
        }
    }
}

impl From<PolicyStatement> for PolicyStatementDto {
    fn from(value: PolicyStatement) -> Self {
        Self {
            effect: value.effect.into(),
            actions: value.actions,
        }
    }
}

impl From<PolicyDocument> for PolicyDocumentDto {
    fn from(value: PolicyDocument) -> Self {
        Self {
            version: value.version,
            statements: value
                .statements
                .into_iter()
                .map(PolicyStatementDto::from)
                .collect(),
        }
    }
}

impl From<Policy> for PolicyDto {
    fn from(value: Policy) -> Self {
        Self {
            id: value.id,
            tenant_id: value.tenant_id,
            branch_id: value.branch_id,
            name: value.name,
            statements: value.statements.into_inner().into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}
