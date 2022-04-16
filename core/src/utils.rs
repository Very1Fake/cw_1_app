use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, Result, SaltString},
    Argon2,
};

pub fn password_hash(password: &[u8]) -> Result<String> {
    Ok(Argon2::default()
        .hash_password(password, &SaltString::generate(&mut OsRng))?
        .to_string())
}
