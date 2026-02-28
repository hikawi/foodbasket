use chrono::{DateTime, Utc};
use fred::prelude::Client as CacheClient;
use fred::prelude::KeysInterface;
use fred::types::Expiration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cache_keys;
use crate::services;

#[derive(thiserror::Error, Debug)]
pub enum SessionServiceError {
    #[error("No session found")]
    NoSessionFound,
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
    pub created_at: DateTime<Utc>,
}

const SESS_CACHE_TTL: i64 = 60 * 1000;

pub async fn get(client: &CacheClient, id: &str) -> Result<Session, SessionServiceError> {
    let key = cache_keys::session(id);

    let raw_json: String = client
        .get::<Option<String>, &str>(&key)
        .await
        .map_err(anyhow::Error::from)?
        .ok_or(SessionServiceError::NoSessionFound)?;

    let session = serde_json::from_str(&raw_json).map_err(anyhow::Error::from)?;

    Ok(session)
}

pub async fn set(
    client: &CacheClient,
    id: &str,
    sess: &Session,
) -> Result<(), SessionServiceError> {
    let key = cache_keys::session(id);
    let raw_json = serde_json::to_string(sess).map_err(anyhow::Error::from)?;

    client
        .set::<(), &str, String>(
            &key,
            raw_json,
            Some(Expiration::EX(SESS_CACHE_TTL)),
            None,
            false,
        )
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}

pub async fn create(client: &CacheClient, sess: &Session) -> Result<String, SessionServiceError> {
    let sess_id = services::random::generate_token(64);
    set(client, &sess_id, sess).await?;
    Ok(sess_id)
}

pub async fn delete(client: &CacheClient, id: &str) -> Result<(), SessionServiceError> {
    let key = cache_keys::session(id);

    client
        .del::<(), &str>(&key)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
