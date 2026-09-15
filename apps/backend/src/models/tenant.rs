use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
