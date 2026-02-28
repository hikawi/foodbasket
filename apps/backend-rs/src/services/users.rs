use sqlx::PgPool;

use crate::{models::User, repos, services};

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
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<User, UserServiceError> {
    let user = repos::users::find_by_email(pool, email)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserServiceError::UserNotFound,
            _ => UserServiceError::UnknownError(anyhow::anyhow!(e)),
        })?;

    let hash = user
        .password
        .as_deref()
        .ok_or(UserServiceError::UserDoesNotUsePassword)?;

    if !services::passwords::verify(password, hash) {
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
    pool: &PgPool,
    name: &str,
    email: &str,
    password: Option<&str>,
) -> Result<User, UserServiceError> {
    let hashed_password = password
        .map(|p| services::passwords::hash(p))
        .transpose()
        .map_err(|e| UserServiceError::UnknownError(anyhow::anyhow!(e)))?;

    let user = repos::users::create_user(pool, name, email, hashed_password.as_deref())
        .await
        .map_err(|e| match e.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => UserServiceError::UserAlreadyExists,
            _ => UserServiceError::UnknownError(anyhow::anyhow!(e)),
        })?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_check_user_credentials_not_found(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";

        let result = check_user_credentials(&pool, test_email, raw_password).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(UserServiceError::UserNotFound)));
    }

    #[sqlx::test]
    async fn test_check_user_credentials_success(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";
        let hashed_password = services::passwords::hash(raw_password).unwrap();

        sqlx::query!(
            "INSERT INTO users (name, email, password, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())",
            "Test User",
            test_email,
            hashed_password,
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = check_user_credentials(&pool, test_email, raw_password).await;

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
            "INSERT INTO users (name, email, password, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())",
            "Test User",
            test_email,
            hashed_password,
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = register_user(&pool, "Test User", test_email, Some(raw_password)).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(UserServiceError::UserAlreadyExists)))
    }

    #[sqlx::test]
    async fn test_register_user_success(pool: sqlx::PgPool) {
        let test_email = "test@foodbasket.app";
        let raw_password = "password123";

        let result = register_user(&pool, "Test User", test_email, Some(raw_password)).await;

        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, test_email);
    }
}
