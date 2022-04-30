use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres};
use uuid::Uuid;

use crate::traits::Insertable;

/// Represents relation table between [`Warehouse`](`super::warehouse::Warehouse`) and [`Supply`](`super::supply::Supply`)
#[derive(Serialize, Deserialize, Debug)]
pub struct WarehouseSupply {
    /// Foreign key references [`Warehouse`](`super::warehouse::Warehouse`)
    pub item: Uuid,
    /// Foreign key references [`Supply`](`super::supply::Supply`)
    pub supply: Uuid,
    pub amount: i32,
    pub created: DateTime<Utc>,
}

impl WarehouseSupply {
    pub const NAME: &'static str = "WarehouseSupply";

    pub const CREATE: &'static str = r#"CREATE TABLE "WarehouseSupply" (
    item uuid NOT NULL REFERENCES "Warehouse" ON DELETE restrict ON UPDATE cascade,
    supply uuid NOT NULL REFERENCES "Supply" ON DELETE restrict ON UPDATE cascade,
    amount int NOT NULL,
    created timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY(item, supply)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "WarehouseSupply";"#;

    pub const fn new(item: Uuid, supply: Uuid, amount: i32, created: DateTime<Utc>) -> Self {
        Self {
            item,
            supply,
            amount,
            created,
        }
    }
}
impl Insertable for WarehouseSupply {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "WarehouseSupply" VALUES ($1, $2, $3, $4);"#)
            .bind(self.item)
            .bind(self.supply)
            .bind(self.amount)
            .bind(self.created)
    }
}
