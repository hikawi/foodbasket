use chrono::{DateTime, Utc};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Permission {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
