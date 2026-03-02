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
    #[error("Invalid JSON format for session: {0}")]
    MalformedJson(#[from] serde_json::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
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
        Self::Unknown(anyhow::Error::new(value))
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use fred::{
        mocks::SimpleMap,
        prelude::{Client, ClientLike},
    };

    fn setup_mock_client() -> Client {
        let mock_cache = SimpleMap::new();

        let config = fred::prelude::Config {
            mocks: Some(Arc::new(mock_cache)),
            ..fred::prelude::Config::default()
        };
        let cache_client = fred::prelude::Client::new(config, None, None, None);
        cache_client.connect();

        cache_client
    }

    #[tokio::test]
    async fn test_get_no_session() -> anyhow::Result<()> {
        let client = setup_mock_client();

        let service = SessionService::new(client);
        let session = service.get("test").await;
        assert!(matches!(session, Err(SessionServiceError::NoSessionFound)));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_session_bad_json() -> anyhow::Result<()> {
        let client = setup_mock_client();
        let _ = client
            .set::<(), _, _>(cache_keys::session("test"), "{\"wee}", None, None, false)
            .await;

        let service = SessionService::new(client);
        let session = service.get("test").await;
        assert!(matches!(
            session,
            Err(SessionServiceError::MalformedJson(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_session_success() -> anyhow::Result<()> {
        let client = setup_mock_client();
        let _ = client
            .set::<(), _, _>(
                cache_keys::session("test"),
                r#"{"userId":null,"userEmail":"testemail@foodbasket.app","createdAt":"2026-03-02T14:30:54.718755750Z"}"#,
                None,
                None,
                false,
            )
            .await;

        let service = SessionService::new(client);
        let session = service.get("test").await;
        assert!(matches!(session, Ok(_)));
        assert_eq!(
            session.unwrap().user_email.unwrap(),
            "testemail@foodbasket.app"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_set_session_success() -> anyhow::Result<()> {
        let client = setup_mock_client();
        let session = Session {
            user_id: Some(Uuid::new_v4()),
            user_email: Some("test@foodbasket.app".into()),
            created_at: Utc::now(),
        };

        let service = SessionService::new(client);
        service.set("test", &session).await?;
        let returned_sess = service.get("test").await?;

        assert_eq!(returned_sess.user_id, session.user_id);
        assert_eq!(returned_sess.user_email, session.user_email);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_session_success() -> anyhow::Result<()> {
        let client = setup_mock_client();
        let session = Session {
            user_id: Some(Uuid::new_v4()),
            user_email: Some("test@foodbasket.app".into()),
            created_at: Utc::now(),
        };

        let service = SessionService::new(client);
        let id = service.create(&session).await?;
        let returned_sess = service.get(&id).await?;

        assert_eq!(returned_sess.user_id, session.user_id);
        assert_eq!(returned_sess.user_email, session.user_email);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_session_success() -> anyhow::Result<()> {
        let client = setup_mock_client();
        let _ = client
            .set::<(), _, _>(
                cache_keys::session("test"),
                r#"{"userId":null,"userEmail":"testemail@foodbasket.app","createdAt":"2026-03-02T14:30:54.718755750Z"}"#,
                None,
                None,
                false,
            )
            .await;

        let service = SessionService::new(client);
        service.delete("test").await?;
        let sess = service.get("test").await;

        assert!(matches!(sess, Err(SessionServiceError::NoSessionFound)));

        Ok(())
    }
}
