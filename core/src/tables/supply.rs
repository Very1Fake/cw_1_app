use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

use crate::types::{metatime::MetaTime, supply_status::SupplyStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct Supply {
    pub uuid: Uuid,
    /// Foreign key references [`SupplyContract`](`super::supply_contract::SupplyContract`)
    pub contract: Uuid,
    /// Foreign key references [`Staff`](`super::staff::Staff`)
    pub staff: Uuid,
    pub status: SupplyStatus,
    pub signed: Option<DateTime<Utc>>,
    pub meta: MetaTime,
}

impl Supply {
    pub const NAME: &'static str = "Supply";

    pub const CREATE: &'static str = r#"CREATE TABLE "Supply" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract uuid NOT NULL REFERENCES "SupplyContract" ON DELETE restrict ON UPDATE cascade,
    staff uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    status "SupplyStatus" NOT NULL DEFAULT 'Review',
    signed timestamptz,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Supply";"#;

    pub const fn new(
        uuid: Uuid,
        contract: Uuid,
        staff: Uuid,
        status: SupplyStatus,
        signed: Option<DateTime<Utc>>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            contract,
            staff,
            status,
            signed,
            meta,
        }
    }

    pub fn new_auto(
        contract: Uuid,
        staff: Uuid,
        status: SupplyStatus,
        signed: Option<DateTime<Utc>>,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            contract,
            staff,
            status,
            signed,
            MetaTime::default(),
        )
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Supply" (uuid, contract, staff, status, signed)
VALUES ($1, $2, $3, $4, $5);"#,
        )
        .bind(self.uuid)
        .bind(self.contract)
        .bind(self.staff)
        .bind(self.status)
        .bind(self.signed)
        .execute(pool)
        .await
    }
}
