use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{types::PgMoney, PgArguments},
    query,
    query::Query,
    query_as,
    types::BigDecimal,
    FromRow, Postgres,
};
use uuid::Uuid;

use crate::{
    traits::Insertable,
    types::{metatime::MetaTime, AccountRole},
    PgQueryAs,
};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Position {
    pub uuid: Uuid,
    pub name: String,
    pub details: Option<String>,
    #[serde(
        deserialize_with = "crate::utils::deserialize_pg_money",
        serialize_with = "crate::utils::serialize_pg_money"
    )]
    pub salary: PgMoney,
    pub meta: MetaTime,
}

impl Position {
    pub const NAME: &'static str = "Position";

    pub const CREATE: &'static str = r#"CREATE TABLE "Position" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    details text,
    salary money NOT NULL,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"
    DROP TABLE "Position";
    "#;

    /// Sample positions with hints for chances and account roles
    pub const SAMPLES: [(&'static str, i64, u16, AccountRole); 10] = [
        ("Director", 500000, 10, AccountRole::Admin),
        ("Manager", 300000, 20, AccountRole::Manager),
        ("Developer", 250000, 10, AccountRole::Admin),
        ("DB Administrator", 150000, 10, AccountRole::Admin),
        ("Chief HR", 125000, 5, AccountRole::HR),
        ("HR", 100000, 10, AccountRole::HR),
        ("Serviceman", 75000, 30, AccountRole::Serviceman),
        ("Shopman", 50000, 25, AccountRole::Shopman),
        (
            "Chief Warehouse Worker",
            75000,
            5,
            AccountRole::WarehouseWorker,
        ),
        ("Warehouse Worker", 40000, 25, AccountRole::WarehouseWorker),
    ];

    pub const fn new(
        uuid: Uuid,
        name: String,
        details: Option<String>,
        salary: PgMoney,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            name,
            details,
            salary,
            meta,
        }
    }

    pub fn new_auto(name: String, details: Option<String>, salary: BigDecimal) -> Self {
        Self::new(
            Uuid::new_v4(),
            name,
            details,
            PgMoney::from_bigdecimal(salary, 2).unwrap(),
            MetaTime::default(),
        )
    }

    pub fn get_all() -> PgQueryAs<Self> {
        query_as(r#"SELECT * FROM "Position""#)
    }
}

impl Insertable for Position {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "Position" (uuid, name, details, salary) VALUES ($1, $2, $3, $4);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.details.clone())
            .bind(self.salary)
    }
}
