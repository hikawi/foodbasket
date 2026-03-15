use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct StaffProfile {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub pin_code: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
