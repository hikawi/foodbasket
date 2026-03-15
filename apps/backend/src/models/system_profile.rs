use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SystemProfile {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
