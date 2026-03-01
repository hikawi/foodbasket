use chrono::{DateTime, Utc};
use fred::prelude::Client as CacheClient;
use fred::prelude::KeysInterface;
use fred::types::Expiration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cache_keys;
use crate::services;

const SESS_CACHE_TTL: i64 = 60 * 1000;

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

pub struct SessionService {
    client: CacheClient,
}

impl From<fred::prelude::Error> for SessionServiceError {
    fn from(value: fred::prelude::Error) -> Self {
        Self::UnknownError(anyhow::Error::new(value))
    }
}

impl From<serde_json::Error> for SessionServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::UnknownError(anyhow::Error::new(value))
    }
}

impl SessionService {
    pub fn new(client: CacheClient) -> Self {
        Self { client }
    }

    /// Retrieves a session mapped to an ID.
    ///
    /// Returns Ok(Session) if the session is valid and exists.
    ///
    /// # Errors
    ///
    /// - `SessionServiceError::NoSessionFound` if the session could not be found.
    /// - `SessionServiceError::UnknownError` if the session data can not be parsed.
    pub async fn get(&self, id: &str) -> Result<Session, SessionServiceError> {
        let key = cache_keys::session(id);

        let raw_json: String = self
            .client
            .get::<Option<String>, &str>(&key)
            .await?
            .ok_or(SessionServiceError::NoSessionFound)?;

        let session = serde_json::from_str(&raw_json)?;

        Ok(session)
    }

    /// Sets a session's data after serialization. Returns `Ok(())` if succeeds.
    ///
    /// # Errors
    ///
    /// - `SessionServiceError::UnknownError` if it's a database error or can't be serialized.
    pub async fn set(&self, id: &str, sess: &Session) -> Result<(), SessionServiceError> {
        let key = cache_keys::session(id);
        let raw_json = serde_json::to_string(sess)?;

        self.client
            .set::<(), _, _>(
                &key,
                raw_json,
                Some(Expiration::EX(SESS_CACHE_TTL)),
                None,
                false,
            )
            .await?;

        Ok(())
    }

    /// Generates a random opaque token and saves the session to that as an ID.
    ///
    /// # Errors
    ///
    /// - `SessionServiceError::UnknownError` if it's a database error or can't be serialized.
    pub async fn create(&self, sess: &Session) -> Result<String, SessionServiceError> {
        let sess_id = services::random::generate_token(64);
        self.set(&sess_id, sess).await?;
        Ok(sess_id)
    }

    /// Deletes the session bound to an id.
    ///
    /// # Errors
    ///
    /// - `SessionServiceError::UnknownError` if it's a database error or can't be serialized.
    pub async fn delete(&self, id: &str) -> Result<(), SessionServiceError> {
        let key = cache_keys::session(id);
        self.client.del::<(), _>(&key).await?;
        Ok(())
    }
}
