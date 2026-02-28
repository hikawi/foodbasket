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
