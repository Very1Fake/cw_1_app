use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres, FromRow};
use uuid::Uuid;

use crate::{
    traits::Insertable,
    types::{contract_status::ContractStatus, metatime::MetaTime},
};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SupplyContract {
    pub uuid: Uuid,
    /// Foreign key references [`Supplier`](super::supplier::Supplier)
    pub supplier: Uuid,
    /// Foreign key references [`Staff`](super::staff::Staff)
    pub manager: Uuid,
    pub status: ContractStatus,
    pub signed: Option<DateTime<Utc>>,
    pub meta: MetaTime,
}

impl SupplyContract {
    pub const NAME: &'static str = "SupplyContract";

    pub const CREATE: &'static str = r#"CREATE TABLE "SupplyContract" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    supplier uuid NOT NULL REFERENCES "Supplier" ON DELETE restrict ON UPDATE cascade,
    manager uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    status "ContractStatus" NOT NULL DEFAULT 'Review',
    signed timestamptz,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "SupplyContract";"#;

    pub const fn new(
        uuid: Uuid,
        supplier: Uuid,
        manager: Uuid,
        status: ContractStatus,
        signed: Option<DateTime<Utc>>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            supplier,
            manager,
            status,
            signed,
            meta,
        }
    }

    pub fn new_auto(
        supplier: Uuid,
        manager: Uuid,
        status: ContractStatus,
        signed: Option<DateTime<Utc>>,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            supplier,
            manager,
            status,
            signed,
            MetaTime::default(),
        )
    }
}

impl Insertable for SupplyContract {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "SupplyContract" (uuid, supplier, manager, status, signed) 
VALUES ($1, $2, $3, $4, $5);"#,
        )
        .bind(self.uuid)
        .bind(self.supplier)
        .bind(self.manager)
        .bind(self.status)
        .bind(self.signed)
    }
}
