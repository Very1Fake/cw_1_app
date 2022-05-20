use serde::{Deserialize, Serialize};
use sqlx::{postgres::{PgArguments, types::PgMoney}, query, query::Query, Postgres, FromRow};
use uuid::Uuid;

use crate::traits::Insertable;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Warehouse`](`super::warehouse::Warehouse`)
#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct OrderWarehouse {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Warehouse`](`super::warehouse::Warehouse`)
    pub item: Uuid,
    pub amount: i32,
    #[serde(
        deserialize_with = "crate::utils::deserialize_pg_money",
        serialize_with = "crate::utils::serialize_pg_money"
    )]
    pub price: PgMoney,
}

impl OrderWarehouse {
    pub const NAME: &'static str = "OrderWarehouse";

    pub const CREATE: &'static str = r#"CREATE TABLE "OrderWarehouse" (
    "order" uuid NOT NULL REFERENCES "Order" ON DELETE cascade ON UPDATE cascade,
    item uuid NOT NULL REFERENCES "Warehouse" ON DELETE restrict ON UPDATE cascade,
    amount int NOT NULL DEFAULT 1,
    price money NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "OrderWarehouse";"#;

    pub fn new(order: Uuid, item: Uuid, amount: i32, price: PgMoney) -> Self {
        Self {
            order,
            item,
            amount,
            price,
        }
    }
}
impl Insertable for OrderWarehouse {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "OrderWarehouse" VALUES ($1, $2, $3, $4);"#)
            .bind(self.order)
            .bind(self.item)
            .bind(self.amount)
            .bind(self.price)
    }
}
