use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, types::BigDecimal, Postgres};
use uuid::Uuid;

use crate::{
    traits::Insertable,
    types::{metatime::MetaTime, AccountRole},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Position {
    pub uuid: Uuid,
    pub name: String,
    pub details: Option<String>,
    pub salary: BigDecimal,
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
        salary: BigDecimal,
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
        Self::new(Uuid::new_v4(), name, details, salary, MetaTime::default())
    }
}

impl Insertable for Position {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "Position" (uuid, name, details, salary) VALUES ($1, $2, $3, $4);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.details.clone())
            .bind(self.salary.clone())
    }
}
