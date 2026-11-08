use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

pub fn hash_password(plain: &str) -> Result<String, argon2::password_hash::Error> {
    // Argon2::default() == Argon2id v19, m=19456 KiB, t=2, p=1 (OWASP minimum)
    Argon2::default()
        .hash_password(plain.as_bytes())
        .map(|h| h.to_string())
}

pub fn check_password(hash: &str, plain: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
}
