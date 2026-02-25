use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Apparently you can't .context here because that's a wildly different error type.
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("hashing failed: {}", e))?
        .to_string();

    Ok(password_hash)
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
    let pw_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e))?;
    match Argon2::default().verify_password(password.as_bytes(), &pw_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_hash_any() {
        let hash = hash("←↑→↓これを読まないでくれ");
        assert!(hash.is_ok())
    }

    #[test]
    fn test_verify_correct() {
        let pw = "password";
        let hash = hash(&pw);

        assert!(hash.is_ok());
        assert!(verify(pw, &hash.unwrap()).unwrap());
    }

    #[test]
    fn test_verify_incorrect() {
        let pw = "password";
        let hash = hash(&pw);

        assert!(hash.is_ok());
        assert!(!verify("password1", &hash.unwrap()).unwrap());
    }
}
