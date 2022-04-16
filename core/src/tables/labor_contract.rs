use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

use crate::types::{contract_status::ContractStatus, metatime::MetaTime};

#[derive(Debug)]
pub struct LaborContract {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub person: Uuid,
    pub passport: String,
    pub status: ContractStatus,
    pub signed: Option<DateTime<Utc>>,
    pub meta: MetaTime,
}

impl LaborContract {
    pub const NAME: &'static str = "LaborContract";

    pub const CREATE: &'static str = r#"CREATE TABLE "LaborContract" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    person uuid NOT NULL REFERENCES "Person" ON DELETE restrict ON UPDATE cascade,
    passport char(10) NOT NULL UNIQUE,
    status "ContractStatus" NOT NULL DEFAULT 'Review',
    signed timestamptz,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "LaborContract";"#;

    pub const fn new(
        uuid: Uuid,
        person: Uuid,
        passport: String,
        status: ContractStatus,
        signed: Option<DateTime<Utc>>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            person,
            passport,
            status,
            signed,
            meta,
        }
    }

    pub fn new_auto(
        person: Uuid,
        passport: String,
        status: ContractStatus,
        signed: Option<DateTime<Utc>>,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            person,
            passport,
            status,
            signed,
            MetaTime::default(),
        )
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "LaborContract" (uuid, person, passport, status, signed) 
VALUES ($1, $2, $3, $4, $5);"#,
        )
        .bind(self.uuid)
        .bind(self.person)
        .bind(self.passport.clone())
        .bind(self.status)
        .bind(self.signed)
        .execute(pool)
        .await
    }
}
