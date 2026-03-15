use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::User,
    repos,
    services::{self, passwords::PasswordServiceError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("User can not be found")]
    UserNotFound,

    #[error("User does not use password")]
    UserDoesNotUsePassword,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Wrong password")]
    WrongPassword,

    #[error("Unknown error: {0}")]
    UnknownError(#[from] anyhow::Error),
}

pub struct UserService {
    pool: PgPool,
}

impl From<sqlx::Error> for UserServiceError {
    fn from(value: sqlx::Error) -> Self {
        match value.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => Self::UserAlreadyExists,
            _ => Self::UnknownError(anyhow::Error::new(value)),
        }
    }
}

impl From<PasswordServiceError> for UserServiceError {
    fn from(value: PasswordServiceError) -> Self {
        Self::UnknownError(anyhow::Error::new(value))
    }
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Checks a user's inputted credentials and returns an `Ok(User)`
    /// if all checks passed.
    ///
    /// # Errors
    ///
    /// - `UserServiceError::UserNotFound` if the email could not be found, or deleted.
    /// - `UserServiceError::UserDoesNotUsePassword` if the user does not use password to authenticate.
    /// - `UserServiceError::WrongPassword` if the passwords didn't match what it verified.
    /// - `UserServiceError::UnknownError` if there was another unknown error.
    pub async fn check_user_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, UserServiceError> {
        let user = repos::users::find_by_email(&self.pool, email)
            .await?
            .ok_or(UserServiceError::UserNotFound)?;

        if !services::passwords::verify(password, &user.password) {
            return Err(UserServiceError::WrongPassword);
        }

        Ok(user)
    }

    /// Registers a new user and returns an `Ok(User)` if it succeeded.
    ///
    /// # Errors
    ///
    /// - `UserServiceError::UserAlreadyExists` if the user already existed.
    /// - `UserServiceError::UnknownError` if there was an unexpected error.
    pub async fn register_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, UserServiceError> {
        let hashed = services::passwords::hash(password)?;
        Ok(repos::users::create_user(&self.pool, email, &hashed).await?)
    }

    /// Retrieves a user by ID.
    ///
    /// # Errors
    ///
    /// - `UserServiceError::UserNotFound` if the email could not be found, or deleted.
    /// - `UserServiceError::UnknownError` if there was an unexpected error.
    pub async fn get_user(&self, user_id: &Uuid) -> Result<User, UserServiceError> {
        repos::users::find_by_id(&self.pool, user_id)
            .await
            .map_err(UserServiceError::from)?
            .ok_or(UserServiceError::UserNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_check_user_credentials_not_found(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";

        let svc = UserService::new(pool);
        let result = svc.check_user_credentials(test_email, raw_password).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(UserServiceError::UserNotFound)));
    }

    #[sqlx::test]
    async fn test_check_user_credentials_success(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";
        let hashed_password = services::passwords::hash(raw_password).unwrap();

        sqlx::query!(
            "INSERT INTO users (email, password, created_at, updated_at)
             VALUES ($1, $2, NOW(), NOW())",
            test_email,
            hashed_password,
        )
        .execute(&pool)
        .await
        .unwrap();

        let svc = UserService::new(pool);
        let result = svc.check_user_credentials(test_email, raw_password).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, test_email);
    }

    #[sqlx::test]
    async fn test_register_user_conflict(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";
        let hashed_password = services::passwords::hash(raw_password).unwrap();

        sqlx::query!(
            "INSERT INTO users (email, password, created_at, updated_at)
             VALUES ($1, $2, NOW(), NOW())",
            test_email,
            hashed_password,
        )
        .execute(&pool)
        .await
        .unwrap();

        let svc = UserService::new(pool);
        let result = svc.register_user(test_email, raw_password).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(UserServiceError::UserAlreadyExists)))
    }

    #[sqlx::test]
    async fn test_register_user_success(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";

        let svc = UserService::new(pool);
        let result = svc.register_user(test_email, raw_password).await;

        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, test_email);
    }
}
