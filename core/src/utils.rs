use std::str::FromStr;

use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2,
};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{
    postgres::{types::PgMoney, PgArguments},
    query::{Query, QueryAs},
    Postgres,
};

pub type PgQuery = Query<'static, Postgres, PgArguments>;
pub type PgQueryAs<T> = QueryAs<'static, Postgres, T, PgArguments>;

pub fn password_hash(password: &[u8]) -> Result<String, Error> {
    Ok(Argon2::default()
        .hash_password(password, &SaltString::generate(&mut OsRng))?
        .to_string())
}

pub fn serialize_pg_money<S>(money: &PgMoney, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    money.to_bigdecimal(2).serialize(serializer)
}

pub fn deserialize_pg_money<'de, D>(deserializer: D) -> Result<PgMoney, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(PgMoney::from_bigdecimal(
        BigDecimal::from_str(String::deserialize(deserializer)?.as_str()).unwrap(),
        2,
    )
    .unwrap())
}
