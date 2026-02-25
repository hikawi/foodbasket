use sqlx::PgPool;
use thiserror::Error;

use crate::{models::User, repo, services};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User could not be found")]
    NotFound,
    #[error("User does not login with password")]
    DoesNotHavePassword,
    #[error("Wrong password")]
    WrongPassword,
    #[error("User already exists")]
    AlreadyExists,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// Checks a user credentials using an email and a password.
///
/// Returns `Ok(())` if user exists and matches.
///
/// # Errors
///
/// - `UserError::NotFound` if the email doesn't exist.
/// - `UserError::DoesNotHavePassword` if the user doesn't use password to login.
/// - `UserError::WrongPassword` if the user exists, but passwords didn't match.
/// - `UserError::UnexpectedError(e)` if the db failed.
pub async fn check_user_credentials(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<(), UserError> {
    let user = repo::users::find_by_email(pool, email)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::UnexpectedError(e.into()),
        })?;

    let pass = user.password.ok_or(UserError::DoesNotHavePassword)?;
    let ok = services::passwords::verify(password, &pass).map_err(UserError::UnexpectedError)?;
    if !ok {
        return Err(UserError::WrongPassword);
    }

    Ok(())
}

/// Registers a new user.
///
/// Returns Ok(User) if it succeeded.
///
/// # Errors
///
/// - `UserError::AlreadyExists` if the user email was already taken.
/// - `UserError::UnexpectedError(e)` if there was a db error.
pub async fn register_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, UserError> {
    let hashed_password = services::passwords::hash(password)?;

    repo::users::create_user(pool, name, email, &hashed_password)
        .await
        .map_err(|e| match e.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => UserError::AlreadyExists,
            _ => UserError::UnexpectedError(e.into()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_register_user_success(pool: PgPool) {
        let name = "Sakura";
        let email = "sakura@foodbasket.app";
        let pass = "password123";

        let result = register_user(&pool, name, email, pass).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, email);
        assert!(user.password.is_some());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_register_duplicate_email(pool: PgPool) {
        let email = "dup@foodbasket.app";
        register_user(&pool, "User 1", email, "pass").await.unwrap();
        let result = register_user(&pool, "User 2", email, "pass").await;
        assert!(matches!(result, Err(UserError::AlreadyExists)));
    }
}
