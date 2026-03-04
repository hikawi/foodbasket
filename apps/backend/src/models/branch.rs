use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Branch {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
