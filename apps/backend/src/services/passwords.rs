use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

#[derive(thiserror::Error, Debug)]
pub enum PasswordServiceError {
    #[error("Failed to hash password")]
    FailedToHash,
}

/// Hashes a plain-text password using Argon2id
pub fn hash(password: &str) -> Result<String, PasswordServiceError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordServiceError::FailedToHash)?
        .to_string();

    Ok(password_hash)
}

/// Verifies a plain-text password against a stored hash
pub fn verify(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_success() {
        let password = "my_super_secret_password";
        let hash = hash(password).expect("Hashing should not fail");
        assert!(verify(password, &hash));
    }

    #[test]
    fn test_verify_failure() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let hash = hash(password).expect("Hashing should not fail");
        assert!(!verify(wrong_password, &hash));
    }

    #[test]
    fn test_unique_salts() {
        let password = "same_password";

        let hash1 = hash(password).unwrap();
        let hash2 = hash(password).unwrap();

        assert_ne!(hash1, hash2);
    }
}
