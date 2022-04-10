use sqlx::{postgres::PgQueryResult, query, types::BigDecimal, Error, PgPool};
use uuid::Uuid;

use crate::types::metatime::MetaTime;

#[derive(Debug)]
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

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "Position" (uuid, name, details, salary) VALUES ($1, $2, $3, $4);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.details.clone())
            .bind(self.salary.clone())
            .execute(pool)
            .await
    }
}
