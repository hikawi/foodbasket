use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Permission {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
