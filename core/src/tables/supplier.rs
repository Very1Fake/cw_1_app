use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, query_as, FromRow, Postgres};
use uuid::Uuid;

use crate::{traits::Insertable, PgQuery, PgQueryAs};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Supplier {
    pub uuid: Uuid,
    pub name: String,
    pub iban: String,
    pub swift: String,
    pub address: String,
    pub country: String,
}

impl Supplier {
    pub const NAME: &'static str = "Supplier";

    pub const CREATE: &'static str = r#"CREATE TABLE "Supplier" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    iban text NOT NULL UNIQUE CHECK (length(iban) <= 32),
    swift text NOT NULL CHECK (length(swift) <= 11),
    address text NOT NULL,
    country char(2) NOT NULL,
    details json,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Supplier";"#;

    pub const fn new(
        uuid: Uuid,
        name: String,
        iban: String,
        swift: String,
        address: String,
        country: String,
    ) -> Self {
        Self {
            uuid,
            name,
            iban,
            swift,
            address,
            country,
        }
    }

    pub fn new_auto(
        name: String,
        iban: String,
        swift: String,
        address: String,
        country: String,
    ) -> Self {
        Self::new(Uuid::new_v4(), name, iban, swift, address, country)
    }

    pub fn delete_by_uuid(uuid: Uuid) -> PgQuery {
        query(r#"DELETE FROM "Supplier" WHERE uuid = $1"#).bind(uuid)
    }

    pub fn get_all() -> PgQueryAs<Self> {
        query_as(r#"SELECT * FROM "Supplier""#)
    }
}

impl Insertable for Supplier {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "Supplier" (uuid, name, iban, swift, address, country) 
VALUES ($1, $2, $3, $4, $5, $6);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.iban.clone())
        .bind(self.swift.clone())
        .bind(self.address.clone())
        .bind(self.country.clone())
    }
}
