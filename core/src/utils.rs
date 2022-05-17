use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, Result, SaltString},
    Argon2,
};
use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

pub type PgQuery = Query<'static, Postgres, PgArguments>;
pub type PgQueryAs<T> = QueryAs<'static, Postgres, T, PgArguments>;

pub fn password_hash(password: &[u8]) -> Result<String> {
    Ok(Argon2::default()
        .hash_password(password, &SaltString::generate(&mut OsRng))?
        .to_string())
}
